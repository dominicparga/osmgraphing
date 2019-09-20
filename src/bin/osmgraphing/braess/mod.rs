//------------------------------------------------------------------------------------------------//
// other modules

use std::path;

use log::{info, warn};
use osmgraphing::network::Graph;
use osmgraphing::{routing, Parser};

//------------------------------------------------------------------------------------------------//
// own modules

mod io_kyle;
mod model;
pub mod routes;
use model::EdgeInfo;

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
    let out_dir_path = io_kyle::create_datetime_dir(cfg.paths.output.dirs.results)?;
    let out_file_path = out_dir_path.join("edge_stats.csv");
    io_kyle::create_file(&out_file_path)?;
    let proto_routes = io_kyle::read_proto_routes(cfg.paths.input.files.proto_routes)?;

    let graph = Parser::parse_and_finalize(&cfg.paths.input.files.map)?;

    //--------------------------------------------------------------------------------------------//
    // prepare statistics

    let mut data: Vec<Option<EdgeInfo>> = vec![None; graph.edge_count()];

    //--------------------------------------------------------------------------------------------//
    // routing and statistics-update

    let mut astar = routing::factory::new_fastest_path_astar();

    // routes
    for (src_id, dst_id) in proto_routes {
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
            info!(
                "Duration {} s from ({}) to ({}).",
                best_path.cost() / 1_000,
                src,
                dst
            );

            update_edge_info(&mut data, &best_path, &graph);
        } else {
            warn!("No best path from ({}) to ({}).", src, dst);
        }
    }

    //--------------------------------------------------------------------------------------------//
    // export statistics

    io_kyle::export_statistics(data, &out_file_path)?;

    Ok(())
}

fn update_edge_info(
    data: &mut Vec<Option<EdgeInfo>>,
    best_path: &routing::astar::Path,
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

        // create EdgeInfo if not existing
        {
            let (edge_src, edge_dst) = (graph.node(edge.src_idx()), graph.node(edge.dst_idx()));

            if data[edge_idx].is_none() {
                data[edge_idx] = Some(EdgeInfo {
                    src_id: edge_src.id(),
                    dst_id: edge_dst.id(),
                    decimicro_lat: {
                        (edge_src.coord().decimicro_lat() + edge_dst.coord().decimicro_lat()) / 2
                    },
                    decimicro_lon: {
                        (edge_src.coord().decimicro_lon() + edge_dst.coord().decimicro_lon()) / 2
                    },
                    is_src: false,
                    is_dst: false,
                    lane_count: edge.lane_count(),
                    length_m: edge.meters(),
                    route_count: 0,
                });
            }
        }

        // update edge-info of path-edges
        {
            let edge_info = data[edge_idx]
                // Option<EdgeInfo> -> already set above
                .as_mut()
                .expect("EdgeInfo should have been set a few lines above.");
            // update path-edges' usages
            edge_info.route_count += 1;
            // update if edge is src/dst
            if edge.src_idx() == best_path.src_idx() {
                edge_info.is_src = true;
            }
            if edge.dst_idx() == best_path.dst_idx() {
                edge_info.is_dst = true;
            }
        }

        current_idx = edge_dst_idx;
    }
}
