use std::ffi::OsString;

use log::error;

use super::{TestEdge, TestNode};
use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//

#[test]
fn parse() {
    let path = OsString::from("resources/maps/routing.fmi");
    let _graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
}

#[test]
fn graph_construction() {
    let path = OsString::from("resources/maps/routing.fmi");
    let graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let node_a = TestNode::from("a", 0, 0_1234567, 0_1234567, &graph);
    let node_b = TestNode::from("b", 1, 1_2345678, 1_2345678, &graph);
    let node_c = TestNode::from("c", 2, 2_3456789, 2_3456789, &graph);
    let node_d = TestNode::from("d", 3, 3_4567890, 3_4567890, &graph);
    let node_e = TestNode::from("e", 4, 4_5678901, 4_5678901, &graph);
    let node_f = TestNode::from("f", 5, 5_6789012, 5_6789012, &graph);
    let node_g = TestNode::from("g", 6, 6_7890123, 6_7890123, &graph);
    let node_h = TestNode::from("h", 7, 7_8901234, 7_8901234, &graph);

    // Due to the offset-array, the edge-ids should match with sorting by src-id, then by dst-id.
    // -> testing offset-array
    // name, id, src, dst, meters, maxspeed
    let edge_b_a = TestEdge::from(None, 0, &node_b, &node_a, 1_000, 30);
    let edge_b_c = TestEdge::from(None, 1, &node_b, &node_a, 1_000, 30);
    let edge_c_a = TestEdge::from(None, 2, &node_b, &node_a, 1_000, 30);
    let edge_c_b = TestEdge::from(None, 3, &node_b, &node_a, 1_000, 30);
    let edge_d_b = TestEdge::from(None, 4, &node_b, &node_a, 1_000, 30);
    let edge_d_e = TestEdge::from(None, 5, &node_b, &node_a, 2_000, 30);
    let edge_d_h = TestEdge::from(None, 6, &node_b, &node_a, 1_000, 30);
    let edge_e_d = TestEdge::from(None, 7, &node_b, &node_a, 2_000, 30);
    let edge_e_f = TestEdge::from(None, 8, &node_b, &node_a, 1_000, 30);
    let edge_f_e = TestEdge::from(None, 9, &node_b, &node_a, 1_000, 30);
    let edge_f_h = TestEdge::from(None, 10, &node_b, &node_a, 1_000, 30);
    let edge_g_e = TestEdge::from(None, 11, &node_b, &node_a, 1_000, 30);
    let edge_g_f = TestEdge::from(None, 12, &node_b, &node_a, 1_000, 30);
    let edge_h_c = TestEdge::from(None, 13, &node_b, &node_a, 4_000, 30);
    let edge_h_d = TestEdge::from(None, 14, &node_b, &node_a, 1_000, 30);
    let edge_h_f = TestEdge::from(None, 15, &node_b, &node_a, 1_000, 30);

    //--------------------------------------------------------------------------------------------//
    // testing graph

    assert_eq!(graph.node_count(), 8, "Wrong node-count");
    assert_eq!(graph.edge_count(), 16, "Wrong edge-count");
    assert!(
        graph.edge(24, 42).is_none(),
        "Edge doesn't exist, so graph should return None."
    );
    assert!(
        graph.leaving_edges(424).is_none(),
        "Node's idx is too high, thus the node should not have any leaving edges."
    );
    assert!(
        graph.leaving_edges(node_a.idx).is_none(),
        "Node has no leaving edges, so the method should return None."
    );

    //--------------------------------------------------------------------------------------------//
    // testing nodes

    node_a.assert(&graph);
    node_b.assert(&graph);
    node_c.assert(&graph);
    node_d.assert(&graph);
    node_e.assert(&graph);
    node_f.assert(&graph);
    node_g.assert(&graph);
    node_h.assert(&graph);

    //--------------------------------------------------------------------------------------------//
    // testing edges

    edge_b_a.assert(&graph);
    edge_b_c.assert(&graph);
    edge_c_a.assert(&graph);
    edge_c_b.assert(&graph);
    edge_d_b.assert(&graph);
    edge_d_e.assert(&graph);
    edge_d_h.assert(&graph);
    edge_e_d.assert(&graph);
    edge_e_f.assert(&graph);
    edge_f_e.assert(&graph);
    edge_f_h.assert(&graph);
    edge_g_e.assert(&graph);
    edge_g_f.assert(&graph);
    edge_h_c.assert(&graph);
    edge_h_d.assert(&graph);
    edge_h_f.assert(&graph);
}
