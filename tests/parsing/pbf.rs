use std::ffi::OsString;

use log::error;

use osmgraphing::Parser;

#[test]
fn isle_of_man() {
    let path = OsString::from("resources/maps/isle-of-man_2019-09-05.osm.pbf");
    let _graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
}
