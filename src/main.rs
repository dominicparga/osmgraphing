use osmgraphing::graph;
use osmgraphing::dijkstra;
use dijkstra::ShortestPath;
use graph::Graph;
use graph::Node;
use graph::Edge;

fn main() {
    let a = Node { id: 0, lat: 1.2345, lon: 5.4321, edge_start: 0, edge_end: 1 };
    let b = Node { id: 1, lat: 6.7890, lon: 0.1234, edge_start: 2, edge_end: 3  };
    let c = Node { id: 2, lat: 6.7890, lon: 0.1234, edge_start: 4, edge_end: 4  };
    let d = Node { id: 3, lat: 6.7890, lon: 0.1234, edge_start: 4, edge_end: 4};
    let edges = vec![Edge { id: 0, src: a.id, dest: b.id, weight: 5 }, 
                     Edge { id: 1, src: a.id, dest: c.id, weight: 10 },
                     Edge { id: 2, src: b.id, dest: c.id, weight: 3 },
                     Edge { id: 3, src: b.id, dest: d.id, weight: 10 },
                     Edge { id: 4, src: c.id, dest: d.id, weight: 3 }];
    let nodes = vec![a, b, c, d];
    let graph = Graph { nodes: nodes, edges: edges, node_count: 4, edge_count: 5 };
    let mut dijkstra = dijkstra::init_dijkstra(&graph);
    dijkstra.compute_shortestPath(0, 3);
    println!("{}", graph);
    for i in 0 .. graph.node_count {
        println!("{{Distance to Node {} is {}}}", i, dijkstra.get_distance(i));
    }
    let path = dijkstra.get_Path(0, 3);
    for i in 0 .. path.len() {
        println!("{{Shortest Path leads along edge {}}}", graph.edges[path[i]]);
    }
}

