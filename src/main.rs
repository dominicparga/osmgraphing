// mod graph;
// use graph::Graph;
// use graph::Node;
// use graph::Edge;

// mod parser;
// use parser::XmlParser;

mod osm;
use osm::Read;

fn main() {
    println!("Hello, world!");

    // let a = Node { id: 0, lat: 1.2345, lon: 5.4321 };
    // let b = Node { id: 1, lat: 6.7890, lon: 0.1234 };
    // let edges = vec![Edge { id: 42, src: a.id, dst: b.id }];
    // let nodes = vec![a, b];
    // let graph = Graph { nodes: nodes, edges: edges };

    // println!("{}", graph);

    // let p = XmlParser { ..Default::default() };
    // p.apply().apply().apply();

    let mut reader = match std::env::args_os().nth(1) {
        Some(filename)  => osm::pbf::Reader::from_os_str(&filename),
        None            => osm::pbf::Reader::from_str("custom/maps/raw/andorra-latest.osm.pbf"),
    };
    reader.stuff();
}
