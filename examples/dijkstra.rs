use log::{error, info};
use osmgraphing::{
    configs, helpers,
    io::Parser,
    network::NodeIdx,
    routing::{self},
};
use std::{path::PathBuf, time::Instant};

fn main() {
    helpers::init_logging("INFO", vec!["dijkstra"]).expect("LogLevel 'INFO' does exist.");
    info!("Executing example: A*");

    let map_file = "resources/configs/simple-stuttgart.fmi.yaml";

    // get config by provided map-file
    let (parsing_cfg, routing_cfg) = {
        let raw_cfg = PathBuf::from(map_file);

        let parsing_cfg = match configs::parsing::Config::try_from_yaml(&raw_cfg) {
            Ok(parsing_cfg) => parsing_cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        };

        let routing_cfg = match configs::routing::Config::try_from_yaml(&raw_cfg, &parsing_cfg) {
            Ok(routing_cfg) => routing_cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        };

        (parsing_cfg, routing_cfg)
    };

    // measure parsing-time
    let now = Instant::now();

    // parse and create graph

    let graph = match Parser::parse_and_finalize(parsing_cfg) {
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
}
