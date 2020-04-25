pub mod parsing {
    use crate::helpers::defaults;
    use osmgraphing::{configs, io::Parser};
    use std::path::PathBuf;

    #[test]
    fn wrong_extension() {
        let mut parsing_cfg =
            configs::parsing::Config::from_yaml(defaults::paths::resources::configs::SMALL_FMI);
        parsing_cfg.map_file = PathBuf::from("foo.asdf");
        assert!(
            Parser::parse(parsing_cfg).is_err(),
            "File-extension 'asdf' should not be supported."
        );
    }

    #[test]
    fn routing_config_from_str() {
        let parsing_cfg =
            configs::parsing::Config::from_yaml(defaults::paths::resources::configs::SMALL_FMI);
        let yaml_str = &format!(
            "routing: {{ metrics: [{{ id: '{}' }}, {{ id: '{}' }}] }}",
            defaults::SPEED_ID,
            defaults::SPEED_ID
        );
        configs::routing::Config::from_str(yaml_str, &parsing_cfg);

        let yaml_str = &format!(
            "routing: {{ metrics: [{{ id: '{}' }}], is-ch-dijkstra: true }}",
            defaults::SPEED_ID
        );
        let routing_cfg = configs::routing::Config::from_str(yaml_str, &parsing_cfg);
        assert!(
            routing_cfg.is_ch_dijkstra,
            "Routing-config should specify ch-dijkstra."
        );

        let yaml_str = &format!(
            "routing: {{ metrics: [{{ id: '{}' }}], is-ch-dijkstra: false }}",
            defaults::SPEED_ID
        );
        let routing_cfg = configs::routing::Config::from_str(yaml_str, &parsing_cfg);
        assert!(
            !routing_cfg.is_ch_dijkstra,
            "Routing-config should specify normal dijkstra."
        );
    }
}
