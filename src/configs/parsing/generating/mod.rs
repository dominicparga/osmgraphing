use serde::Deserialize;
pub mod edges;
pub mod nodes;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ProtoConfig {
    pub nodes: nodes::ProtoConfig,
    pub edges: edges::ProtoConfig,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig {
            nodes: nodes::ProtoConfig::from(raw_cfg.nodes),
            edges: edges::ProtoConfig::from(raw_cfg.edges),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    pub nodes: nodes::RawConfig,
    pub edges: edges::RawConfig,
}
