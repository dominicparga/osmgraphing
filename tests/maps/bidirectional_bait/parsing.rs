use crate::helpers::{assert_nodes, defaults, parse, TestEdge, TestNode};
use osmgraphing::{
    configs::{self, Config},
    network::NodeIdx,
};

#[test]
fn fmi_yaml() {
    Config::from_yaml(defaults::paths::resources::configs::BIDIRECTIONAL_BAIT_FMI).unwrap();
}

#[test]
fn yaml_str() {
    let cfg =
        Config::from_yaml(defaults::paths::resources::configs::BIDIRECTIONAL_BAIT_FMI).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::DURATION_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::LENGTH_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();
}

#[test]
fn fmi_graph() {
    let cfg =
        Config::from_yaml(defaults::paths::resources::configs::BIDIRECTIONAL_BAIT_FMI).unwrap();
    let graph = parse(cfg.parser);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let node_ll = TestNode::new("left", 0, 0.0000000, 0.0000000, 0, &graph);
    let node_bb = TestNode::new("bottom", 1, 0.0000000, 0.0000000, 0, &graph);
    let node_rr = TestNode::new("right", 2, 0.0000000, 0.0000000, 0, &graph);
    let node_tr = TestNode::new("top-right", 3, 0.0000000, 0.0000000, 0, &graph);
    let node_tl = TestNode::new("top-left", 4, 0.0000000, 0.0000000, 0, &graph);

    // Due to the offset-array, the fwd-edge-ids should match with sorting by src-id, then by dst-id.
    // name, idx, id, src, dst, length, maxspeed, duration
    let fwd_edge_ll_bb = TestEdge::new_fwd(None, 0, &node_ll, &node_bb, 0.005, 30.0, 0.60);
    let fwd_edge_ll_tl = TestEdge::new_fwd(None, 1, &node_ll, &node_tl, 0.003, 30.0, 0.36);
    let fwd_edge_bb_ll = TestEdge::new_fwd(None, 2, &node_bb, &node_ll, 0.005, 30.0, 0.60);
    let fwd_edge_bb_rr = TestEdge::new_fwd(None, 3, &node_bb, &node_rr, 0.005, 30.0, 0.60);
    let fwd_edge_rr_bb = TestEdge::new_fwd(None, 4, &node_rr, &node_bb, 0.005, 30.0, 0.60);
    let fwd_edge_rr_tr = TestEdge::new_fwd(None, 5, &node_rr, &node_tr, 0.003, 30.0, 0.36);
    let fwd_edge_tr_rr = TestEdge::new_fwd(None, 6, &node_tr, &node_rr, 0.003, 30.0, 0.36);
    let fwd_edge_tr_tl = TestEdge::new_fwd(None, 7, &node_tr, &node_tl, 0.003, 30.0, 0.36);
    let fwd_edge_tl_ll = TestEdge::new_fwd(None, 8, &node_tl, &node_ll, 0.003, 30.0, 0.36);
    let fwd_edge_tl_tr = TestEdge::new_fwd(None, 9, &node_tl, &node_tr, 0.003, 30.0, 0.36);

    // Due to the offset-array, the bwd-edge-ids should match with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    // name, idx, id, src, dst, length, maxspeed, duration
    let bwd_edge_bb_ll = TestEdge::new_bwd(None, 0, &node_bb, &node_ll, 0.005, 30.0, 0.60);
    let bwd_edge_tl_ll = TestEdge::new_bwd(None, 1, &node_tl, &node_ll, 0.003, 30.0, 0.36);
    let bwd_edge_ll_bb = TestEdge::new_bwd(None, 2, &node_ll, &node_bb, 0.005, 30.0, 0.60);
    let bwd_edge_rr_bb = TestEdge::new_bwd(None, 3, &node_rr, &node_bb, 0.005, 30.0, 0.60);
    let bwd_edge_bb_rr = TestEdge::new_bwd(None, 4, &node_bb, &node_rr, 0.005, 30.0, 0.60);
    let bwd_edge_tr_rr = TestEdge::new_bwd(None, 5, &node_tr, &node_rr, 0.003, 30.0, 0.36);
    let bwd_edge_rr_tr = TestEdge::new_bwd(None, 6, &node_rr, &node_tr, 0.003, 30.0, 0.36);
    let bwd_edge_tl_tr = TestEdge::new_bwd(None, 7, &node_tl, &node_tr, 0.003, 30.0, 0.36);
    let bwd_edge_ll_tl = TestEdge::new_bwd(None, 8, &node_ll, &node_tl, 0.003, 30.0, 0.36);
    let bwd_edge_tr_tl = TestEdge::new_bwd(None, 9, &node_tr, &node_tl, 0.003, 30.0, 0.36);

    //--------------------------------------------------------------------------------------------//
    // testing graph

    let _nodes = graph.nodes();
    let nodes = graph.nodes(); // calling twice should be fine
    let _fwd_edges = graph.fwd_edges();
    let fwd_edges = graph.fwd_edges(); // calling twice should be fine
    let _bwd_edges = graph.bwd_edges();
    let bwd_edges = graph.bwd_edges(); // calling twice should be fine

    let expected = 5;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let expected = 10;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
    let expected = 10;
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

    assert_nodes(&vec![node_ll, node_bb, node_rr, node_tr, node_tl], &nodes);

    //--------------------------------------------------------------------------------------------//
    // testing fwd-edges

    fwd_edge_ll_bb.assert_correct(&graph);
    fwd_edge_bb_rr.assert_correct(&graph);
    fwd_edge_rr_tr.assert_correct(&graph);
    fwd_edge_tr_tl.assert_correct(&graph);
    fwd_edge_tl_ll.assert_correct(&graph);
    fwd_edge_ll_tl.assert_correct(&graph);
    fwd_edge_tl_tr.assert_correct(&graph);
    fwd_edge_tr_rr.assert_correct(&graph);
    fwd_edge_rr_bb.assert_correct(&graph);
    fwd_edge_bb_ll.assert_correct(&graph);

    //--------------------------------------------------------------------------------------------//
    // testing bwd-edges

    bwd_edge_bb_ll.assert_correct(&graph);
    bwd_edge_rr_bb.assert_correct(&graph);
    bwd_edge_tr_rr.assert_correct(&graph);
    bwd_edge_tl_tr.assert_correct(&graph);
    bwd_edge_ll_tl.assert_correct(&graph);
    bwd_edge_tl_ll.assert_correct(&graph);
    bwd_edge_tr_tl.assert_correct(&graph);
    bwd_edge_rr_tr.assert_correct(&graph);
    bwd_edge_bb_rr.assert_correct(&graph);
    bwd_edge_ll_bb.assert_correct(&graph);
}
