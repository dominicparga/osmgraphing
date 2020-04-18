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
        cfg.parsing.map_file = PathBuf::from("foo.asdf");
        assert!(
            Parser::parse(cfg.parsing).is_err(),
            "File-extension 'asdf' should not be supported."
        );
    }

    #[test]
    fn routing_config_from_str() {
        let cfg = Config::from_yaml(defaults::paths::resources::configs::SMALL_FMI).unwrap();
        configs::routing::Config::from_str(
            "routing: { metrics: [{ id: 'Meters' }, { id: 'Seconds' }] }",
            &cfg.parsing,
        )
        .expect("Routing-config is wrong.");

        let yaml_str = &format!(
            "routing: {{ metrics: [{{ id: '{}' }}], is-ch-dijkstra: true }}",
            defaults::DURATION_ID
        );
        let routing_cfg = configs::routing::Config::from_str(yaml_str, &cfg.parsing).unwrap();
        assert!(
            routing_cfg.is_ch_dijkstra,
            "Routing-config should specify ch-dijkstra."
        );

        let yaml_str = &format!(
            "routing: {{ metrics: [{{ id: '{}' }}], is-ch-dijkstra: false }}",
            defaults::DISTANCE_ID
        );
        let routing_cfg = configs::routing::Config::from_str(yaml_str, &cfg.parsing).unwrap();
        assert!(
            !routing_cfg.is_ch_dijkstra,
            "Routing-config should specify normal dijkstra."
        );
    }
}
