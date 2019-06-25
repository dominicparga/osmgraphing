use osmgraphing::osm;
use std::ffi::OsString;

#[test]
#[ignore]
fn construction() {
    let path = OsString::from("resources/osm/small.fmi");

    let parser = osm::fmi::Parser;
    let _graph = parser.parse(&path);

    // TODO check nodes
    // TODO check edges
    // TODO check offset array
    unimplemented!()
}
