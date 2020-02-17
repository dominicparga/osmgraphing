mod network;
mod parsing;
mod routing;

use osmgraphing::{configs::graph, network::Graph, Parser};

fn parse(cfg: &graph::Config) -> Graph {
    match Parser::parse_and_finalize(cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!(
                "Could not parse {}. ERROR: {}",
                cfg.paths().map_file().display(),
                msg
            );
        }
    }
}
