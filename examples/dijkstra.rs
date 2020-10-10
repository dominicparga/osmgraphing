use log::{error, info};
use osmgraphing::{
    configs,
    helpers::{err, init_logging},
    io::network::graph::Parser,
    network::NodeIdx,
    routing::dijkstra::{self, Dijkstra},
};
use std::{path::PathBuf, time::Instant};

fn main() {
    init_logging("INFO", &["dijkstra"]).expect("LogLevel 'INFO' does exist.");
    let result = run();
    if let Err(msg) = result {
        error!("{}", msg);
        panic!("{}", msg);
    }
}

fn run() -> err::Feedback {
    info!("Executing example: Dijkstra");

    let raw_cfg = PathBuf::from("resources/simple_stuttgart/fmi.yaml");

    // parsing

    let parsing_cfg = configs::parsing::Config::try_from_yaml(&raw_cfg)?;
    let now = Instant::now();
    let graph = Parser::parse_and_finalize(parsing_cfg)?;
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    // routing

    let routing_cfg = configs::routing::Config::try_from_yaml(&raw_cfg, graph.cfg())?;
    let mut dijkstra = Dijkstra::new();

    // generate route-pairs

    let nodes = graph.nodes();
    let src = nodes.create(NodeIdx(1));
    let dst = nodes.create(NodeIdx(5));

    let now = Instant::now();
    let option_path = dijkstra.compute_best_path(dijkstra::Query {
        src_idx: src.idx(),
        dst_idx: dst.idx(),
        graph: &graph,
        routing_cfg: &routing_cfg,
    });

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
