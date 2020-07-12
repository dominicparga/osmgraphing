use crate::{configs::SimpleId, defaults};
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Config {
    pub is_writing_shortcuts: bool,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        Config {
            is_writing_shortcuts: proto_cfg
                .is_writing_shortcuts
                .unwrap_or(defaults::parsing::IS_USING_SHORTCUTS),
            ids: proto_cfg.ids,
        }
    }
}

#[derive(Debug)]
pub struct ProtoConfig {
    pub is_writing_shortcuts: Option<bool>,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig {
            is_writing_shortcuts: raw_cfg.is_writing_shortcuts,
            ids: raw_cfg
                .ids
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
pub struct RawConfig {
    #[serde(rename = "with_shortcuts")]
    pub is_writing_shortcuts: Option<bool>,
    pub ids: Vec<RawCategory>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawCategory {
    Id(SimpleId),
    Ignored,
}
