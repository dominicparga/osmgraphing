use dijkstra::ShortestPath;
use graph::Graph;
use osmgraphing::dijkstra;
use osmgraphing::graph;

fn main() {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        node_count: 0,
        edge_count: 0,
    };

    // TODO check for windows
    graph
        .read_graph("res/graphs/small.txt")
        .expect("error reading file!");
    graph.set_edge_offset();
    //println!("{}", graph);
    let mut dijkstra = dijkstra::init_dijkstra(&graph);
    dijkstra.compute_shortest_path(0, 100000000);
    println!("Distance to Node 4: {}", dijkstra.get_distance(9990));
}
