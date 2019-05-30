mod graph;
use graph::Read;

fn main() {
    //println!("Hello, world!");

    /* let a = graph::Node { id: 0, lat: 1.2345, lon: 5.4321 };
    let b = graph::Node { id: 1, lat: 6.7890, lon: 0.1234 };
    let edges = vec![graph::Edge { id: 42, src: a.id, dst: b.id }];
    let nodes = vec![a, b];
    let graph = graph::Graph { nodes: nodes, edges: edges }; */
    let mut graph = graph::Graph {number_of_nodes: 0, number_of_edges: 0, nodes : Vec::new(), edges: Vec::new()};
    graph.read_graph("test_graph.txt").expect("error");
    println!("{}", graph);
}
