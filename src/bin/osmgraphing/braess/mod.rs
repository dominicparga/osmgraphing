//------------------------------------------------------------------------------------------------//
// other modules

use std::cmp::min;
use std::path;
use std::sync::Arc;

use log::{debug, info, trace, warn};
use osmgraphing::network::{Graph, Node};
use osmgraphing::{routing, Parser};
use progressing::Bar;

//------------------------------------------------------------------------------------------------//
// own modules

mod io_kyle;
mod model;
use model::{EdgeInfo, SmallEdgeInfo};
mod multithreading;
use multithreading::WorkerSocket;
pub mod routes;

//------------------------------------------------------------------------------------------------//
// config

pub mod config {
    use std::path;

    pub struct Config<'a, P: AsRef<path::Path> + ?Sized> {
        pub paths: Paths<'a, P>,
        pub thread_count: u8,
        pub params: SimParams,
    }
    pub struct SimParams {
        pub route_count: Option<usize>,
        pub loop_count: Option<usize>,
        pub removed_edges_per_loop: Option<usize>,
    }
    impl SimParams {
        pub fn loop_count(&self) -> usize {
            self.loop_count.unwrap_or(2)
        }
        pub fn removed_edges_per_loop(&self) -> usize {
            self.removed_edges_per_loop.unwrap_or(10)
        }
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

    // optimization-loops
    let loop_count = 2;

    // check path of io-files before expensive simulation
    let out_dir_paths = {
        let tmp = io_kyle::create_datetime_dir(cfg.paths.output.dirs.results)?;
        let mut out_dir_paths = vec![];
        for i in 0..loop_count {
            let out_dir_path = tmp.join(format!("loop_{}", i));
            io_kyle::create_dir(&out_dir_path)?;
            out_dir_paths.push(out_dir_path);
        }
        out_dir_paths
    };
    let out_file_paths = {
        let mut out_file_paths = vec![];
        for out_dir_path in out_dir_paths {
            let out_file_path = out_dir_path.join("edge_stats.csv");
            io_kyle::create_file(&out_file_path)?;
            out_file_paths.push(out_file_path);
        }
        out_file_paths
    };

    // proto_routes
    let mut proto_routes_backup = io_kyle::read_proto_routes(cfg.paths.input.files.proto_routes)?;
    {
        // route-count
        let route_count = cfg.params.route_count.unwrap_or(proto_routes_backup.len());
        if route_count > proto_routes_backup.len() {
            warn!(
                "{} routes should be used, but taking {} src-dst-pairs from provided file.",
                route_count,
                proto_routes_backup.len()
            );
        } else {
            proto_routes_backup.truncate(route_count);
        }
    }
    // reverse since split_off returns last part
    proto_routes_backup.reverse();
    info!("Using {} proto-routes", proto_routes_backup.len());
    info!(
        "Running {} optimization-loop-runs, removing {} edges each",
        cfg.params.loop_count(),
        cfg.params.removed_edges_per_loop()
    );

    // graph
    let graph = Parser::parse_and_finalize(&cfg.paths.input.files.map)?;
    let mut graph = Arc::new(graph);

    // routing
    let mut astar = routing::factory::new_fastest_path_astar();

    //--------------------------------------------------------------------------------------------//
    // routing and statistics-update

    for loop_i in 0..loop_count {
        //----------------------------------------------------------------------------------------//
        // prepare loop

        // routes
        let mut proto_routes = proto_routes_backup.clone();

        // logging
        let mut progressbar =
            progressing::BernoulliBar::from_goal(proto_routes_backup.len() as u32);

        // stats
        let mut stats: Vec<Option<SmallEdgeInfo>> = vec![None; graph.edge_count()];

        // multithreading
        {
            if cfg.thread_count == 0 {
                return Err("Number of threads should be > 0".to_owned());
            }
            info!("Using {} threads", cfg.thread_count);
        }
        let (mut workers, stats_rx) = WorkerSocket::spawn_some(cfg.thread_count, &graph)?;
        let workpkg_route_count = 10;

        //----------------------------------------------------------------------------------------//
        // run loop

        // End of loop
        //
        // threads send data before this main-thread/master-thread sends
        // -> recv will eventually be triggered
        // -> if no work is available, drop respective sender
        // -> as far as all senders are dropped, break loop

        info!(
            "Starting braess-optimization-loop {}/{} ..",
            loop_i + 1,
            loop_count
        );
        // process incoming stats
        info!("Intermediate simulation-progress is stored (silently) to the final csv-file.");
        progressbar.reprint()?;
        let mut last_progress = progressbar.progress();
        loop {
            if let Ok(packet) = stats_rx.recv() {
                // merge received stats and update progress
                merge_into(&packet.stats, &mut stats);
                // update progress and export intermediate results to file
                progressbar.add(packet.progress).reprint()?;
                if progressbar.progress().successes - last_progress.successes >= 100 {
                    // remember state for next comparison
                    last_progress = progressbar.progress();
                    debug!(
                        "\nExporting stats after {} processed routes",
                        progressbar.progress().successes
                    );
                    let appending = false;
                    let data = data_from_stats(&stats, &graph);
                    io_kyle::write_edge_stats(&data, &out_file_paths[loop_i], appending)?;
                    debug!("Exported");
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
                        None => {
                            progressbar.reprintln()?;
                            return Err("Worker's sender should not be released yet".to_owned());
                        }
                    };
                    if let Err(e) = worker.data_tx().send(workpkg) {
                        progressbar.reprintln()?;
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
        progressbar.reprintln()?;
        info!("Finished braess-optimization-loop");

        //----------------------------------------------------------------------------------------//
        // export statistics

        info!("Exporting stats to {}", out_file_paths[loop_i].display());

        let appending = false;
        let data = data_from_stats(&stats, &graph);
        io_kyle::write_edge_stats(&data, &out_file_paths[loop_i], appending)?;

        //----------------------------------------------------------------------------------------//
        // optimize graph

        optimize_graph(
            Arc::get_mut(&mut graph).expect("No other thread should hold some Arc(graph)"),
            cfg.params.removed_edges_per_loop(),
            &mut astar,
            data,
        )?;
    }

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

fn optimize_graph(
    graph: &mut Graph,
    mut disable_count: usize,
    astar: &mut Box<dyn routing::Astar>,
    mut data: Vec<EdgeInfo>,
) -> Result<(), String> {
    //--------------------------------------------------------------------------------------------//

    // sort in descending order
    data.sort_by(|a, b| {
        b.edge_utilization_thousandth()
            .cmp(&a.edge_utilization_thousandth())
    });

    // iterate over sorted "edges" and disable the worst ones
    for edge_info in data {
        if disable_count == 0 {
            break;
        }

        let (src, dst) = src_dst_from(edge_info.src_id, edge_info.dst_id, graph)?;
        let (_edge, edge_idx) = match graph.edge_from(src.idx(), dst.idx()) {
            Some((e, i)) => (e, i),
            None => {
                return Err(format!(
                    "Graph should contain this edge ({}->{}).",
                    src.idx(),
                    dst.idx()
                ))
            }
        };

        // disable edge
        graph.disable_edge(edge_idx);
        // get new references due to immut-mut-immut-usage
        let (src, dst) = src_dst_from(edge_info.src_id, edge_info.dst_id, graph)?;
        // check if src and dst are still connected
        if astar.compute_best_path(src, dst, graph).is_some() {
            disable_count -= 1;
        } else {
            graph.enable_edge(edge_idx);
        }
    }

    if disable_count > 0 {
        warn!(
            "Did not disable {} edges because there are too less edges used.",
            disable_count
        );
    }
    Ok(())
}

fn data_from_stats(stats: &Vec<Option<SmallEdgeInfo>>, graph: &Graph) -> Vec<EdgeInfo> {
    // remove all None-values
    // and parse data into output-format
    stats
        .into_iter()
        .filter_map(|s| match s {
            Some(small_edge_info) => Some(EdgeInfo::from(&small_edge_info, &graph)),
            None => None,
        })
        .collect()
}

//------------------------------------------------------------------------------------------------//

/// return (successes, attempts)
fn work_off_all(
    proto_routes: &[(i64, i64)],
    astar: &mut Box<dyn routing::Astar>,
    stats: &mut Vec<Option<SmallEdgeInfo>>,
    graph: &Graph,
) -> Result<progressing::BernoulliProgress, String> {
    // progress
    let mut progress = progressing::BernoulliProgress::new();

    // loop over all routes, calculate best path, evaluate and update stats
    for proto_route in proto_routes {
        let new_progress = work_off(proto_route, astar, stats, graph)?;
        progress += new_progress;
    }

    Ok(progress)
}
/// return (successes, attempts)
fn work_off(
    proto_route: &(i64, i64),
    astar: &mut Box<dyn routing::Astar>,
    stats: &mut Vec<Option<SmallEdgeInfo>>,
    graph: &Graph,
) -> Result<progressing::BernoulliProgress, String> {
    // loop over all routes, calculate best path, evaluate and update stats
    let &(src_id, dst_id) = proto_route;

    let (src, dst) = src_dst_from(src_id, dst_id, graph)?;

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

        Ok((1, 1).into())
    } else {
        warn!("No path from ({}) to ({}).", src, dst);

        Ok((0, 1).into())
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

//------------------------------------------------------------------------------------------------//

fn src_dst_from(src_id: i64, dst_id: i64, graph: &Graph) -> Result<(&Node, &Node), String> {
    // get nodes: src and dst
    let src = graph.node(match graph.node_idx_from(src_id) {
        Ok(src_idx) => src_idx,
        Err(_) => {
            return Err(format!(
                "Src-id {} from src-dst-pair ({}, {}) could not be found in the graph.",
                src_id, src_id, dst_id,
            ))
        }
    });
    let dst = graph.node(match graph.node_idx_from(dst_id) {
        Ok(dst_idx) => dst_idx,
        Err(_) => {
            return Err(format!(
                "Dst-id {} from src-dst-pair ({}, {}) could not be found in the graph.",
                dst_id, src_id, dst_id,
            ))
        }
    });

    let src = match src {
        Some(node) => node,
        None => {
            return Err(format!(
                "Src-id {} from src-dst-pair ({}, {}) could not be found in the graph.",
                src_id, src_id, dst_id,
            ))
        }
    };
    let dst = match dst {
        Some(dst_idx) => dst_idx,
        None => {
            return Err(format!(
                "Dst-id {} from src-dst-pair ({}, {}) could not be found in the graph.",
                dst_id, src_id, dst_id,
            ))
        }
    };

    Ok((src, dst))
}
