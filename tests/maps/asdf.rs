pub mod parsing {
    use crate::helpers::defaults;
    use osmgraphing::{
        configs::{self, Config},
        io::Parser,
    };
    use std::path::PathBuf;

    #[test]
    fn wrong_extension() {
        let mut cfg = Config::from_yaml(defaults::paths::resources::configs::SMALL_FMI).unwrap();
        cfg.parser.map_file = PathBuf::from("foo.asdf");
        assert!(
            Parser::parse(cfg.parser).is_err(),
            "File-extension 'asdf' should not be supported."
        );
    }

    #[test]
    fn routing_config_from_str() {
        let cfg = Config::from_yaml(defaults::paths::resources::configs::SMALL_FMI).unwrap();
        configs::routing::Config::from_str(
            "routing: [{ id: 'Meters' }, { id: 'Seconds' }]",
            &cfg.parser,
        )
        .expect("MetricIds should be provided");
    }
}
