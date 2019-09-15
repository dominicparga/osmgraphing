//------------------------------------------------------------------------------------------------//
// other modules

use log::{info, warn};
use osmgraphing::{routing, Parser};
use serde::{Deserialize, Serialize};

//------------------------------------------------------------------------------------------------//
// own modules

pub mod routes;

//------------------------------------------------------------------------------------------------//
// config

pub struct Config<'a> {
    pub mapfile: &'a str,
}

//------------------------------------------------------------------------------------------------//
// simulation

#[derive(Serialize, Deserialize)]
struct EdgeInfo {
    src_id: i64,
    dst_id: i64,
    lat: f64,
    lon: f64,
    is_src: bool,
    is_dst: bool,
    volume: u32,
    usage: u16,
}

pub fn run(cfg: Config) -> Result<(), String> {
    info!("Executing braess-optimization");

    //--------------------------------------------------------------------------------------------//
    // parsing

    let graph = Parser::parse_and_finalize(&cfg.mapfile)?;
    println!("{}", graph);

    //--------------------------------------------------------------------------------------------//
    // read in src-dst-pairs

    let proto_routes = vec![(0, 5), (0, 3), (2, 4)];

    //--------------------------------------------------------------------------------------------//
    // prepare statistics

    let mut usages = vec![0u32; graph.edge_count()];
    let mut is_src = vec![false; graph.edge_count()];
    let mut is_dst = vec![false; graph.edge_count()];
    let mut data: Vec<EdgeInfo> = Vec::with_capacity(1_000);

    //--------------------------------------------------------------------------------------------//
    // routing

    let mut astar = routing::factory::new_fastest_path_astar();

    // routes
    for (src_idx, dst_idx) in proto_routes {
        let src = graph.node(src_idx);
        let dst = graph.node(dst_idx);

        // compute best path
        let option_path = astar.compute_best_path(src.id(), dst.id(), &graph);
        if let Some(path) = option_path {
            info!("Duration {} s from ({}) to ({}).", path.cost(), src, dst);

            // create path to update edge-statistics
            let mut current_idx = src_idx;
            while let Some(edge_dst_idx) = path.succ_node_idx(current_idx) {
                if let Some((_edge, edge_idx)) = graph.edge_from(current_idx, edge_dst_idx) {
                    // update path-edges' usages
                    usages[edge_idx] += 1;
                    // update if edge is src/dst
                    if current_idx == src_idx {
                        is_src[edge_idx] = true;
                    }
                    if edge_dst_idx == dst_idx {
                        is_dst[edge_idx] = true;
                    }
                }
                current_idx = edge_dst_idx;
            }
        } else {
            warn!("No path from ({}) to ({}).", src, dst);
        }
    }

    //--------------------------------------------------------------------------------------------//
    // calculate statistics

    // TODO move loop into loop above since edges are only for interest if they are part of a path
    for edge_idx in 0..graph.edge_count() {
        if usages[edge_idx] == 0 {
            continue;
        }

        let edge = graph.edge(edge_idx);
        let src = graph.node(edge.src_idx());
        let dst = graph.node(edge.dst_idx());
        let lat = (src.lat() + dst.lat()) / 2.0;
        let lon = (src.lon() + dst.lon()) / 2.0;
        let usage = (usages[edge_idx] / 1) as u16; // TODO calculate
        data.push(EdgeInfo {
            src_id: src.id(),
            dst_id: dst.id(),
            lat,
            lon,
            is_src: is_src[edge_idx],
            is_dst: is_dst[edge_idx],
            volume: edge.meters(),
            usage: usage,
        });
    }

    //--------------------------------------------------------------------------------------------//
    // export statistics

    if let Ok(json_data) = serde_json::to_string_pretty(&data) {
        println!("{}", json_data);
    }

    Ok(())
}
