use crate::configs::SimpleId;
use serde::Deserialize;
pub mod metrics;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Category {
    Meta { info: MetaInfo, id: SimpleId },
}

impl From<ProtoCategory> for Category {
    fn from(proto_category: ProtoCategory) -> Category {
        match proto_category {
            ProtoCategory::Meta { info, id } => Category::Meta {
                info: MetaInfo::from(info),
                id,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    NodeId,
    NodeIdx,
    CHLevel,
}

impl From<ProtoMetaInfo> for MetaInfo {
    fn from(proto_info: ProtoMetaInfo) -> MetaInfo {
        match proto_info {
            ProtoMetaInfo::NodeId => MetaInfo::NodeId,
            ProtoMetaInfo::NodeIdx => MetaInfo::NodeIdx,
            ProtoMetaInfo::CHLevel => MetaInfo::CHLevel,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawConfig")]
pub struct ProtoConfig {
    pub categories: Vec<ProtoCategory>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig {
            categories: raw_cfg.0.into_iter().map(ProtoCategory::from).collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ProtoCategory {
    Meta { info: ProtoMetaInfo, id: SimpleId },
}

impl From<RawCategory> for ProtoCategory {
    fn from(raw_category: RawCategory) -> ProtoCategory {
        match raw_category {
            RawCategory::Meta { info, id } => ProtoCategory::Meta {
                info: ProtoMetaInfo::from(info),
                id,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ProtoMetaInfo {
    NodeId,
    NodeIdx,
    CHLevel,
}

impl From<RawMetaInfo> for ProtoMetaInfo {
    fn from(raw_info: RawMetaInfo) -> ProtoMetaInfo {
        match raw_info {
            RawMetaInfo::NodeIdx => ProtoMetaInfo::NodeIdx,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig(pub Vec<RawCategory>);

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawCategory {
    Meta { info: RawMetaInfo, id: SimpleId },
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum RawMetaInfo {
    NodeIdx,
}
