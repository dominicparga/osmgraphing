use osmgraphing::routing;

use std::ffi::OsString;
use std::time::Instant;

fn main() {
    let filename = match std::env::args_os().nth(1) {
        Some(filename) => filename,
        // TODO check for windows
        None => OsString::from("custom/res/graphs/fmi/germany.fmi"),
    };

    let mut graph = routing::Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        node_count: 0,
        edge_count: 0,
    };

    graph.read_graph(filename).expect("error reading file!");
    graph.set_edge_offset();
    //println!("{}", graph);
    let now = Instant::now();
    let mut dijkstra = routing::Dijkstra::new(&graph);
    let src = 4_647;
    let dst = 54_288;
    let _ = dijkstra.compute_shortest_path(src, dst);
    let path = dijkstra.compute_shortest_path(src, dst);
    println!(
        "Ran Dijkstra in {} microseconds a.k.a {} seconds",
        now.elapsed().as_micros(),
        now.elapsed().as_secs()
    );
    println!("Distance from (id:{}) to (id:{}): {}", src, dst, path.cost[dst]);
}
