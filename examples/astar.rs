use std::ffi::OsString;
use std::time::Instant;

use log::{error, info};

use osmgraphing::{routing, Parser};

//------------------------------------------------------------------------------------------------//
// points in Germany

// somewhere in Stuttgart (Schwabstrasse)
// id 20_443_604 osm-id 2_933_335_353 lat 48.77017570000000291 lon 9.15657690000000102

// "near" Esslingen
// id:4_647 osm-id:163_354 lat:48.66743380000000485 lon:9.24459110000000095

// somewhere in Ulm
// id 9_058_109 osm-id 580_012_224 lat 48.39352330000000535 lon 9.9816315000000006

// near Aalen
// id 54_288 osm-id 2_237_652 lat 48.88542720000000230 lon 10.13642900000000147

// somewhere in Berlin
// id 296_679 osm-id 26_765_334 lat 52.50536590000000103 lon 13.38662390000000002

//------------------------------------------------------------------------------------------------//

fn init_logging(verbosely: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if verbose logging: log `info` for the server and this repo
    if verbosely {
        builder.filter(Some(env!("CARGO_PKG_NAME")), log::LevelFilter::Info);
        builder.filter(Some("astar"), log::LevelFilter::Info);
    }
    // overwrite default with environment-variables
    if let Ok(filters) = std::env::var("RUST_LOG") {
        builder.parse_filters(&filters);
    }
    if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&write_style);
    }
    // init
    builder.init();
}

fn main() {
    init_logging(true);
    info!("Executing example: A*");

    //--------------------------------------------------------------------------------------------//
    // parsing

    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/maps/simple_stuttgart.fmi"),
    };

    let now = Instant::now();
    let graph = match Parser::parse_and_finalize(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    //--------------------------------------------------------------------------------------------//
    // astar

    let mut astar = routing::factory::new_shortest_path_astar();

    let src_idx = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();
    // let dsts: Vec<usize> = vec![80]; problem on baden-wuerttemberg.osm.pbf

    let src = graph
        .node(src_idx)
        .expect("src-node of idx={} should be in graph");

    for dst_idx in dsts {
        let dst = graph
            .node(dst_idx)
            .expect("dst-node of idx={} should be in graph");

        info!("");

        let now = Instant::now();
        let option_path = astar.compute_best_path(src, dst, &graph);
        info!(
            "Ran A* in {} µs a.k.a {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        if let Some(path) = option_path {
            info!("Distance {} m from ({}) to ({}).", path.cost(), src, dst);
        } else {
            info!("No path from ({}) to ({}).", src, dst);
        }
    }
}
