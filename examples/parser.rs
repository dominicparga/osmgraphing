use std::ffi::OsString;
use std::time::Instant;

use log::error;

use osmgraphing::{Parser, Parsing};

fn main() {
    env_logger::Builder::from_env("RUST_LOG").init();
    println!("Executing example: dijkstra");

    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/osm/small.fmi"),
    };

    let now = Instant::now();
    let graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    println!(
        "Finished parsing in {} seconds ({} ms).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    println!("");
    println!("{}", graph);
}
