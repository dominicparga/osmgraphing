use crate::helpers::{assert_graph_sloppy, defaults, parse};
use defaults::paths::resources::stuttgart_regbez as resources;
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

    let expected_node_count = 2_688_220;
    let expected_edge_count = 5_592_415;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}

#[test]
fn fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::FMI_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 2_688_220;
    let expected_edge_count = 5_592_415;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}

#[test]
fn ch_fmi_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(resources::CH_FMI_YAML);
    let graph = parse(parsing_cfg);

    let expected_node_count = 2_688_220;
    let expected_edge_count = 13_327_936;
    assert_graph_sloppy(expected_node_count, expected_edge_count, &graph);
}
