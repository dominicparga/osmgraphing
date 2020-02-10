use super::parse;
use osmgraphing::Parser;

mod fmi {
    #[test]
    fn simple_stuttgart() {
        let _graph = super::parse("resources/maps/simple_stuttgart.fmi");
    }

    #[test]
    fn small() {
        let _graph = super::parse("resources/maps/small.fmi");
    }
}

mod pbf {
    #[test]
    fn isle_of_man() {
        let _graph = super::parse("resources/maps/isle-of-man_2019-09-05.osm.pbf");
    }
}

#[test]
fn wrong_extension() {
    assert!(
        Parser::parse("foo.asdf").is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
