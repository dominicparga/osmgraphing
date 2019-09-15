mod network;
mod parsing;
mod routing;

//------------------------------------------------------------------------------------------------//

use std::ffi::OsString;

use osmgraphing::network::Graph;
use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//
// helpers

fn parse(path: &str) -> Graph {
    let path = OsString::from(&path);
    match Parser::parse_and_finalize(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!("Could not parse {:?}. ERROR: {}", &path, msg);
        }
    }
}
