use crate::helpers::{assert_nodes, create_config, defaults, parse, TestEdge, TestNode, TestType};
use osmgraphing::{
    configs::{self, Config},
    network::NodeIdx,
};
use std::path::PathBuf;

#[test]
fn yaml() {
    let mut cfg = Config::from_yaml("resources/configs/simple-stuttgart.fmi.yaml").unwrap();
    cfg.parser.map_file = PathBuf::from("resources/maps/small.fmi");
}

#[test]
fn yaml_str() {
    let mut cfg = Config::from_yaml("resources/configs/simple-stuttgart.fmi.yaml").unwrap();
    cfg.parser.map_file = PathBuf::from("resources/maps/small.fmi");

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::DURATION_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::LENGTH_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();
}

#[test]
fn fmi() {
    let cfg = create_config(TestType::Small, None);
    let graph = parse(cfg.parser);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let node_a = TestNode::new("a", 0, 0.0000000, 0.0000000, &graph);
    let node_b = TestNode::new("b", 1, 0.0000000, 0.0000000, &graph);
    let node_c = TestNode::new("c", 2, 0.0000000, 0.0000000, &graph);
    let node_d = TestNode::new("d", 3, 0.0000000, 0.0000000, &graph);
    let node_e = TestNode::new("e", 4, 0.0000000, 0.0000000, &graph);
    let node_f = TestNode::new("f", 5, 0.0000000, 0.0000000, &graph);
    let node_g = TestNode::new("g", 6, 0.0000000, 0.0000000, &graph);
    let node_h = TestNode::new("h", 7, 0.0000000, 0.0000000, &graph);

    // Due to the offset-array, the fwd-edge-ids should match with sorting by src-id, then by dst-id.
    // name, idx, id, src, dst, length, maxspeed, duration
    let fwd_edge_b_a = TestEdge::new_fwd(None, 0, &node_b, &node_a, 0.001, 30.0, 0.12);
    let fwd_edge_b_c = TestEdge::new_fwd(None, 1, &node_b, &node_c, 0.001, 30.0, 0.12);
    let fwd_edge_c_a = TestEdge::new_fwd(None, 2, &node_c, &node_a, 0.001, 30.0, 0.12);
    let fwd_edge_c_b = TestEdge::new_fwd(None, 3, &node_c, &node_b, 0.001, 30.0, 0.12);
    let fwd_edge_d_b = TestEdge::new_fwd(None, 4, &node_d, &node_b, 0.001, 30.0, 0.12);
    let fwd_edge_d_e = TestEdge::new_fwd(None, 5, &node_d, &node_e, 0.002, 30.0, 0.24);
    let fwd_edge_d_h = TestEdge::new_fwd(None, 6, &node_d, &node_h, 0.001, 30.0, 0.12);
    let fwd_edge_e_d = TestEdge::new_fwd(None, 7, &node_e, &node_d, 0.002, 30.0, 0.24);
    let fwd_edge_e_f = TestEdge::new_fwd(None, 8, &node_e, &node_f, 0.001, 30.0, 0.12);
    let fwd_edge_f_e = TestEdge::new_fwd(None, 9, &node_f, &node_e, 0.001, 30.0, 0.12);
    let fwd_edge_f_h = TestEdge::new_fwd(None, 10, &node_f, &node_h, 0.001, 30.0, 0.12);
    let fwd_edge_g_e = TestEdge::new_fwd(None, 11, &node_g, &node_e, 0.001, 30.0, 0.12);
    let fwd_edge_g_f = TestEdge::new_fwd(None, 12, &node_g, &node_f, 0.001, 30.0, 0.12);
    let fwd_edge_h_c = TestEdge::new_fwd(None, 13, &node_h, &node_c, 0.004, 30.0, 0.48);
    let fwd_edge_h_d = TestEdge::new_fwd(None, 14, &node_h, &node_d, 0.001, 30.0, 0.12);
    let fwd_edge_h_f = TestEdge::new_fwd(None, 15, &node_h, &node_f, 0.001, 30.0, 0.12);

    // Due to the offset-array, the bwd-edge-ids should match with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    // name, idx, id, src, dst, length, maxspeed, duration
    let bwd_edge_a_b = TestEdge::new_bwd(None, 0, &node_a, &node_b, 0.001, 30.0, 0.12);
    let bwd_edge_c_b = TestEdge::new_bwd(None, 1, &node_c, &node_b, 0.001, 30.0, 0.12);
    let bwd_edge_a_c = TestEdge::new_bwd(None, 2, &node_a, &node_c, 0.001, 30.0, 0.12);
    let bwd_edge_b_c = TestEdge::new_bwd(None, 3, &node_b, &node_c, 0.001, 30.0, 0.12);
    let bwd_edge_b_d = TestEdge::new_bwd(None, 4, &node_b, &node_d, 0.001, 30.0, 0.12);
    let bwd_edge_e_d = TestEdge::new_bwd(None, 5, &node_e, &node_d, 0.002, 30.0, 0.24);
    let bwd_edge_h_d = TestEdge::new_bwd(None, 6, &node_h, &node_d, 0.001, 30.0, 0.12);
    let bwd_edge_d_e = TestEdge::new_bwd(None, 7, &node_d, &node_e, 0.002, 30.0, 0.24);
    let bwd_edge_f_e = TestEdge::new_bwd(None, 8, &node_f, &node_e, 0.001, 30.0, 0.12);
    let bwd_edge_e_f = TestEdge::new_bwd(None, 9, &node_e, &node_f, 0.001, 30.0, 0.12);
    let bwd_edge_h_f = TestEdge::new_bwd(None, 10, &node_h, &node_f, 0.001, 30.0, 0.12);
    let bwd_edge_e_g = TestEdge::new_bwd(None, 11, &node_e, &node_g, 0.001, 30.0, 0.12);
    let bwd_edge_f_g = TestEdge::new_bwd(None, 12, &node_f, &node_g, 0.001, 30.0, 0.12);
    let bwd_edge_c_h = TestEdge::new_bwd(None, 13, &node_c, &node_h, 0.004, 30.0, 0.48);
    let bwd_edge_d_h = TestEdge::new_bwd(None, 14, &node_d, &node_h, 0.001, 30.0, 0.12);
    let bwd_edge_f_h = TestEdge::new_bwd(None, 15, &node_f, &node_h, 0.001, 30.0, 0.12);

    //--------------------------------------------------------------------------------------------//
    // testing graph

    let _nodes = graph.nodes();
    let nodes = graph.nodes(); // calling twice should be fine
    let _fwd_edges = graph.fwd_edges();
    let fwd_edges = graph.fwd_edges(); // calling twice should be fine
    let _bwd_edges = graph.bwd_edges();
    let bwd_edges = graph.bwd_edges(); // calling twice should be fine

    let expected = 8;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let expected = 16;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
    let expected = 16;
    assert_eq!(
        bwd_edges.count(),
        expected,
        "Number of bwd-edges in graph should be {} but is {}.",
        expected,
        bwd_edges.count()
    );

    for i in nodes.count()..(2 * nodes.count()) {
        for j in nodes.count()..(2 * nodes.count()) {
            assert!(
                fwd_edges.starting_from(NodeIdx(i)).is_none(),
                "NodeIdx {} >= n={} shouldn't have leaving-edges in fwd-edges",
                i,
                nodes.count()
            );
            assert!(
                bwd_edges.starting_from(NodeIdx(j)).is_none(),
                "NodeIdx {} >= n={} shouldn't have leaving-edges in bwd-edges",
                j,
                nodes.count()
            );
            assert!(
                fwd_edges.between(NodeIdx(i), NodeIdx(j)).is_none(),
                "There should be no fwd-edge from NodeIdx {} to NodeIdx {}.",
                i,
                j
            );
            assert!(
                bwd_edges.between(NodeIdx(j), NodeIdx(i)).is_none(),
                "There should be no bwd-edge from NodeIdx {} to NodeIdx {}.",
                j,
                i
            );
        }
    }

    //--------------------------------------------------------------------------------------------//
    // testing nodes

    assert_nodes(
        &vec![
            node_a, node_b, node_c, node_d, node_e, node_f, node_g, node_h,
        ],
        &nodes,
    );

    //--------------------------------------------------------------------------------------------//
    // testing fwd-edges

    fwd_edge_b_a.assert_correct(&graph);
    fwd_edge_b_c.assert_correct(&graph);
    fwd_edge_c_a.assert_correct(&graph);
    fwd_edge_c_b.assert_correct(&graph);
    fwd_edge_d_b.assert_correct(&graph);
    fwd_edge_d_e.assert_correct(&graph);
    fwd_edge_d_h.assert_correct(&graph);
    fwd_edge_e_d.assert_correct(&graph);
    fwd_edge_e_f.assert_correct(&graph);
    fwd_edge_f_e.assert_correct(&graph);
    fwd_edge_f_h.assert_correct(&graph);
    fwd_edge_g_e.assert_correct(&graph);
    fwd_edge_g_f.assert_correct(&graph);
    fwd_edge_h_c.assert_correct(&graph);
    fwd_edge_h_d.assert_correct(&graph);
    fwd_edge_h_f.assert_correct(&graph);

    //--------------------------------------------------------------------------------------------//
    // testing bwd-edges

    bwd_edge_a_b.assert_correct(&graph);
    bwd_edge_c_b.assert_correct(&graph);
    bwd_edge_a_c.assert_correct(&graph);
    bwd_edge_b_c.assert_correct(&graph);
    bwd_edge_b_d.assert_correct(&graph);
    bwd_edge_e_d.assert_correct(&graph);
    bwd_edge_h_d.assert_correct(&graph);
    bwd_edge_d_e.assert_correct(&graph);
    bwd_edge_f_e.assert_correct(&graph);
    bwd_edge_e_f.assert_correct(&graph);
    bwd_edge_h_f.assert_correct(&graph);
    bwd_edge_e_g.assert_correct(&graph);
    bwd_edge_f_g.assert_correct(&graph);
    bwd_edge_c_h.assert_correct(&graph);
    bwd_edge_d_h.assert_correct(&graph);
    bwd_edge_f_h.assert_correct(&graph);
}
