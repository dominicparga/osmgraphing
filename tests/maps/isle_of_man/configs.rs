mod pbf {
    use osmgraphing::configs::Config;

    #[test]
    pub fn deserialize() {
        Config::from_yaml("resources/configs/isle-of-man.pbf.yaml").unwrap();
    }
}
