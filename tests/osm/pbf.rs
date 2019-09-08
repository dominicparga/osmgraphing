use std::ffi::OsString;

use osmgraphing::{Parser, Parsing};

#[test]
#[ignore]
fn parsing() {
    let path = OsString::from("resources/osm/small.pbf"); // file missing
    let _graph = Parser::parse(&path);

    // check graph structure
    unimplemented!()
}

#[test]
fn parsing_wrong_extension() {
    assert!(
        Parser::parse(&OsString::from("foo.asdf")).is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
