//--------------------------------------------------------------------------------------------------
// test following mods by mentioning them here

mod graph;
mod support;

//--------------------------------------------------------------------------------------------------

use osmgraphing::osm;
use std::ffi::OsString;

//--------------------------------------------------------------------------------------------------

#[test]
#[ignore]
fn parse_fmi() {
    let path = OsString::from("resources/osm/small.fmi");
    let parser = osm::fmi::Parser;
    let _graph = parser.parse(&path);

    // check graph structure
    unimplemented!()
}

#[test]
#[ignore]
fn parse_pbf() {
    let path = OsString::from("resources/osm/small.pbf"); // file missing
    let parser = osm::pbf::Parser;
    let _graph = parser.parse(&path);

    // check graph structure
    unimplemented!()
}

#[test]
#[ignore]
fn parse_osm() {
    let path = OsString::from("resources/osm/small.osm"); // file missing
    let parser = osm::xml::Parser;
    let _graph = parser.parse(&path);

    // check graph structure
    unimplemented!()
}
