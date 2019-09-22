//------------------------------------------------------------------------------------------------//
// other modules

use std::path::Path;

use osmgraphing::network::Graph;
use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//
// own modules

mod network;
mod parsing;
mod routing;

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
