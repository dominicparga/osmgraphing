use std::ffi::OsString;

use osmgraphing::osm;

fn main() {
    let filename = match std::env::args_os().nth(1) {
        Some(filename) => filename,
        None => OsString::from("custom/resources/osm/raw/andorra-latest.osm.pbf"),
    };

    // check if filetype is supported
    let parser = match osm::Support::from_path(&filename) {
        Ok(osm::Support::PBF) => osm::pbf::Parser,
        Err(e) => panic!("{:}", e),
    };

    match parser.parse(&filename) {
        Err(e) => panic!("{:}", e),
        _ => (),
    }
}
