use std::ffi::OsString;

use log::error;

use osmgraphing::{Parser, Parsing};

#[test]
fn parsing() {
    let path = OsString::from("resources/osm/isle-of-man_2019-09-05.osm.pbf");
    let _graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

    // TODO check graph structure
}

#[test]
fn parsing_wrong_extension() {
    assert!(
        Parser::parse(&OsString::from("foo.asdf")).is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
