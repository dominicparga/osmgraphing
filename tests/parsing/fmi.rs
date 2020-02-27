use super::{TestEdge, TestNode, TestType};
use osmgraphing::network::{EdgeIdx, NodeIdx};

//------------------------------------------------------------------------------------------------//
// TODO take results from actions of commit f28d88a

pub fn simple_stuttgart() {
    let cfg = super::create_config(TestType::SimpleStuttgart);
    let graph = super::parse(cfg.graph);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let node_opp = TestNode::new("Oppenweiler", 26_033_921, 48.9840100, 9.4589188, &graph);
    let node_bac = TestNode::new("Backnang", 26_160_028, 48.9416023, 9.4332023, &graph);
    let node_wai = TestNode::new("Waiblingen", 252_787_940, 48.8271096, 9.3098661, &graph);
    let node_end = TestNode::new("Endersbach", 298_249_467, 48.8108510, 9.3679493, &graph);
    let node_dea = TestNode::new("Dead-end", 1_621_605_361, 48.9396327, 9.4188681, &graph);
    let node_stu = TestNode::new("Stuttgart", 2_933_335_353, 48.7701757, 9.1565768, &graph);

    // Due to the offset-array, the fwd-edge-ids should match with sorting by src-id, then by dst-id.
    // name, idx, id, src, dst, meters, maxspeed, ms
    let fwd_edge_opp_bac =
        TestEdge::new_fwd(None, EdgeIdx(0), &node_opp, &node_bac, 8_000, 50, 576_000);
    let fwd_edge_bac_opp =
        TestEdge::new_fwd(None, EdgeIdx(1), &node_bac, &node_opp, 8_000, 50, 576_000);
    let fwd_edge_bac_wai =
        TestEdge::new_fwd(None, EdgeIdx(2), &node_bac, &node_wai, 23_000, 120, 690_000);
    let fwd_edge_bac_end =
        TestEdge::new_fwd(None, EdgeIdx(3), &node_bac, &node_end, 22_000, 80, 990_000);
    // 1_069 is the length of a straight line, since the file contains trash in there.
    let fwd_edge_bac_dea =
        TestEdge::new_fwd(None, EdgeIdx(4), &node_bac, &node_dea, 1_069, 30, 128_280);
    let fwd_edge_wai_bac =
        TestEdge::new_fwd(None, EdgeIdx(5), &node_wai, &node_bac, 23_000, 120, 690_000);
    let fwd_edge_wai_end =
        TestEdge::new_fwd(None, EdgeIdx(6), &node_wai, &node_end, 8_000, 50, 576_000);
    let fwd_edge_wai_stu =
        TestEdge::new_fwd(None, EdgeIdx(7), &node_wai, &node_stu, 17_000, 100, 612_000);
    let fwd_edge_end_bac =
        TestEdge::new_fwd(None, EdgeIdx(8), &node_end, &node_bac, 22_000, 80, 990_000);
    let fwd_edge_end_wai =
        TestEdge::new_fwd(None, EdgeIdx(9), &node_end, &node_wai, 8_000, 50, 576_000);
    let fwd_edge_end_stu =
        TestEdge::new_fwd(None, EdgeIdx(10), &node_end, &node_stu, 21_000, 80, 945_000);
    let fwd_edge_stu_wai = TestEdge::new_fwd(
        None,
        EdgeIdx(11),
        &node_stu,
        &node_wai,
        17_000,
        100,
        612_000,
    );
    let fwd_edge_stu_end =
        TestEdge::new_fwd(None, EdgeIdx(12), &node_stu, &node_end, 21_000, 80, 945_000);

    // Due to the offset-array, the bwd-edge-ids should match with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    // name, idx, id, src, dst, meters, maxspeed, ms
    let bwd_edge_bac_opp =
        TestEdge::new_bwd(None, EdgeIdx(0), &node_bac, &node_opp, 8_000, 50, 576_000);
    let bwd_edge_opp_bac =
        TestEdge::new_bwd(None, EdgeIdx(1), &node_opp, &node_bac, 8_000, 50, 576_000);
    let bwd_edge_wai_bac =
        TestEdge::new_bwd(None, EdgeIdx(2), &node_wai, &node_bac, 23_000, 120, 690_000);
    let bwd_edge_end_bac =
        TestEdge::new_bwd(None, EdgeIdx(3), &node_end, &node_bac, 22_000, 80, 990_000);
    // 1_069 is the length of a straight line, since the file contains trash in there.
    let bwd_edge_dea_bac =
        TestEdge::new_bwd(None, EdgeIdx(4), &node_dea, &node_bac, 1_069, 30, 128_280);
    let bwd_edge_bac_wai =
        TestEdge::new_bwd(None, EdgeIdx(5), &node_bac, &node_wai, 23_000, 120, 690_000);
    let bwd_edge_end_wai =
        TestEdge::new_bwd(None, EdgeIdx(6), &node_end, &node_wai, 8_000, 50, 576_000);
    let bwd_edge_stu_wai =
        TestEdge::new_bwd(None, EdgeIdx(7), &node_stu, &node_wai, 17_000, 100, 612_000);
    let bwd_edge_bac_end =
        TestEdge::new_bwd(None, EdgeIdx(8), &node_bac, &node_end, 22_000, 80, 990_000);
    let bwd_edge_wai_end =
        TestEdge::new_bwd(None, EdgeIdx(9), &node_wai, &node_end, 8_000, 50, 576_000);
    let bwd_edge_stu_end =
        TestEdge::new_bwd(None, EdgeIdx(10), &node_stu, &node_end, 21_000, 80, 945_000);
    let bwd_edge_wai_stu = TestEdge::new_bwd(
        None,
        EdgeIdx(11),
        &node_wai,
        &node_stu,
        17_000,
        100,
        612_000,
    );
    let bwd_edge_end_stu =
        TestEdge::new_bwd(None, EdgeIdx(12), &node_end, &node_stu, 21_000, 80, 945_000);

    //--------------------------------------------------------------------------------------------//
    // testing graph

    let _nodes = graph.nodes();
    let nodes = graph.nodes(); // calling twice should be fine
    let _fwd_edges = graph.fwd_edges();
    let fwd_edges = graph.fwd_edges(); // calling twice should be fine
    let _bwd_edges = graph.bwd_edges();
    let bwd_edges = graph.bwd_edges(); // calling twice should be fine

    let expected = 6;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let expected = 13;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
    let expected = 13;
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

    //------------------------------------------------------------------------------------//
    // testing nodes

    super::assert_nodes(
        &vec![node_opp, node_bac, node_end, node_wai, node_dea, node_stu],
        &nodes,
    );

    //------------------------------------------------------------------------------------//
    // testing fwd-edges

    fwd_edge_opp_bac.assert_correct(&graph);
    fwd_edge_bac_opp.assert_correct(&graph);
    fwd_edge_bac_wai.assert_correct(&graph);
    fwd_edge_bac_end.assert_correct(&graph);
    fwd_edge_bac_dea.assert_correct(&graph);
    fwd_edge_wai_bac.assert_correct(&graph);
    fwd_edge_wai_end.assert_correct(&graph);
    fwd_edge_wai_stu.assert_correct(&graph);
    fwd_edge_end_bac.assert_correct(&graph);
    fwd_edge_end_wai.assert_correct(&graph);
    fwd_edge_end_stu.assert_correct(&graph);
    fwd_edge_stu_wai.assert_correct(&graph);
    fwd_edge_stu_end.assert_correct(&graph);

    //------------------------------------------------------------------------------------//
    // testing fwd-edges

    bwd_edge_bac_opp.assert_correct(&graph);
    bwd_edge_opp_bac.assert_correct(&graph);
    bwd_edge_wai_bac.assert_correct(&graph);
    bwd_edge_end_bac.assert_correct(&graph);
    bwd_edge_bac_wai.assert_correct(&graph);
    bwd_edge_end_wai.assert_correct(&graph);
    bwd_edge_stu_wai.assert_correct(&graph);
    bwd_edge_bac_end.assert_correct(&graph);
    bwd_edge_wai_end.assert_correct(&graph);
    bwd_edge_stu_end.assert_correct(&graph);
    bwd_edge_dea_bac.assert_correct(&graph);
    bwd_edge_wai_stu.assert_correct(&graph);
    bwd_edge_end_stu.assert_correct(&graph);
}

