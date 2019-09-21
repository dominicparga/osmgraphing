//------------------------------------------------------------------------------------------------//
// other modules

use std::path;
use std::sync::Arc;

use log::{info, trace, warn};
use osmgraphing::network::Graph;
use osmgraphing::{routing, Parser};

//------------------------------------------------------------------------------------------------//
// own modules

mod io_kyle;
mod model;
use model::{EdgeInfo, SmallEdgeInfo};
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
    info!("Executing braess-optimization");

    //--------------------------------------------------------------------------------------------//
    // prepare simulation

    // check path of io-files before expensive simulation
    let out_dir_path = {
        let out_dir_path = io_kyle::create_datetime_dir(cfg.paths.output.dirs.results)?;
        let out_dir_path = out_dir_path.join("loop_0");
        io_kyle::create_dir(&out_dir_path)?
    };
    let out_file_path = out_dir_path.join("edge_stats.csv");
    io_kyle::create_file(&out_file_path)?;

    // proto_routes
    let mut proto_routes = io_kyle::read_proto_routes(cfg.paths.input.files.proto_routes)?;

    // graph
    let graph = Parser::parse_and_finalize(&cfg.paths.input.files.map)?;
    let graph = Arc::new(graph);

    //--------------------------------------------------------------------------------------------//
    // prepare statistics

    // logging
    let mut progress_bar = progressing::Bar::from(proto_routes.len() as u32);

    // stats
    let mut stats: Vec<Option<SmallEdgeInfo>> = vec![None; graph.edge_count()];

    // routing
    let mut astar = routing::factory::new_fastest_path_astar();

    // multithreading
    // let (workers, stats_rx) = WorkerSocket::spawn_some(8, &graph);

    //--------------------------------------------------------------------------------------------//
    // routing and statistics-update

    progress_bar.log();
    while proto_routes.len() > 0 {
        // let workpkg = proto_routes.split_off(1);
        let proto_route = proto_routes.pop().unwrap();
        let (k, n) = work_off(&proto_route, &mut astar, &mut stats, &graph)?;

        progress_bar.update_n(n).update_k(k).try_log();
    }
    // TODO update data
    // TODO ask workers for results
    // TODO update data
    // TODO update progress_bar and log

    //--------------------------------------------------------------------------------------------//
    // export statistics

    let data: Vec<EdgeInfo> = stats
        .drain(..)
        .filter_map(|s| match s {
            Some(small_edge_info) => Some(EdgeInfo::from(small_edge_info, &graph)),
            None => None,
        })
        .collect();
    let appending = false;
    io_kyle::write_edge_stats(&data, &out_file_path, appending)?;

    Ok(())
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
