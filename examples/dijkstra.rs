use log::info;
use osmgraphing::{
    configs, helpers,
    io::network::Parser,
    network::NodeIdx,
    routing::{self},
};
use std::{path::PathBuf, time::Instant};

fn main() -> Result<(), String> {
    helpers::init_logging("INFO", vec!["dijkstra"]).expect("LogLevel 'INFO' does exist.");
    info!("Executing example: A*");

    let raw_cfg = PathBuf::from("resources/configs/simple-stuttgart.fmi.yaml");

    // parsing

    let parsing_cfg = match configs::parsing::Config::try_from_yaml(&raw_cfg) {
        Ok(parsing_cfg) => parsing_cfg,
        Err(msg) => return Err(format!("{}", msg)),
    };

    // measure parsing-time
    let now = Instant::now();

    // parse and create graph

    let graph = match Parser::parse_and_finalize(parsing_cfg) {
        Ok(graph) => graph,
        Err(msg) => return Err(format!("{}", msg)),
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
        Err(msg) => return Err(format!("{}", msg)),
    };
    let mut dijkstra = routing::Dijkstra::new();

    // generate route-pairs

    let nodes = graph.nodes();
    let src = nodes.create(NodeIdx(1));
    let dst = nodes.create(NodeIdx(5));

    let now = Instant::now();
    let option_path = dijkstra.compute_best_path(src.idx(), dst.idx(), &graph, &routing_cfg);

    info!("");
    info!(
        "Ran Dijkstra-query in {} ms",
        now.elapsed().as_micros() as f64 / 1_000.0,
    );
    if let Some(path) = option_path {
        info!(
            "Path costs {:?} from ({}) to ({}).",
            path.flatten(&graph).costs(),
            src,
            dst
        );
    } else {
        info!("No path from ({}) to ({}).", src, dst);
    }

    Ok(())
}
