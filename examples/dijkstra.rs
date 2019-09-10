use std::ffi::OsString;
use std::time::Instant;

use log::{error, info};

use osmgraphing::{routing, Parser};

fn init_logging(verbosely: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if verbose logging: log `info` for the server and this repo
    if verbosely {
        builder.filter(Some("dijkstra"), log::LevelFilter::Info);
        builder.filter(Some("osmgraphing"), log::LevelFilter::Info);
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
    info!("Executing example: dijkstra");

    //----------------------------------------------------------------------------------------------
    // parsing

    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/osm/small.fmi"),
    };

    let now = Instant::now();
    let graph = match Parser::parse(&path) {
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

    //----------------------------------------------------------------------------------------------
    // dijkstra

    // routing
    let mut dijkstra = routing::Dijkstra::new(&graph);
    let src_idx = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();
    // let dsts: Vec<usize> = vec![80]; problem on baden-wuerttemberg.osm.pbf

    let src = graph.node(src_idx);

    for dst_idx in dsts {
        let dst = graph.node(dst_idx);

        info!("");

        let now = Instant::now();
        let path = dijkstra.compute_shortest_path(src_idx, dst_idx);
        info!(
            "Ran Dijkstra in {} µs a.k.a {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        info!(
            "Distance {} m from ({}) to ({}).",
            path.cost[dst_idx], src, dst
        );
    }
}
