//------------------------------------------------------------------------------------------------//
// other modules

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufWriter, Write};

use log::{info, warn};
use osmgraphing::{routing, Parser};
use serde::{Deserialize, Serialize};

//------------------------------------------------------------------------------------------------//
// own modules

pub mod routes;

//------------------------------------------------------------------------------------------------//
// config

pub struct Config<'a> {
    pub map_filepath: &'a str,
    pub out_dirpath: &'a str,
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

    let graph = Parser::parse_and_finalize(&cfg.map_filepath)?;

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

            // reconstruct path to update edge-statistics
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
    // calculate and export statistics

    // open output-file
    let out_filepath = OsString::from([cfg.out_dirpath, "results.json"].join("/"));
    let out_file = match File::create(&out_filepath) {
        Ok(file) => file,
        Err(_) => return Err(format!("Could not open file {:?}", out_filepath)),
    };
    let mut writer = BufWriter::new(out_file);
    let mut write = |s: String| -> Result<(), String> {
        if let Err(io_err) = writer.write(s.as_bytes()) {
            return Err(format!("{}", io_err));
        }
        Ok(())
    };

    // write beginning of json-file
    write(["{", "\"edges\": [", ""].join("\n"))?;

    for _ in 0..16_000 {
        // setup stats before writing to file
        let mut print_with_comma = false;
        for edge_idx in 0..graph.edge_count() {
            if usages[edge_idx] == 0 {
                continue;
            }

            // collect data
            let edge = graph.edge(edge_idx);
            let edge_src = graph.node(edge.src_idx());
            let edge_dst = graph.node(edge.dst_idx());
            let lat = (edge_src.lat() + edge_dst.lat()) / 2.0;
            let lon = (edge_src.lon() + edge_dst.lon()) / 2.0;
            let usage = (usages[edge_idx] / 1) as u16; // TODO calculate
            let edge_info = EdgeInfo {
                src_id: edge_src.id(),
                dst_id: edge_dst.id(),
                lat,
                lon,
                is_src: is_src[edge_idx],
                is_dst: is_dst[edge_idx],
                volume: edge.meters(),
                usage: usage,
            };

            // write to file
            if let Ok(mut json_data) = serde_json::to_string_pretty(&edge_info) {
                if print_with_comma {
                    json_data = format!(",{}", json_data);
                } else {
                    print_with_comma = true;
                }
                write(json_data)?;
            }
        }
    }

    // write ending of json-file
    write(["", "]", "}"].join("\n"))?;

    Ok(())
}
