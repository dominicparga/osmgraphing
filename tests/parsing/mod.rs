use super::parse;
use osmgraphing::{
    configs::{graph, paths},
    Parser,
};
use std::path::PathBuf;

// TODO take results from actions of commit f28d88a
mod fmi {
    use osmgraphing::configs::{graph, paths};
    use std::path::PathBuf;

    #[test]
    fn simple_stuttgart() {
        let cfg = graph::Config {
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/simple_stuttgart.fmi"),
            },
            ..Default::default()
        };
        let graph = super::parse(&cfg);

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
        let cfg = graph::Config {
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/small.fmi"),
            },
            ..Default::default()
        };
        let graph = super::parse(&cfg);

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
        let cfg = graph::Config {
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/bidirectional_bait.fmi"),
            },
            ..Default::default()
        };
        let graph = super::parse(&cfg);

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
    use osmgraphing::configs::{edges, graph, paths, MetricType};
    use std::path::PathBuf;

    #[test]
    fn isle_of_man() {
        let cfg = graph::Config {
            is_graph_suitable: false,
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/isle-of-man_2019-09-05.osm.pbf"),
            },
            edges: edges::Config {
                metric_ids: vec![
                    String::from("src-id"),
                    String::from("dst-id"),
                    String::from("length"),
                    String::from("maxspeed"),
                ],
                metric_types: vec![
                    MetricType::Id,
                    MetricType::Id,
                    MetricType::Length { provided: false },
                    MetricType::Maxspeed { provided: true },
                ],
            },
            ..Default::default()
        };
        let graph = super::parse(&cfg);

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
    let cfg = graph::Config {
        paths: paths::Config {
            map_file: PathBuf::from("foo.asdf"),
        },
        ..Default::default()
    };
    assert!(
        Parser::parse(&cfg).is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
