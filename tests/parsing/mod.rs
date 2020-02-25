use super::{create_config, parse, TestType};
use osmgraphing::Parser;
use std::path::PathBuf;

// TODO take results from actions of commit f28d88a
mod fmi {
    use super::super::{create_config, TestType};

    #[test]
    fn simple_stuttgart() {
        let cfg = create_config(TestType::SimpleStuttgart);
        let graph = super::parse(cfg.graph);

        let nodes = graph.nodes();
        let expected = 6;
        assert_eq!(
            nodes.count(),
            expected,
            "Number of nodes in graph should be {} but is {}.",
            expected,
            nodes.count()
        );
        let fwd_edges = graph.fwd_edges();
        let expected = 13;
        assert_eq!(
            fwd_edges.count(),
            expected,
            "Number of fwd-edges in graph should be {} but is {}.",
            expected,
            fwd_edges.count()
        );
    }

    #[test]
    fn small() {
        let cfg = create_config(TestType::Small);
        let graph = super::parse(cfg.graph);

        let nodes = graph.nodes();
        let expected = 8;
        assert_eq!(
            nodes.count(),
            expected,
            "Number of nodes in graph should be {} but is {}.",
            expected,
            nodes.count()
        );
        let fwd_edges = graph.fwd_edges();
        let expected = 16;
        assert_eq!(
            fwd_edges.count(),
            expected,
            "Number of fwd-edges in graph should be {} but is {}.",
            expected,
            fwd_edges.count()
        );
    }

    #[test]
    fn bait() {
        let cfg = create_config(TestType::BidirectionalBait);
        let graph = super::parse(cfg.graph);

        let nodes = graph.nodes();
        let expected = 5;
        assert_eq!(
            nodes.count(),
            expected,
            "Number of nodes in graph should be {} but is {}.",
            expected,
            nodes.count()
        );
        let fwd_edges = graph.fwd_edges();
        let expected = 10;
        assert_eq!(
            fwd_edges.count(),
            expected,
            "Number of fwd-edges in graph should be {} but is {}.",
            expected,
            fwd_edges.count()
        );
    }
}

mod pbf {
    use super::super::{create_config, TestType};

    #[test]
    fn isle_of_man() {
        let cfg = create_config(TestType::IsleOfMan);
        let graph = super::parse(cfg.graph);

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
        let expected = 103_920;
        assert_eq!(
            fwd_edges.count(),
            expected,
            "Number of fwd-edges in graph should be {} but is {}.",
            expected,
            fwd_edges.count()
        );
    }
}

#[test]
fn wrong_extension() {
    let mut cfg = create_config(TestType::Small);
    cfg.graph.map_file = PathBuf::from("foo.asdf");
    assert!(
        Parser::parse(&cfg.graph).is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
