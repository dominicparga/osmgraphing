use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub is_ch_dijkstra: Option<bool>,
    pub metrics: Vec<Entry>,
}

impl Config {
    pub fn from_str(yaml_str: &str) -> Result<Config, String> {
        #[derive(Deserialize)]
        struct Wrapper {
            routing: Config,
        }

        let wrapper: Result<Wrapper, _> = serde_yaml::from_str(yaml_str);
        match wrapper {
            Ok(wrapper) => Ok(wrapper.routing),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub id: SimpleId,
    pub alpha: Option<f64>,
}
