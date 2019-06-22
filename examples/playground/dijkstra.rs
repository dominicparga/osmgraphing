use osmgraphing::routing;

use std::ffi::OsString;
use std::time::Instant;

fn main() {
    let filename = match std::env::args_os().nth(1) {
        Some(filename) => filename,
        None           => OsString::from("custom/res/graphs/fmi/germany.fmi"),
    };

    let mut graph = routing::Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        node_count: 0,
        edge_count: 0
    };

    // TODO check for windows
    graph.read_graph(filename).expect("error reading file!");
    graph.set_edge_offset();
    //println!("{}", graph);
    let now = Instant::now();
    let mut dijkstra = routing::Dijkstra::new(&graph);
    dijkstra.compute_shortest_path(0, 100000000);
    println!("Ran Dijkstra in {} microseconds a.k.a {} seconds", now.elapsed().as_micros(),now.elapsed().as_secs());
    println!("Distance to Node 4: {}", dijkstra.get_distance(9990));
}
