use std::ffi::OsString;
use std::time::Instant;

use osmgraphing::osm;

fn main() {
    env_logger::Builder::from_env("RUST_LOG").init();
    println!("Executing example: dijkstra");

    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/osm/small.fmi"),
    };

    let now = Instant::now();
    let graph = match osm::Support::from_path(&path) {
        Ok(osm::Support::PBF) => osm::pbf::Parser::parse(&path),
        Ok(osm::Support::FMI) => osm::fmi::Parser::parse(&path),
        Ok(osm::Support::XML) => unimplemented!(),
        Err(e) => panic!("{:}", e),
    };
    println!(
        "Finished parsing in {} seconds ({} ms).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    println!("");
    println!("{}", graph);
}
