use crate::helpers::{assert_graph_sloppy, defaults, parse};
use defaults::paths::resources::isle_of_man as resources;
use osmgraphing::configs;

#[test]
fn pbf_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::OSM_PBF_YAML);
    assert!(
        configs::writing::network::graph::Config::try_from_yaml(resources::OSM_PBF_YAML).is_err()
    );
    configs::writing::routing::Config::from_yaml(resources::OSM_PBF_YAML);
    assert!(
        configs::routing::Config::try_from_yaml(resources::OSM_PBF_YAML, &parsing_cfg).is_err()
    );
}

#[test]
fn fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    assert!(configs::writing::network::graph::Config::try_from_yaml(resources::FMI_YAML).is_err());
    configs::writing::routing::Config::from_yaml(resources::FMI_YAML);
    configs::routing::Config::try_from_yaml(resources::FMI_YAML, &parsing_cfg);
}

#[test]
fn ch_fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    assert!(
        configs::writing::network::graph::Config::try_from_yaml(resources::CH_FMI_YAML).is_err()
    );
    configs::writing::routing::Config::from_yaml(resources::CH_FMI_YAML);
    assert!(configs::routing::Config::try_from_yaml(resources::CH_FMI_YAML, &parsing_cfg).is_err());
}

#[test]
fn pbf_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::OSM_PBF_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 52_803;
    let expected_edge_count = 107_031;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}

#[test]
fn fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 52_803;
    let expected_edge_count = 107_031;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}

#[test]
fn ch_fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 52_803;
    let expected_edge_count = 189_145;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}
