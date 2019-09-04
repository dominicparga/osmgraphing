use std::ffi::OsString;

use osmgraphing::osm;
use osmgraphing::osm::geo;
use osmgraphing::routing;

//------------------------------------------------------------------------------------------------//
// helpers

struct TestNode {
    name: String,
    id: i64,
    idx: usize,
    decimicro_lat: i32,
    decimicro_lon: i32,
}
impl TestNode {
    fn from(
        name: &str,
        id: i64,
        decimicro_lat: i32,
        decimicro_lon: i32,
        graph: &routing::Graph,
    ) -> TestNode {
        let idx = graph
            .node_idx_from(id)
            .expect(&format!("The node_id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            decimicro_lat,
            decimicro_lon,
        }
    }

    fn assert(&self, graph: &routing::Graph) {
        let node = graph.node(self.idx);
        let coord = geo::Coordinate::new(self.decimicro_lat, self.decimicro_lon);
        assert_eq!(node.id(), self.id, "Wrong node_id for {}", self.name);
        assert_eq!(node.coord(), &coord, "Wrong coordinate for {}", self.name);
    }
}

//------------------------------------------------------------------------------------------------//
// tests

#[test]
fn parsing() {
    let path = OsString::from("resources/osm/small.fmi");

    let parser = osm::fmi::Parser;
    let graph = parser.parse(&path);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // name, id, decimicro_lat, decimicro_lon
    let oppenweiler = TestNode::from("Oppenweiler", 26033921, 489840100, 94589188, &graph);
    let backnang = TestNode::from("Backnang", 26160028, 489416023, 94332023, &graph);
    let endersbach = TestNode::from("Endersbach", 298249467, 488108510, 93679493, &graph);
    let waiblingen = TestNode::from("Waiblingen", 252787940, 488271096, 93098661, &graph);
    let stuttgart = TestNode::from("Stuttgart", 2933335353, 487701757, 91565768, &graph);

    // TODO store edges

    //--------------------------------------------------------------------------------------------//
    // testing general

    assert_eq!(graph.node_count(), 5, "Wrong node-count");
    assert_eq!(graph.edge_count(), 12, "Wrong edge-count");

    //--------------------------------------------------------------------------------------------//
    // testing nodes

    oppenweiler.assert(&graph);
    backnang.assert(&graph);
    endersbach.assert(&graph);
    waiblingen.assert(&graph);
    stuttgart.assert(&graph);

    //--------------------------------------------------------------------------------------------//
    // testing edges

    // TODO

    //--------------------------------------------------------------------------------------------//
    // testing offset-array

    // TODO
}
