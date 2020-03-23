use crate::helpers::{assert_graph, defaults, parse, TestEdge, TestNode};
use osmgraphing::{
    configs::Config,
    network::EdgeIdx,
    units::{geo::Coordinate, length::Kilometers, speed::KilometersPerHour, time::Seconds},
};

const CONFIG: &str = defaults::paths::resources::configs::SIMPLE_STUTTGART_FMI;

#[test]
fn fmi_yaml() {
    Config::from_yaml(CONFIG).unwrap();
}

#[test]
fn yaml_str() {
    Config::from_yaml(CONFIG).unwrap();
}

#[test]
fn fmi_graph() {
    let cfg = Config::from_yaml(CONFIG).unwrap();
    let graph = parse(cfg.parser);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let test_nodes: Vec<_> = vec![
        ("Oppenweiler", 26_033_921, (48.9840100, 9.4589188)),
        ("Backnang", 26_160_028, (48.9416023, 9.4332023)),
        ("Waiblingen", 252_787_940, (48.8271096, 9.3098661)),
        ("Endersbach", 298_249_467, (48.8108510, 9.3679493)),
        ("Dead-end", 1_621_605_361, (48.9396327, 9.4188681)),
        ("Stuttgart", 2_933_335_353, (48.7701757, 9.1565768)),
    ]
    .into_iter()
    .map(|(name, id, (lat, lon))| TestNode::new(name, id, Coordinate { lat, lon }, 0, &graph))
    .collect();
    let node_opp = &test_nodes[0];
    let node_bac = &test_nodes[1];
    let node_wai = &test_nodes[2];
    let node_end = &test_nodes[3];
    let node_dea = &test_nodes[4];
    let node_stu = &test_nodes[5];

    // Due to the offset-array, the fwd-edge-ids should match
    // with sorting by src-id, then by dst-id.
    let fwd_test_edges: Vec<_> = vec![
        // name, idx, id, src, dst, kilometers, kmph, s
        (0, &node_opp, &node_bac, 8.0, 50.0, 576.0),
        (1, &node_bac, &node_opp, 8.0, 50.0, 576.0),
        (2, &node_bac, &node_wai, 23.0, 120.0, 690.0),
        (3, &node_bac, &node_end, 22.0, 80.0, 990.0),
        (4, &node_bac, &node_dea, 1.069, 30.0, 128.28),
        (5, &node_wai, &node_bac, 23.0, 120.0, 690.0),
        (6, &node_wai, &node_end, 8.0, 50.0, 576.0),
        (7, &node_wai, &node_stu, 17.0, 100.0, 612.0),
        (8, &node_end, &node_bac, 22.0, 80.0, 990.0),
        (9, &node_end, &node_wai, 8.0, 50.0, 576.0),
        (10, &node_end, &node_stu, 21.0, 80.0, 945.0),
        (11, &node_stu, &node_wai, 17.0, 100.0, 612.0),
        (12, &node_stu, &node_end, 21.0, 80.0, 945.0),
    ]
    .into_iter()
    .map(|(idx, src, dst, kilometers, kmph, s)| {
        // attention: fwd
        TestEdge::new_fwd(
            None,
            EdgeIdx(idx),
            src,
            dst,
            Kilometers(kilometers),
            KilometersPerHour(kmph),
            Seconds(s),
        )
    })
    .collect();

    // Due to the offset-array, the bwd-edge-ids should match
    // with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    let bwd_test_edges: Vec<_> = vec![
        // name, idx, id, src, dst, kilometers, kmph, s
        (0, &node_bac, &node_opp, 8.0, 50.0, 576.0),
        (1, &node_opp, &node_bac, 8.0, 50.0, 576.0),
        (2, &node_wai, &node_bac, 23.0, 120.0, 690.0),
        (3, &node_end, &node_bac, 22.0, 80.0, 990.0),
        (4, &node_dea, &node_bac, 1.069, 30.0, 128.28),
        (5, &node_bac, &node_wai, 23.0, 120.0, 690.0),
        (6, &node_end, &node_wai, 8.0, 50.0, 576.0),
        (7, &node_stu, &node_wai, 17.0, 100.0, 612.0),
        (8, &node_bac, &node_end, 22.0, 80.0, 990.0),
        (9, &node_wai, &node_end, 8.0, 50.0, 576.0),
        (10, &node_stu, &node_end, 21.0, 80.0, 945.0),
        (11, &node_wai, &node_stu, 17.0, 100.0, 612.0),
        (12, &node_end, &node_stu, 21.0, 80.0, 945.0),
    ]
    .into_iter()
    .map(|(idx, src, dst, kilometers, kmph, s)| {
        // attention: bwd
        TestEdge::new_bwd(
            None,
            EdgeIdx(idx),
            src,
            dst,
            Kilometers(kilometers),
            KilometersPerHour(kmph),
            Seconds(s),
        )
    })
    .collect();

    assert_graph(test_nodes, fwd_test_edges, bwd_test_edges, &graph);
}
