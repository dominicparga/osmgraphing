use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Config {
    pub ids: Vec<Option<SimpleId>>,
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        Config { ids: proto_cfg.ids }
    }
}

#[derive(Debug)]
pub struct ProtoConfig {
    pub ids: Vec<Option<SimpleId>>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig {
            ids: raw_cfg
                .0
                .into_iter()
                .map(|category| match category {
                    RawCategory::Id(id) => Some(id),
                    RawCategory::Ignored => None,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawConfig(pub Vec<RawCategory>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawCategory {
    Id(SimpleId),
    Ignored,
}
