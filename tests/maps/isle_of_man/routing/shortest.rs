use crate::helpers::{assert_path, create_config, defaults, TestNode, TestType};
use osmgraphing::routing;

#[test]
#[ignore]
fn bidirectional() {
    let cfg = create_config(
        TestType::IsleOfMan,
        Some(&format!("routing: [{{ id: '{}' }}]", defaults::LENGTH_ID)),
    );

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths();

    assert_path(&mut dijkstra, expected_paths, cfg);
}

fn expected_paths() -> Vec<(TestNode, TestNode, Option<(f32, Vec<Vec<TestNode>>)>)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
