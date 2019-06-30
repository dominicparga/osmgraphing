#[macro_use]
extern crate log;

use std::ffi::OsString;
use std::time::Instant;

use osmgraphing::Logging;
use osmgraphing::osm;
use osmgraphing::routing;

fn main() {
    //----------------------------------------------------------------------------------------------
    // parsing

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

    //----------------------------------------------------------------------------------------------
    // dijkstra

    // routing
    let mut dijkstra = routing::Dijkstra::new(&graph);
    let src_idx = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();
    // let dsts: Vec<usize> = vec![80]; problem on baden-wuerttemberg.osm.pbf

    let src = graph.node(src_idx);

    for dst_idx in dsts {
        let dst = graph.node(dst_idx);

        info!("");

        let now = Instant::now();
        let path = dijkstra.compute_shortest_path(src_idx, dst_idx);
        info!(
            "Ran Dijkstra in {} microseconds a.k.a {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        info!(
            "Distance {} m from ({}) to ({}).",
            path.cost[dst_idx], src.lat(), dst
        );
    }
}
