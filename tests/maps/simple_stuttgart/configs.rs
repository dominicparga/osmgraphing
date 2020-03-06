mod fmi {
    use osmgraphing::configs::Config;

    #[test]
    pub fn deserialize() {
        Config::from_yaml("resources/configs/simple-stuttgart.fmi.yaml").unwrap();
    }
}
