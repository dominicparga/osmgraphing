use std::ffi::OsString;

use osmgraphing::osm;

fn main() {
    // TODO check for windows
    let filename = match std::env::args_os().nth(1) {
        Some(filename) => filename,
        None => OsString::from("custom/maps/raw/andorra-latest.osm.pbf"),
    };
    let parser = osm::Parser {};
    parser.parse(&filename);
}
