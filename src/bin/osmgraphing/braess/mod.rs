//------------------------------------------------------------------------------------------------//
// other modules

use std::cmp::min;
use std::path;
use std::sync::Arc;

use log::{info, trace, warn};
use osmgraphing::network::Graph;
use osmgraphing::{routing, Parser};

//------------------------------------------------------------------------------------------------//
// own modules

mod io_kyle;
mod model;
use model::SmallEdgeInfo;
mod multithreading;
use multithreading::WorkerSocket;
mod progressing;
pub mod routes;

//------------------------------------------------------------------------------------------------//
// config

pub mod config {
    use std::path;

    pub struct Config<'a, P: AsRef<path::Path> + ?Sized> {
        pub paths: Paths<'a, P>,
    }
    pub struct Paths<'a, P: AsRef<path::Path> + ?Sized> {
        pub input: InputPaths<'a, P>,
        pub output: OutputPaths<'a, P>,
    }

    //--------------------------------------------------------------------------------------------//
    // input-paths

    pub struct InputPaths<'a, P: AsRef<path::Path> + ?Sized> {
        pub files: InputFiles<'a, P>,
    }
    pub struct InputFiles<'a, P: AsRef<path::Path> + ?Sized> {
        pub map: &'a P,
        pub proto_routes: &'a P,
    }

    //--------------------------------------------------------------------------------------------//
    // output-paths

    pub struct OutputPaths<'a, P: AsRef<path::Path> + ?Sized> {
        pub dirs: OutputDirs<'a, P>,
    }
    pub struct OutputDirs<'a, P: AsRef<path::Path> + ?Sized> {
        pub results: &'a P,
    }
}
pub use config as cfg;
use config::Config;

//------------------------------------------------------------------------------------------------//
// simulation

pub fn run<P: AsRef<path::Path> + ?Sized>(cfg: Config<P>) -> Result<(), String> {
    info!("Executing braess-simulation ..");

    //--------------------------------------------------------------------------------------------//
    // prepare simulation
    info!("Preparing simulation ..");

    // check path of io-files before expensive simulation
    let out_dir_path = {
        let out_dir_path = io_kyle::create_datetime_dir(cfg.paths.output.dirs.results)?;
        let out_dir_path = out_dir_path.join("loop_0");
        io_kyle::create_dir(&out_dir_path)?
    };
    let out_file_path = {
        let out_file_path = out_dir_path.join("edge_stats.csv");
        io_kyle::create_file(&out_file_path)?;
        out_file_path
    };

    // proto_routes
    let mut proto_routes = io_kyle::read_proto_routes(cfg.paths.input.files.proto_routes)?;
    // reverse since split_off returns last part
    proto_routes.reverse();

    // graph
    let graph = Parser::parse_and_finalize(&cfg.paths.input.files.map)?;
    let graph = Arc::new(graph);

    //--------------------------------------------------------------------------------------------//
    // prepare statistics

    // logging
    let mut progress_bar = progressing::Bar::from(proto_routes.len() as u32);

    // stats
    let mut stats: Vec<Option<SmallEdgeInfo>> = vec![None; graph.edge_count()];

    // multithreading
    let thread_count = 8;
    let (mut workers, stats_rx) = WorkerSocket::spawn_some(thread_count, &graph)?;
    let workpkg_route_count = 10;

    //--------------------------------------------------------------------------------------------//
    // routing and statistics-update

    // End of loop
    //
    // threads send data before this main-thread/master-thread sends
    // -> recv will eventually be triggered
    // -> if no work is available, drop respective sender
    // -> as far as all senders are dropped, break loop

    info!("Starting braess-optimization-loop ..");
    // process incoming stats
    progress_bar.log();
    loop {
        if let Ok(packet) = stats_rx.recv() {
            // merge received stats and update progress
            merge_into(&packet.stats, &mut stats);
            // update progress and export intermediate results to file
            let result = progress_bar.update_n(packet.n).update_k(packet.k).try_log();
            if result.is_ok() {
                info!(
                    "Exporting stats after {} processed routes",
                    progress_bar.k()
                );
                let appending = false;
                io_kyle::write_edge_stats(&stats, &out_file_path, appending, &graph)?;
                info!("Exported");
            }
            // and send new data if available
            if proto_routes.len() > 0 {
                let workpkg = {
                    let len = proto_routes.len();
                    let work_count = min(workpkg_route_count, len);
                    proto_routes.split_off(len - work_count)
                };
                // send workpkg
                let worker = match workers[packet.worker_idx as usize].as_ref() {
                    Some(worker) => worker,
                    None => return Err("Worker's sender should not be released yet".to_owned()),
                };
                if let Err(e) = worker.data_tx().send(workpkg) {
                    return Err(format!("Sending stucks due to {}", e));
                }
            } else {
                // drop sender
                if let Some(worker) = workers[packet.worker_idx as usize].take() {
                    worker.drop_and_join()?;
                }
            }
        } else {
            // -> disconnected
            break;
        }
    }
    info!("Finished braess-optimization-loop");

    //--------------------------------------------------------------------------------------------//
    // export statistics
    info!("Exporting stats to {}", out_file_path.display());

    let appending = false;
    io_kyle::write_edge_stats(&stats, &out_file_path, appending, &graph)?;

    info!("Finished braess-simulation");
    Ok(())
}

