use serde::Deserialize;
pub mod edges;
pub mod nodes;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub nodes: nodes::Config,
    pub edges: edges::Config,
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        Config {
            nodes: nodes::Config {
                categories: proto_cfg
                    .nodes
                    .categories
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProtoConfig {
    pub nodes: nodes::ProtoConfig,
    pub edges: edges::ProtoConfig,
}
