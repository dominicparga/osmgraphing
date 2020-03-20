use crate::helpers::{assert_path, defaults, TestNode};
use osmgraphing::{
    configs::{self, Config},
    routing,
};

#[test]
#[ignore]
fn chdijkstra() {
    let mut cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}] }}",
            defaults::LENGTH_ID
        ),
        &cfg.parser,
    )
    .ok();

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths();

    assert_path(&mut dijkstra, expected_paths, cfg);
}

#[test]
#[ignore]
fn dijkstra() {
    let mut cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}] }}",
            defaults::LENGTH_ID
        ),
        &cfg.parser,
    )
    .ok();

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths();

    assert_path(&mut dijkstra, expected_paths, cfg);
}

fn expected_paths() -> Vec<(TestNode, TestNode, Option<(f64, Vec<Vec<TestNode>>)>)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
