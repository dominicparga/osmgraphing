use crate::helpers::TestNode;

mod astar {
    use super::expected_paths;
    use crate::helpers::{assert_path, create_config, defaults, TestType};
    use osmgraphing::routing;

    #[test]
    #[ignore]
    fn bidirectional() {
        let cfg = create_config(TestType::IsleOfMan);

        let mut astar = routing::factory::astar::shortest::bidirectional(
            cfg.graph.edges.metrics.idx(&defaults::LENGTH_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut astar, expected_paths, cfg.graph);
    }

    #[test]
    #[ignore]
    pub fn unidirectional() {
        let cfg = create_config(TestType::IsleOfMan);

        let mut astar = routing::factory::astar::shortest::unidirectional(
            cfg.graph.edges.metrics.idx(&defaults::LENGTH_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut astar, expected_paths, cfg.graph);
    }
}

mod dijkstra {
    use super::expected_paths;
    use crate::helpers::{assert_path, create_config, defaults, TestType};
    use osmgraphing::routing;

    #[test]
    #[ignore]
    fn bidirectional() {
        let cfg = create_config(TestType::IsleOfMan);

        let mut dijkstra = routing::factory::dijkstra::bidirectional(
            cfg.graph.edges.metrics.idx(&defaults::LENGTH_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut dijkstra, expected_paths, cfg.graph);
    }

    #[test]
    #[ignore]
    pub fn unidirectional() {
        let cfg = create_config(TestType::IsleOfMan);

        let mut dijkstra = routing::factory::dijkstra::unidirectional(
            cfg.graph.edges.metrics.idx(&defaults::LENGTH_ID.into()),
        );
        let expected_paths = expected_paths();

        assert_path(&mut dijkstra, expected_paths, cfg.graph);
    }
}

fn expected_paths() -> Vec<(TestNode, TestNode, Option<(f32, Vec<Vec<TestNode>>)>)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
