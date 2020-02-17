use super::parse;
use osmgraphing::{configs::graph, Parser};

mod fmi {
    use osmgraphing::configs::graph;

    #[test]
    fn simple_stuttgart() {
        let mut cfg = graph::Config::default();
        cfg.paths_mut()
            .set_map_file("resources/maps/simple_stuttgart.fmi");
        let _graph = super::parse(&cfg);
    }

    #[test]
    fn small() {
        let mut cfg = graph::Config::default();
        cfg.paths_mut().set_map_file("resources/maps/small.fmi");
        let _graph = super::parse(&cfg);
    }

    #[test]
    fn bait() {
        let mut cfg = graph::Config::default();
        cfg.paths_mut()
            .set_map_file("resources/maps/bidirectional_bait.fmi");
        let _graph = super::parse(&cfg);
    }
}

mod pbf {
    use osmgraphing::configs::graph;

    #[test]
    fn isle_of_man() {
        let mut cfg = graph::Config::default();
        cfg.paths_mut()
            .set_map_file("resources/maps/isle-of-man_2019-09-05.osm.pbf");
        let _graph = super::parse(&cfg);
    }
}

#[test]
fn wrong_extension() {
    let mut cfg = graph::Config::default();
    cfg.paths_mut().set_map_file("foo.asdf");
    assert!(
        Parser::parse(&cfg).is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
