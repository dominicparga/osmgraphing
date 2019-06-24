use osmgraphing::routing;

use std::ffi::OsString;
use std::time::Instant;

fn main() {
    let filename = match std::env::args_os().nth(1) {
        Some(filename) => filename,
        // TODO check for windows
        None => OsString::from("custom/resources/graphs/fmi/germany.fmi"),
    };

    let mut graph = routing::Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        node_count: 0,
        edge_count: 0,
    };

    graph.read_graph(filename).expect("error reading file!");
    graph.set_edge_offset();

    // somewhere in Stuttgart (Schwabstrasse)
    // id 20_443_604 osm-id 2_933_335_353 lat 48.77017570000000291 lon 9.15657690000000102

    // "near" Esslingen
    // id:4_647 osm-id:163_354 lat:48.66743380000000485 lon:9.24459110000000095

    // somewhere in Ulm
    // id 9_058_109 osm-id 580_012_224 lat 48.39352330000000535 lon 9.9816315000000006

    // near Aalen
    // id 54_288 osm-id 2_237_652 lat 48.88542720000000230 lon 10.13642900000000147

    // somewhere in Berlin
    // id 296_679 osm-id 26_765_334 lat 52.50536590000000103 lon 13.38662390000000002
    let mut dijkstra = routing::Dijkstra::new(&graph);
    let src = 20_443_604;
    let dsts = vec![9_058_109, 296_679];

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
