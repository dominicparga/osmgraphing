use std::ffi::OsString;

use log::error;

use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//

#[test]
fn simple_stuttgart() {
    let path = OsString::from("resources/maps/simple_stuttgart.fmi");
    let _graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
}

#[test]
fn small() {
    let path = OsString::from("resources/maps/small.fmi");
    let _graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
}
