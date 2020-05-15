use serde::Deserialize;
pub mod edges;
pub mod nodes;
pub mod proto;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub nodes: nodes::Config,
    pub edges: edges::Config,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
        Config {
            nodes: nodes::Config {
                categories: proto_cfg
                    .nodes
                    .0
                    .into_iter()
                    .map(|proto_category| proto_category.into())
                    .collect(),
            },
            edges: edges::Config {
                categories: proto_cfg
                    .edges
                    .0
                    .into_iter()
                    .map(|proto_category| proto_category.into())
                    .collect(),
            },
        }
    }
}
