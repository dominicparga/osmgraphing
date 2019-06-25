use std::ffi::OsString;
use std::time::Instant;

use osmgraphing::osm;

fn main() {
    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/osm/small.fmi"),
    };

    println!("Start parsing...");
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
    println!(
        "Finished in {} seconds ({} ms).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    println!();
    println!("{}", graph);
}
