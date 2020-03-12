use crate::helpers::{defaults, parse};
use osmgraphing::configs::{self, Config};

#[test]
fn pbf_yaml() {
    Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();
}

#[test]
fn fmi_yaml() {
    Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_FMI).unwrap();
}

#[test]
fn ch_fmi_yaml() {
    Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI).unwrap();
}

#[test]
fn yaml_str() {
    let cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::DURATION_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::LENGTH_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.parser).unwrap();
}

#[test]
fn pbf_graph() {
    let cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();
    let graph = parse(cfg.parser);

    let nodes = graph.nodes();
    let expected = 51_310;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let fwd_edges = graph.fwd_edges();
    // let expected = 103_920; // before removing duplicates
    let expected = 103_916; // after removing duplicates
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
    let cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_FMI).unwrap();
    let graph = parse(cfg.parser);

    let nodes = graph.nodes();
    let expected = 51_310;
    assert_eq!(
        nodes.count(),
        expected,
        "Number of nodes in graph should be {} but is {}.",
        expected,
        nodes.count()
    );
    let fwd_edges = graph.fwd_edges();
    // let expected = 103_920; // before removing duplicates
    let expected = 103_916; // after removing duplicates
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
    let cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI).unwrap();
    let _graph = parse(cfg.parser);
}
