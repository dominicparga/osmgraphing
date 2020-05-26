use crate::helpers::{assert_graph, defaults, parse, TestEdge, TestNode};
use defaults::paths::resources::small as resources;
use kissunits::{
    distance::Kilometers,
    geo::Coordinate,
    speed::KilometersPerHour,
    time::{Minutes, Seconds},
};
use osmgraphing::{configs, network::EdgeIdx};

#[test]
fn ch_fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    assert!(configs::writing::network::Config::try_from_yaml(resources::CH_FMI_YAML).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(resources::CH_FMI_YAML).is_ok());
    assert!(configs::routing::Config::try_from_yaml(resources::CH_FMI_YAML, &parsing_cfg).is_err());
}

#[test]
fn fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    assert!(configs::writing::network::Config::try_from_yaml(resources::FMI_YAML).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(resources::FMI_YAML).is_ok());
    assert!(configs::routing::Config::try_from_yaml(resources::FMI_YAML, &parsing_cfg).is_err());
}

#[test]
fn yaml_str() {
    // TODO
    fmi_yaml();
}

#[test]
fn fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    let graph = parse(parsing_cfg);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let test_nodes = vec![
        TestNode::new("a", 0, Coordinate::zero(), 0, &graph),
        TestNode::new("b", 1, Coordinate::zero(), 0, &graph),
        TestNode::new("c", 2, Coordinate::zero(), 0, &graph),
        TestNode::new("d", 3, Coordinate::zero(), 0, &graph),
        TestNode::new("e", 4, Coordinate::zero(), 0, &graph),
        TestNode::new("f", 5, Coordinate::zero(), 0, &graph),
        TestNode::new("g", 6, Coordinate::zero(), 0, &graph),
        TestNode::new("h", 7, Coordinate::zero(), 0, &graph),
    ];
    let node_a = &test_nodes[0];
    let node_b = &test_nodes[1];
    let node_c = &test_nodes[2];
    let node_d = &test_nodes[3];
    let node_e = &test_nodes[4];
    let node_f = &test_nodes[5];
    let node_g = &test_nodes[6];
    let node_h = &test_nodes[7];

    // Due to the offset-array, the fwd-edge-ids should match
    // with sorting by src-id, then by dst-id.
    let fwd_test_edges: Vec<_> = vec![
        // name, idx, id, src, dst, meters, kmph, s
        (0, &node_b, &node_a, 1.0, 30.0, 0.12),
        (1, &node_b, &node_c, 1.0, 30.0, 0.12),
        (2, &node_c, &node_a, 1.0, 30.0, 0.12),
        (3, &node_c, &node_b, 1.0, 30.0, 0.12),
        (4, &node_d, &node_b, 1.0, 30.0, 0.12),
        (5, &node_d, &node_e, 2.0, 30.0, 0.24),
        (6, &node_d, &node_h, 1.0, 30.0, 0.12),
        (7, &node_e, &node_d, 2.0, 30.0, 0.24),
        (8, &node_e, &node_f, 1.0, 30.0, 0.12),
        (9, &node_f, &node_e, 1.0, 30.0, 0.12),
        (10, &node_f, &node_h, 1.0, 30.0, 0.12),
        (11, &node_g, &node_e, 1.0, 30.0, 0.12),
        (12, &node_g, &node_f, 1.0, 30.0, 0.12),
        (13, &node_h, &node_c, 4.0, 30.0, 0.48),
        (14, &node_h, &node_d, 1.0, 30.0, 0.12),
        (15, &node_h, &node_f, 1.0, 30.0, 0.12),
    ]
    .into_iter()
    .map(|(idx, src, dst, meters, kmph, s)| {
        // attention: fwd
        TestEdge::new_fwd(
            None,
            EdgeIdx(idx),
            src,
            dst,
            Kilometers(meters / 1_000.0),
            KilometersPerHour(kmph),
            Minutes::from(Seconds(s)),
        )
    })
    .collect();

    // Due to the offset-array, the bwd-edge-ids should match
    // with sorting by src-id, then by dst-id.
    // But the graph-structure changes that to the same as fwd-edges (dst-id, then src-id).
    let bwd_test_edges: Vec<_> = vec![
        // name, idx, id, src, dst, meters, kmph, s
        (0, &node_a, &node_b, 1.0, 30.0, 0.12),
        (1, &node_c, &node_b, 1.0, 30.0, 0.12),
        (2, &node_a, &node_c, 1.0, 30.0, 0.12),
        (3, &node_b, &node_c, 1.0, 30.0, 0.12),
        (4, &node_b, &node_d, 1.0, 30.0, 0.12),
        (5, &node_e, &node_d, 2.0, 30.0, 0.24),
        (6, &node_h, &node_d, 1.0, 30.0, 0.12),
        (7, &node_d, &node_e, 2.0, 30.0, 0.24),
        (8, &node_f, &node_e, 1.0, 30.0, 0.12),
        (9, &node_e, &node_f, 1.0, 30.0, 0.12),
        (10, &node_h, &node_f, 1.0, 30.0, 0.12),
        (11, &node_e, &node_g, 1.0, 30.0, 0.12),
        (12, &node_f, &node_g, 1.0, 30.0, 0.12),
        (13, &node_c, &node_h, 4.0, 30.0, 0.48),
        (14, &node_d, &node_h, 1.0, 30.0, 0.12),
        (15, &node_f, &node_h, 1.0, 30.0, 0.12),
    ]
    .into_iter()
    .map(|(idx, src, dst, meters, kmph, s)| {
        // attention: bwd
        TestEdge::new_bwd(
            None,
            EdgeIdx(idx),
            src,
            dst,
            Kilometers(meters / 1_000.0),
            KilometersPerHour(kmph),
            Minutes::from(Seconds(s)),
        )
    })
    .collect();

    assert_graph(test_nodes, fwd_test_edges, bwd_test_edges, &graph);
}
