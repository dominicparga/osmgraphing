use crate::helpers::{defaults, parse};
use defaults::paths::resources::isle_of_man as resources;
use osmgraphing::configs;

#[test]
fn pbf_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::OSM_PBF_YAML);
    assert!(configs::writing::network::Config::try_from_yaml(resources::OSM_PBF_YAML).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(resources::OSM_PBF_YAML).is_ok());
    assert!(
        configs::routing::Config::try_from_yaml(resources::OSM_PBF_YAML, &parsing_cfg).is_err()
    );
}

#[test]
fn fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    assert!(configs::writing::network::Config::try_from_yaml(resources::FMI_YAML).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(resources::FMI_YAML).is_ok());
    assert!(configs::routing::Config::try_from_yaml(resources::FMI_YAML, &parsing_cfg).is_err());
}

#[test]
fn ch_fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    assert!(configs::writing::network::Config::try_from_yaml(resources::CH_FMI_YAML).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(resources::CH_FMI_YAML).is_ok());
    assert!(configs::routing::Config::try_from_yaml(resources::CH_FMI_YAML, &parsing_cfg).is_err());
}

#[test]
fn pbf_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::OSM_PBF_YAML);
    let graph = parse(parsing_cfg);

    let nodes = graph.nodes();
    let expected = 52_803;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let fwd_edges = graph.fwd_edges();
    let expected = 107_031;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
}

#[test]
fn fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    let graph = parse(parsing_cfg);

    let nodes = graph.nodes();
    let expected = 52_803;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let fwd_edges = graph.fwd_edges();
    let expected = 107_031;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
}

#[test]
fn ch_fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    let graph = parse(parsing_cfg);

    let nodes = graph.nodes();
    let expected = 52_803;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let fwd_edges = graph.fwd_edges();
    let bwd_edges = graph.bwd_edges();
    let expected = 189_145;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
    assert_eq!(
        bwd_edges.count(),
        expected,
        "Number of bwd-edges in graph should be {} but is {}.",
        expected,
        bwd_edges.count()
    );

    // check consistency

    for edge_idx in &fwd_edges {
        if let Some(&[sc_edge_0, sc_edge_1]) = fwd_edges.sc_edges(edge_idx) {
            let src_idx = bwd_edges.dst_idx(edge_idx);
            let dst_idx = fwd_edges.dst_idx(edge_idx);
            let src_0_idx = bwd_edges.dst_idx(sc_edge_0);
            let dst_0_idx = fwd_edges.dst_idx(sc_edge_0);
            let src_1_idx = bwd_edges.dst_idx(sc_edge_1);
            let dst_1_idx = fwd_edges.dst_idx(sc_edge_1);

            let err_msg = format!(
                "Shortcut-edge (edge-idx: {}) (node-idx: {} -> node-idx: {}) \
                doesn't match with sc-edges (node-idx: {} -> node-idx: {}) \
                and (node-idx: {} -> node-idx: {})",
                edge_idx, src_idx, dst_idx, src_0_idx, dst_0_idx, src_1_idx, dst_1_idx
            );

            assert_eq!(src_idx, src_0_idx, "{}", err_msg);
            assert_eq!(dst_0_idx, src_1_idx, "{}", err_msg);
            assert_eq!(dst_1_idx, dst_idx, "{}", err_msg);
        } else {
            assert!(
                !fwd_edges.is_shortcut(edge_idx),
                "Not every shortcut-edge is seen as shortcut-edge."
            );
        }
    }
}
