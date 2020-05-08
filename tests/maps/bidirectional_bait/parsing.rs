use crate::helpers::{assert_graph, defaults, parse, TestEdge, TestNode};
use kissunits::{
    distance::{Kilometers, Meters},
    geo::Coordinate,
    speed::KilometersPerHour,
    time::{Minutes, Seconds},
};
use osmgraphing::{configs, network::EdgeIdx};

const FMI_CONFIG: &str = defaults::paths::resources::configs::BIDIRECTIONAL_BAIT_FMI;

#[test]
fn fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(FMI_CONFIG);
    assert!(configs::writing::network::Config::try_from_yaml(FMI_CONFIG).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(FMI_CONFIG).is_err());
    assert!(configs::routing::Config::try_from_yaml(FMI_CONFIG, &parsing_cfg).is_err());
}

#[test]
fn yaml_str() {
    // TODO
    fmi_yaml();
}

#[test]
fn fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(FMI_CONFIG);
    let graph = parse(parsing_cfg);

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let test_nodes: Vec<_> = vec![
        ("left", 0, Coordinate::zero()),
        ("bottom", 1, Coordinate::zero()),
        ("right", 2, Coordinate::zero()),
        ("top-right", 3, Coordinate::zero()),
        ("top-left", 4, Coordinate::zero()),
    ]
    .into_iter()
    .map(|(name, id, coord)| TestNode::new(name, id, coord, 0, &graph))
    .collect();
    let node_ll = &test_nodes[0];
    let node_bb = &test_nodes[1];
    let node_rr = &test_nodes[2];
    let node_tr = &test_nodes[3];
    let node_tl = &test_nodes[4];

    // Due to the offset-array, the fwd-edge-ids should match
    // with sorting by src-id, then by dst-id.
    let fwd_test_edges: Vec<_> = vec![
        // name, idx, id, src, dst, meters, kmph, s
        (0, &node_ll, &node_bb, 5.0, 30.0, 0.60),
        (1, &node_ll, &node_tl, 3.0, 30.0, 0.36),
        (2, &node_bb, &node_ll, 5.0, 30.0, 0.60),
        (3, &node_bb, &node_rr, 5.0, 30.0, 0.60),
        (4, &node_rr, &node_bb, 5.0, 30.0, 0.60),
        (5, &node_rr, &node_tr, 3.0, 30.0, 0.36),
        (6, &node_tr, &node_rr, 3.0, 30.0, 0.36),
        (7, &node_tr, &node_tl, 3.0, 30.0, 0.36),
        (8, &node_tl, &node_ll, 3.0, 30.0, 0.36),
        (9, &node_tl, &node_tr, 3.0, 30.0, 0.36),
    ]
    .into_iter()
    .map(|(idx, src, dst, meters, kmph, s)| {
        // attention: fwd
        TestEdge::new_fwd(
            None,
            EdgeIdx(idx),
            src,
            dst,
            Kilometers::from(Meters(meters)),
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
        (0, &node_bb, &node_ll, 5.0, 30.0, 0.60),
        (1, &node_tl, &node_ll, 3.0, 30.0, 0.36),
        (2, &node_ll, &node_bb, 5.0, 30.0, 0.60),
        (3, &node_rr, &node_bb, 5.0, 30.0, 0.60),
        (4, &node_bb, &node_rr, 5.0, 30.0, 0.60),
        (5, &node_tr, &node_rr, 3.0, 30.0, 0.36),
        (6, &node_rr, &node_tr, 3.0, 30.0, 0.36),
        (7, &node_tl, &node_tr, 3.0, 30.0, 0.36),
        (8, &node_ll, &node_tl, 3.0, 30.0, 0.36),
        (9, &node_tr, &node_tl, 3.0, 30.0, 0.36),
    ]
    .into_iter()
    .map(|(idx, src, dst, meters, kmph, s)| {
        // attention: bwd
        TestEdge::new_bwd(
            None,
            EdgeIdx(idx),
            src,
            dst,
            Kilometers::from(Meters(meters)),
            KilometersPerHour(kmph),
            Minutes::from(Seconds(s)),
        )
    })
    .collect();

    assert_graph(test_nodes, fwd_test_edges, bwd_test_edges, &graph);
}
