use log::{error, info};
use osmgraphing::{
    configs::Config,
    helpers,
    io::Parser,
    network::NodeIdx,
    routing::{self},
};
use std::{path::PathBuf, time::Instant};

fn main() {
    helpers::init_logging("INFO", vec!["dijkstra"]).expect("LogLevel 'INFO' does exist.");
    info!("Executing example: A*");

    // get config by provided map-file
    let cfg = {
        let cfg_file = PathBuf::from("resources/configs/simple-stuttgart.fmi.yaml");
        match Config::from_yaml(&cfg_file) {
            Ok(cfg) => cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        }
    };
    let cfg_routing = cfg
        .routing
        .expect("Config-file should contain routing-settings.");

    // measure parsing-time
    let now = Instant::now();
    // parse and create graph
    let graph = match Parser::parse_and_finalize(cfg.parsing) {
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

    // init routing to reuse it
    let mut dijkstra = routing::Dijkstra::new();

    // generate route-pairs
    let nodes = graph.nodes();
    let src = nodes.create(NodeIdx(1));
    let dst = nodes.create(NodeIdx(5));

    let now = Instant::now();
    let option_path = dijkstra.compute_best_path(src.idx(), dst.idx(), &graph, &cfg_routing);

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
}
