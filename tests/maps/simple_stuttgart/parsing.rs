use crate::helpers::{assert_nodes, defaults, parse, TestEdge, TestNode};
use osmgraphing::{
    configs::{self, Config},
    network::NodeIdx,
};

#[test]
fn fmi_yaml() {
    Config::from_yaml(defaults::paths::resources::configs::SIMPLE_STUTTGART_FMI).unwrap();
}

#[test]
fn yaml_str() {
    let cfg = Config::from_yaml(defaults::paths::resources::configs::SIMPLE_STUTTGART_FMI).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::DURATION_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::LENGTH_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();
}

#[test]
fn fmi_graph() {
    let cfg = Config::from_yaml(defaults::paths::resources::configs::SIMPLE_STUTTGART_FMI).unwrap();
    let graph = parse(cfg.parser);

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
    // name, idx, id, src, dst, length, maxspeed, duration
    let fwd_edge_opp_bac = TestEdge::new_fwd(None, 0, &node_opp, &node_bac, 8.0, 50.0, 576.0);
    let fwd_edge_bac_opp = TestEdge::new_fwd(None, 1, &node_bac, &node_opp, 8.0, 50.0, 576.0);
    let fwd_edge_bac_wai = TestEdge::new_fwd(None, 2, &node_bac, &node_wai, 23.0, 120.0, 690.0);
    let fwd_edge_bac_end = TestEdge::new_fwd(None, 3, &node_bac, &node_end, 22.0, 80.0, 990.0);
    let fwd_edge_bac_dea = TestEdge::new_fwd(None, 4, &node_bac, &node_dea, 1.069, 30.0, 128.28);
    let fwd_edge_wai_bac = TestEdge::new_fwd(None, 5, &node_wai, &node_bac, 23.0, 120.0, 690.0);
    let fwd_edge_wai_end = TestEdge::new_fwd(None, 6, &node_wai, &node_end, 8.0, 50.0, 576.0);
    let fwd_edge_wai_stu = TestEdge::new_fwd(None, 7, &node_wai, &node_stu, 17.0, 100.0, 612.0);
    let fwd_edge_end_bac = TestEdge::new_fwd(None, 8, &node_end, &node_bac, 22.0, 80.0, 990.0);
    let fwd_edge_end_wai = TestEdge::new_fwd(None, 9, &node_end, &node_wai, 8.0, 50.0, 576.0);
    let fwd_edge_end_stu = TestEdge::new_fwd(None, 10, &node_end, &node_stu, 21.0, 80.0, 945.0);
    let fwd_edge_stu_wai = TestEdge::new_fwd(None, 11, &node_stu, &node_wai, 17.0, 100.0, 612.0);
    let fwd_edge_stu_end = TestEdge::new_fwd(None, 12, &node_stu, &node_end, 21.0, 80.0, 945.0);

    // Due to the offset-array, the bwd-edge-ids should match with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    // name, idx, id, src, dst, length, maxspeed, duration
    let bwd_edge_bac_opp = TestEdge::new_bwd(None, 0, &node_bac, &node_opp, 8.0, 50.0, 576.0);
    let bwd_edge_opp_bac = TestEdge::new_bwd(None, 1, &node_opp, &node_bac, 8.0, 50.0, 576.0);
    let bwd_edge_wai_bac = TestEdge::new_bwd(None, 2, &node_wai, &node_bac, 23.0, 120.0, 690.0);
    let bwd_edge_end_bac = TestEdge::new_bwd(None, 3, &node_end, &node_bac, 22.0, 80.0, 990.0);
    let bwd_edge_dea_bac = TestEdge::new_bwd(None, 4, &node_dea, &node_bac, 1.069, 30.0, 128.28);
    let bwd_edge_bac_wai = TestEdge::new_bwd(None, 5, &node_bac, &node_wai, 23.0, 120.0, 690.0);
    let bwd_edge_end_wai = TestEdge::new_bwd(None, 6, &node_end, &node_wai, 8.0, 50.0, 576.0);
    let bwd_edge_stu_wai = TestEdge::new_bwd(None, 7, &node_stu, &node_wai, 17.0, 100.0, 612.0);
    let bwd_edge_bac_end = TestEdge::new_bwd(None, 8, &node_bac, &node_end, 22.0, 80.0, 990.0);
    let bwd_edge_wai_end = TestEdge::new_bwd(None, 9, &node_wai, &node_end, 8.0, 50.0, 576.0);
    let bwd_edge_stu_end = TestEdge::new_bwd(None, 10, &node_stu, &node_end, 21.0, 80.0, 945.0);
    let bwd_edge_wai_stu = TestEdge::new_bwd(None, 11, &node_wai, &node_stu, 17.0, 100.0, 612.0);
    let bwd_edge_end_stu = TestEdge::new_bwd(None, 12, &node_end, &node_stu, 21.0, 80.0, 945.0);

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

    assert_nodes(
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
