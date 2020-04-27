use serde::Deserialize;
pub mod edges;
pub mod nodes;
pub mod raw;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub nodes: nodes::Config,
    pub edges: edges::Config,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        Config {
            nodes: nodes::Config {
                categories: raw_cfg
                    .nodes
                    .0
                    .into_iter()
                    .map(|raw_category| raw_category.into())
                    .collect(),
            },
            edges: edges::Config {
                categories: raw_cfg
                    .edges
                    .0
                    .into_iter()
                    .map(|raw_category| raw_category.into())
                    .collect(),
            },
        }
    }
}
