use crate::helpers::{assert_graph_sloppy, defaults, parse};
use defaults::paths::resources::saarland as resources;
use osmgraphing::configs;

#[test]
fn pbf_yaml() {
    let _parsing_cfg = configs::parsing::Config::from_yaml(resources::OSM_PBF_YAML);
    assert!(
        configs::writing::network::graph::Config::try_from_yaml(resources::OSM_PBF_YAML).is_err()
    );
    assert!(configs::writing::routing::Config::try_from_yaml(resources::OSM_PBF_YAML).is_err());
    // Fails, but should work after building will have been generated the distance.
    // configs::routing::Config::from_yaml(resources::OSM_PBF_YAML, &parsing_cfg);
}

#[test]
fn ch_fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    assert!(
        configs::writing::network::graph::Config::try_from_yaml(resources::CH_FMI_YAML).is_err()
    );
    configs::writing::routing::Config::from_yaml(resources::CH_FMI_YAML);
    configs::routing::Config::from_yaml(resources::CH_FMI_YAML, &parsing_cfg);
}

#[test]
fn pbf_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::OSM_PBF_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 255_271;
    let expected_edge_count = 493_346;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}

#[test]
fn ch_fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 255_271;
    let expected_edge_count = 893_847;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}
