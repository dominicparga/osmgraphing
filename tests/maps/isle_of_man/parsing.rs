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
fn yaml_str() {
    // TODO
    pbf_yaml();
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
    let expected = 183_366;
    assert_eq!(
        fwd_edges.count(),
        expected,
        "Number of fwd-edges in graph should be {} but is {}.",
        expected,
        fwd_edges.count()
    );
}