pub fn small() {
    let cfg = super::create_config(TestType::Small);
    let graph = super::parse(cfg.graph);

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
    // name, idx, id, src, dst, meters, maxspeed, ms
    let fwd_edge_b_a = TestEdge::new_fwd(None, EdgeIdx(0), &node_b, &node_a, 1, 30, 120);
    let fwd_edge_b_c = TestEdge::new_fwd(None, EdgeIdx(1), &node_b, &node_c, 1, 30, 120);
    let fwd_edge_c_a = TestEdge::new_fwd(None, EdgeIdx(2), &node_c, &node_a, 1, 30, 120);
    let fwd_edge_c_b = TestEdge::new_fwd(None, EdgeIdx(3), &node_c, &node_b, 1, 30, 120);
    let fwd_edge_d_b = TestEdge::new_fwd(None, EdgeIdx(4), &node_d, &node_b, 1, 30, 120);
    let fwd_edge_d_e = TestEdge::new_fwd(None, EdgeIdx(5), &node_d, &node_e, 2, 30, 240);
    let fwd_edge_d_h = TestEdge::new_fwd(None, EdgeIdx(6), &node_d, &node_h, 1, 30, 120);
    let fwd_edge_e_d = TestEdge::new_fwd(None, EdgeIdx(7), &node_e, &node_d, 2, 30, 240);
    let fwd_edge_e_f = TestEdge::new_fwd(None, EdgeIdx(8), &node_e, &node_f, 1, 30, 120);
    let fwd_edge_f_e = TestEdge::new_fwd(None, EdgeIdx(9), &node_f, &node_e, 1, 30, 120);
    let fwd_edge_f_h = TestEdge::new_fwd(None, EdgeIdx(10), &node_f, &node_h, 1, 30, 120);
    let fwd_edge_g_e = TestEdge::new_fwd(None, EdgeIdx(11), &node_g, &node_e, 1, 30, 120);
    let fwd_edge_g_f = TestEdge::new_fwd(None, EdgeIdx(12), &node_g, &node_f, 1, 30, 120);
    let fwd_edge_h_c = TestEdge::new_fwd(None, EdgeIdx(13), &node_h, &node_c, 4, 30, 480);
    let fwd_edge_h_d = TestEdge::new_fwd(None, EdgeIdx(14), &node_h, &node_d, 1, 30, 120);
    let fwd_edge_h_f = TestEdge::new_fwd(None, EdgeIdx(15), &node_h, &node_f, 1, 30, 120);

    // Due to the offset-array, the bwd-edge-ids should match with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    // name, idx, id, src, dst, meters, maxspeed, ms
    let bwd_edge_a_b = TestEdge::new_bwd(None, EdgeIdx(0), &node_a, &node_b, 1, 30, 120);
    let bwd_edge_c_b = TestEdge::new_bwd(None, EdgeIdx(1), &node_c, &node_b, 1, 30, 120);
    let bwd_edge_a_c = TestEdge::new_bwd(None, EdgeIdx(2), &node_a, &node_c, 1, 30, 120);
    let bwd_edge_b_c = TestEdge::new_bwd(None, EdgeIdx(3), &node_b, &node_c, 1, 30, 120);
    let bwd_edge_b_d = TestEdge::new_bwd(None, EdgeIdx(4), &node_b, &node_d, 1, 30, 120);
    let bwd_edge_e_d = TestEdge::new_bwd(None, EdgeIdx(5), &node_e, &node_d, 2, 30, 240);
    let bwd_edge_h_d = TestEdge::new_bwd(None, EdgeIdx(6), &node_h, &node_d, 1, 30, 120);
    let bwd_edge_d_e = TestEdge::new_bwd(None, EdgeIdx(7), &node_d, &node_e, 2, 30, 240);
    let bwd_edge_f_e = TestEdge::new_bwd(None, EdgeIdx(8), &node_f, &node_e, 1, 30, 120);
    let bwd_edge_e_f = TestEdge::new_bwd(None, EdgeIdx(9), &node_e, &node_f, 1, 30, 120);
    let bwd_edge_h_f = TestEdge::new_bwd(None, EdgeIdx(10), &node_h, &node_f, 1, 30, 120);
    let bwd_edge_e_g = TestEdge::new_bwd(None, EdgeIdx(11), &node_e, &node_g, 1, 30, 120);
    let bwd_edge_f_g = TestEdge::new_bwd(None, EdgeIdx(12), &node_f, &node_g, 1, 30, 120);
    let bwd_edge_c_h = TestEdge::new_bwd(None, EdgeIdx(13), &node_c, &node_h, 4, 30, 480);
    let bwd_edge_d_h = TestEdge::new_bwd(None, EdgeIdx(14), &node_d, &node_h, 1, 30, 120);
    let bwd_edge_f_h = TestEdge::new_bwd(None, EdgeIdx(15), &node_f, &node_h, 1, 30, 120);

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

    super::assert_nodes(
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

pub fn bidirectional_bait() {
    let cfg = super::create_config(TestType::BidirectionalBait);
    let graph = super::parse(cfg.graph);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let node_ll = TestNode::new("left", 0, 0.0000000, 0.0000000, &graph);
    let node_bb = TestNode::new("bottom", 1, 0.0000000, 0.0000000, &graph);
    let node_rr = TestNode::new("right", 2, 0.0000000, 0.0000000, &graph);
    let node_tr = TestNode::new("top-right", 3, 0.0000000, 0.0000000, &graph);
    let node_tl = TestNode::new("top-left", 4, 0.0000000, 0.0000000, &graph);

    // Due to the offset-array, the fwd-edge-ids should match with sorting by src-id, then by dst-id.
    // name, idx, id, src, dst, meters, maxspeed, ms
    let fwd_edge_ll_bb = TestEdge::new_fwd(None, EdgeIdx(0), &node_ll, &node_bb, 5, 30, 600);
    let fwd_edge_ll_tl = TestEdge::new_fwd(None, EdgeIdx(1), &node_ll, &node_tl, 3, 30, 360);
    let fwd_edge_bb_ll = TestEdge::new_fwd(None, EdgeIdx(2), &node_bb, &node_ll, 5, 30, 600);
    let fwd_edge_bb_rr = TestEdge::new_fwd(None, EdgeIdx(3), &node_bb, &node_rr, 5, 30, 600);
    let fwd_edge_rr_bb = TestEdge::new_fwd(None, EdgeIdx(4), &node_rr, &node_bb, 5, 30, 600);
    let fwd_edge_rr_tr = TestEdge::new_fwd(None, EdgeIdx(5), &node_rr, &node_tr, 3, 30, 360);
    let fwd_edge_tr_rr = TestEdge::new_fwd(None, EdgeIdx(6), &node_tr, &node_rr, 3, 30, 360);
    let fwd_edge_tr_tl = TestEdge::new_fwd(None, EdgeIdx(7), &node_tr, &node_tl, 3, 30, 360);
    let fwd_edge_tl_ll = TestEdge::new_fwd(None, EdgeIdx(8), &node_tl, &node_ll, 3, 30, 360);
    let fwd_edge_tl_tr = TestEdge::new_fwd(None, EdgeIdx(9), &node_tl, &node_tr, 3, 30, 360);

    // Due to the offset-array, the bwd-edge-ids should match with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    // name, idx, id, src, dst, meters, maxspeed, ms
    let bwd_edge_bb_ll = TestEdge::new_bwd(None, EdgeIdx(0), &node_bb, &node_ll, 5, 30, 600);
    let bwd_edge_tl_ll = TestEdge::new_bwd(None, EdgeIdx(1), &node_tl, &node_ll, 3, 30, 360);
    let bwd_edge_ll_bb = TestEdge::new_bwd(None, EdgeIdx(2), &node_ll, &node_bb, 5, 30, 600);
    let bwd_edge_rr_bb = TestEdge::new_bwd(None, EdgeIdx(3), &node_rr, &node_bb, 5, 30, 600);
    let bwd_edge_bb_rr = TestEdge::new_bwd(None, EdgeIdx(4), &node_bb, &node_rr, 5, 30, 600);
    let bwd_edge_tr_rr = TestEdge::new_bwd(None, EdgeIdx(5), &node_tr, &node_rr, 3, 30, 360);
    let bwd_edge_rr_tr = TestEdge::new_bwd(None, EdgeIdx(6), &node_rr, &node_tr, 3, 30, 360);
    let bwd_edge_tl_tr = TestEdge::new_bwd(None, EdgeIdx(7), &node_tl, &node_tr, 3, 30, 360);
    let bwd_edge_ll_tl = TestEdge::new_bwd(None, EdgeIdx(8), &node_ll, &node_tl, 3, 30, 360);
    let bwd_edge_tr_tl = TestEdge::new_bwd(None, EdgeIdx(9), &node_tr, &node_tl, 3, 30, 360);

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

    super::assert_nodes(&vec![node_ll, node_bb, node_rr, node_tr, node_tl], &nodes);

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
