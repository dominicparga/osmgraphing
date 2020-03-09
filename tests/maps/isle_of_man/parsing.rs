use crate::helpers::{create_config, defaults, parse, TestType};
use osmgraphing::configs::{self, Config};

#[test]
fn yaml() {
    Config::from_yaml("resources/configs/isle-of-man.pbf.yaml").unwrap();
}

#[test]
fn yaml_str() {
    let cfg = Config::from_yaml("resources/configs/isle-of-man.pbf.yaml").unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::DURATION_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.graph).unwrap();

    let yaml_str = &format!("routing: [{{ id: '{}' }}]", defaults::LENGTH_ID);
    configs::routing::Config::from_str(yaml_str, &cfg.graph).unwrap();
}

#[test]
fn pbf() {
    let cfg = create_config(TestType::IsleOfMan, None);
    let graph = parse(cfg.graph);

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
