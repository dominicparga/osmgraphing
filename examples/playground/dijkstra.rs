use std::ffi::{OsStr, OsString};
use std::time::Instant;

use osmgraphing::osm;
use osmgraphing::routing;

fn parse<S: AsRef<OsStr> + ?Sized>(path: &S) -> routing::Graph {
    match osm::Support::from_path(&path) {
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
    }
}

fn main() {
    // parse -> graph
    let graph = parse(&OsString::from("resources/osm/small.fmi"));
    println!("{}", graph);

    // routing
    let mut dijkstra = routing::Dijkstra::new(&graph);
    let src = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();

    for dst in dsts {
        println!();

        let now = Instant::now();
        let path = dijkstra.compute_shortest_path(src, dst);
        println!(
            "Ran Dijkstra in {} microseconds a.k.a {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        println!(
            "Distance from (id:{}) to (id:{}): {}",
            src, dst, path.cost[dst]
        );
    }
}
