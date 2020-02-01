//------------------------------------------------------------------------------------------------//
// own modules

mod network;
mod parsing;
mod routing;

//------------------------------------------------------------------------------------------------//
// other modules

use osmgraphing::{network::Graph, Parser};
use std::path::Path;

//------------------------------------------------------------------------------------------------//
// helpers

fn parse<P: AsRef<Path> + ?Sized>(path: &P) -> Graph {
    let path = path.as_ref();
    match Parser::parse_and_finalize(path) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!("Could not parse {}. ERROR: {}", path.display(), msg);
        }
    }
}
