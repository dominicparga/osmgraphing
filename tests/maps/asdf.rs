pub mod parsing {
    use crate::helpers::{create_config, TestType};
    use osmgraphing::{configs, io::Parser};
    use std::path::PathBuf;

    #[test]
    fn wrong_extension() {
        let mut cfg = create_config(TestType::Small, None);
        cfg.parser.map_file = PathBuf::from("foo.asdf");
        assert!(
            Parser::parse(&cfg.parser).is_err(),
            "File-extension 'asdf' should not be supported."
        );
    }

    #[test]
    fn routing_config_from_str() {
        let cfg = create_config(TestType::Small, None);
        configs::routing::Config::from_str(
            "routing: [{ id: 'Meters' }, { id: 'Seconds' }]",
            &cfg.parser,
        )
        .expect("MetricIds should be provided");
    }
}
