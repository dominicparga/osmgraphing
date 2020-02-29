// https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust
// https://serde.rs/attributes.html
// https://serde.rs/container-attrs.html
// https://serde.rs/variant-attrs.html
// https://serde.rs/field-attrs.html

// https://serde.rs/enum-representations.html
// https://docs.rs/serde_yaml/0.8.11/serde_yaml/

pub mod fmi {
    use osmgraphing::configs::Config;

    pub fn deserialize() -> Result<Config, String> {
        Config::from_path("resources/configs/fmi.yaml")
    }
}

pub mod pbf {
    use osmgraphing::configs::Config;

    pub fn deserialize() -> Result<Config, String> {
        Config::from_path("resources/configs/pbf.yaml")
    }
}
