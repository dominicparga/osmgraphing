#[macro_use]
extern crate log;

use std::ffi::OsString;
use std::time::Instant;

use osmgraphing::osm;
use osmgraphing::Logging;

fn main() {
    Logging::init();

    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/osm/small.fmi"),
    };

    let now = Instant::now();
    let graph = match osm::Support::from_path(&path) {
        Ok(osm::Support::PBF) => {
            let parser = osm::pbf::Parser;
            parser.parse(&path)
        }
        Ok(osm::Support::FMI) => {
            let parser = osm::fmi::Parser;
            parser.parse(&path)
        }
        Ok(osm::Support::XML) => unimplemented!(),
        Err(e) => panic!("{:}", e),
    };
    info!(
        "Finished parsing in {} seconds ({} ms).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);
}
