use log::{error, info};
use osmgraphing::{
    configs,
    helpers::{err, init_logging},
    io::network::graph::Parser,
    routing::{
        dijkstra::{self, Dijkstra},
        exploration::ConvexHullExplorator,
    },
};
use std::{path::PathBuf, time::Instant};

fn main() {
    init_logging("INFO", &["exploration"]).expect("LogLevel 'INFO' does exist.");
    let result = run();
    if let Err(msg) = result {
        error!("{}", msg);
        panic!("{}", msg);
    }
}

fn run() -> err::Feedback {
    info!("Executing example: Exploration");

    let raw_cfg = PathBuf::from("resources/isle_of_man_2020-03-14/fmi.yaml");

    // parsing

    let parsing_cfg = match configs::parsing::Config::try_from_yaml(&raw_cfg) {
        Ok(parsing_cfg) => parsing_cfg,
        Err(msg) => return Err(err::Msg::from(format!("{}", msg))),
    };

    // measure parsing-time
    let now = Instant::now();

    // parse and create graph

    let graph = match Parser::parse_and_finalize(parsing_cfg) {
        Ok(graph) => graph,
        Err(msg) => return Err(format!("{}", msg).into()),
    };
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    // routing

    let routing_cfg = match configs::routing::Config::try_from_yaml(&raw_cfg, graph.cfg()) {
        Ok(routing_cfg) => routing_cfg,
        Err(msg) => return Err(format!("{}", msg).into()),
    };
    let mut dijkstra = Dijkstra::new();
    let mut explorator = ConvexHullExplorator::new();

    // generate route-pairs

    let nodes = graph.nodes();

    let src = nodes
        .create_from(283477868)
        .expect("Src-node should exist.");
    let dst = nodes
        .create_from(283477875)
        .expect("Dst-node should exist.");

    let now = Instant::now();
    let found_paths = explorator.fully_explorate(
        dijkstra::Query {
            src_idx: src.idx(),
            dst_idx: dst.idx(),
            graph: &graph,
            routing_cfg: &routing_cfg,
        },
        &mut dijkstra,
    );

    info!("");
    info!(
        "Ran Exploration-query in {} ms",
        now.elapsed().as_micros() as f64 / 1_000.0,
    );
    if found_paths.is_empty() {
        info!("No path found from ({}) to ({}).", src, dst);
    } else {
        info!("Found {} path(s):", found_paths.len());
        found_paths.iter().for_each(|path| info!("  {}", path))
    }

    Ok(())
}
