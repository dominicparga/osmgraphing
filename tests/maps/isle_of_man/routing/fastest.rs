use crate::helpers::TestNode;
use crate::helpers::{assert_path, create_config, defaults, TestType};
use osmgraphing::routing;
use smallvec::smallvec;

#[test]
#[ignore]
fn bidirectional() {
    let cfg = create_config(TestType::IsleOfMan);

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths();
    let preferences = routing::dijkstra::Preferences {
        alphas: smallvec![1.0],
        metric_indices: smallvec![cfg.graph.edges.metrics.idx(&defaults::DURATION_ID.into())],
    };

    assert_path(&mut dijkstra, &preferences, expected_paths, cfg.graph);
}

fn expected_paths() -> Vec<(TestNode, TestNode, Option<(f32, Vec<Vec<TestNode>>)>)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
