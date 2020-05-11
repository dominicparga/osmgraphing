use crate::helpers::{defaults, parse};
use osmgraphing::configs;

const PBF_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_PBF;
const CH_FMI_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI;
const FMI_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_FMI;

#[test]
fn pbf_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(PBF_CONFIG);
    assert!(configs::writing::network::Config::try_from_yaml(PBF_CONFIG).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(PBF_CONFIG).is_ok());
    assert!(configs::routing::Config::try_from_yaml(PBF_CONFIG, &parsing_cfg).is_err());
}

#[test]
fn fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(FMI_CONFIG);
    assert!(configs::writing::network::Config::try_from_yaml(FMI_CONFIG).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(FMI_CONFIG).is_ok());
    assert!(configs::routing::Config::try_from_yaml(FMI_CONFIG, &parsing_cfg).is_err());
}

#[test]
fn ch_fmi_yaml() {
    let parsing_cfg = configs::parsing::Config::from_yaml(CH_FMI_CONFIG);
    assert!(configs::writing::network::Config::try_from_yaml(CH_FMI_CONFIG).is_err());
    assert!(configs::writing::routing::Config::try_from_yaml(CH_FMI_CONFIG).is_ok());
    assert!(configs::routing::Config::try_from_yaml(CH_FMI_CONFIG, &parsing_cfg).is_err());
}

#[test]
fn yaml_str() {
    // TODO
    pbf_yaml();
}

#[test]
fn pbf_graph() {
    let parsing_cfg = configs::parsing::Config::from_yaml(PBF_CONFIG);
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
    let parsing_cfg =
        configs::parsing::Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_FMI);
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
    let parsing_cfg = configs::parsing::Config::from_yaml(
        defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI,
    );
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