//------------------------------------------------------------------------------------------------//

fn merge_into(packet_stats: &Vec<Option<SmallEdgeInfo>>, stats: &mut Vec<Option<SmallEdgeInfo>>) {
    for opt_small_edge_info in packet_stats {
        if let Some(small_edge_info) = opt_small_edge_info {
            let edge_idx = small_edge_info.edge_idx;

            // update edge-info or, if not existing, add new one
            // stats[idx] guaranteed due to stats-init
            if let Some(edge_info) = stats
                .get_mut(edge_idx)
                .expect("Should be set after initializing stats.")
            {
                edge_info.update(small_edge_info);
            } else {
                stats[edge_idx] = Some(small_edge_info.clone());
            }
        }
    }
}

//------------------------------------------------------------------------------------------------//

/// return (k, n)
fn work_off_all(
    proto_routes: &[(i64, i64)],
    astar: &mut Box<dyn routing::Astar>,
    stats: &mut Vec<Option<SmallEdgeInfo>>,
    graph: &Graph,
) -> Result<(u32, u32), String> {
    // progress
    let mut k = 0;
    let mut n = 0;

    // loop over all routes, calculate best path, evaluate and update stats
    for proto_route in proto_routes {
        let (delta_k, delta_n) = work_off(proto_route, astar, stats, graph)?;
        k += delta_k;
        n += delta_n;
    }

    Ok((k, n))
}
/// return (k, n)
fn work_off(
    proto_route: &(i64, i64),
    astar: &mut Box<dyn routing::Astar>,
    stats: &mut Vec<Option<SmallEdgeInfo>>,
    graph: &Graph,
) -> Result<(u32, u32), String> {
    // loop over all routes, calculate best path, evaluate and update stats
    let &(src_id, dst_id) = proto_route;

    // get nodes: src and dst
    let src = graph.node(match graph.node_idx_from(src_id) {
        Ok(src_idx) => src_idx,
        Err(_) => {
            return Err(format!(
                "Src-id {} from proto-route ({}, {}) could not be found in the graph.",
                src_id, src_id, dst_id,
            ))
        }
    });
    let dst = graph.node(match graph.node_idx_from(dst_id) {
        Ok(dst_idx) => dst_idx,
        Err(_) => {
            return Err(format!(
                "Dst-id {} from proto-route ({}, {}) could not be found in the graph.",
                dst_id, src_id, dst_id,
            ))
        }
    });

    // compute best path
    let option_best_path = astar.compute_best_path(src, dst, &graph);
    if let Some(best_path) = option_best_path {
        trace!(
            "Duration {} s from ({}) to ({}).",
            best_path.cost() / 1_000,
            src,
            dst
        );

        // update stats
        evaluate_best_path(&best_path, stats, &graph);

        Ok((1, 1))
    } else {
        warn!("No path from ({}) to ({}).", src, dst);

        Ok((0, 1))
    }
}

fn evaluate_best_path(
    best_path: &routing::astar::Path,
    stats: &mut Vec<Option<SmallEdgeInfo>>,
    graph: &Graph,
) {
    // reconstruct best path to update edge-statistics
    let mut current_idx = best_path.src_idx();
    while let Some(edge_dst_idx) = best_path.succ_node_idx(current_idx) {
        // get edge from its nodes
        let (edge, edge_idx) = graph
            .edge_from(current_idx, edge_dst_idx)
            .expect("Path should only use edges from the graph.");
        debug_assert_eq!(edge.src_idx(), current_idx, "edge.src_idx() != current_idx");
        debug_assert_eq!(
            edge.dst_idx(),
            edge_dst_idx,
            "edge.dst_idx() != edge_dst_idx"
        );

        // update edge-info or, if not existing, add new one
        // stats[idx] guaranteed due to stats-init
        if let Some(edge_info) = stats
            .get_mut(edge_idx)
            .expect("Should be set after initializing stats.")
        {
            edge_info.is_src |= edge.src_idx() == best_path.src_idx();
            edge_info.is_dst |= edge.dst_idx() == best_path.dst_idx();
            edge_info.route_count += 1;
        } else {
            stats[edge_idx] = Some(SmallEdgeInfo {
                edge_idx,
                is_src: edge.src_idx() == best_path.src_idx(),
                is_dst: edge.dst_idx() == best_path.dst_idx(),
                route_count: 1,
            });
        }

        // next loop run
        current_idx = edge_dst_idx;
    }
}
