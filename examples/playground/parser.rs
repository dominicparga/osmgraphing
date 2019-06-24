use std::ffi::{OsString, OsStr};

use osmgraphing::osm;

fn parse_pbf<S: AsRef<OsStr> + ?Sized>(path: &S) {
    let parser = osm::pbf::Parser;
    match parser.parse(&path) {
        Err(e) => panic!("{:}", e),
        _ => (),
    }
}

fn main() {
    let filename = match std::env::args_os().nth(1) {
        Some(filename) => filename,
        None => OsString::from("custom/resources/osm/raw/andorra-latest.osm.pbf"),
    };

    // check if filetype is supported
    match osm::Support::from_path(&filename) {
        Ok(osm::Support::PBF) => parse_pbf(&filename),
        Err(e) => panic!("{:}", e),
    };
}
