use crate::configs::{parsing::generating::nodes as gen, SimpleId};
use serde::Deserialize;
pub mod metrics;

#[derive(Clone, Debug)]
pub struct Config {
    pub categories: Vec<Category>,
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        let mut categories: Vec<Category> = Vec::new();

        for category in proto_cfg.0.into_iter() {
            categories.push(category.into());
        }

        Config { categories }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum Category {
    Meta {
        info: MetaInfo,
        id: SimpleId,
    },
    Metric {
        unit: metrics::UnitInfo,
        id: SimpleId,
    },
    Ignored,
}

impl From<ProtoCategory> for Category {
    fn from(proto_category: ProtoCategory) -> Category {
        match proto_category {
            ProtoCategory::Meta { info, id } => Category::Meta {
                info: MetaInfo::from(info),
                id,
            },
            ProtoCategory::Metric { unit, id } => Category::Metric {
                unit: metrics::UnitInfo::from(unit),
                id,
            },
            ProtoCategory::Ignored => Category::Ignored,
        }
    }
}

impl From<gen::Category> for Category {
    fn from(gen_category: gen::Category) -> Category {
        match gen_category {
            gen::Category::Meta { info, id } => Category::Meta {
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
            ProtoMetaInfo::CHLevel => MetaInfo::CHLevel,
        }
    }
}

impl From<gen::MetaInfo> for MetaInfo {
    fn from(gen_info: gen::MetaInfo) -> MetaInfo {
        match gen_info {
            gen::MetaInfo::NodeId => MetaInfo::NodeId,
            gen::MetaInfo::NodeIdx => MetaInfo::NodeIdx,
            gen::MetaInfo::CHLevel => MetaInfo::CHLevel,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "RawConfig")]
pub struct ProtoConfig(pub Vec<ProtoCategory>);

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig(raw_cfg.0.into_iter().map(ProtoCategory::from).collect())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ProtoCategory {
    Meta {
        info: ProtoMetaInfo,
        id: SimpleId,
    },
    Metric {
        unit: metrics::ProtoUnitInfo,
        id: SimpleId,
    },
    Ignored,
}

impl From<RawCategory> for ProtoCategory {
    fn from(raw_category: RawCategory) -> ProtoCategory {
        match raw_category {
            RawCategory::Meta { info, id } => ProtoCategory::Meta {
                info: ProtoMetaInfo::from(info),
                id,
            },
            RawCategory::Metric { unit, id } => ProtoCategory::Metric {
                unit: metrics::ProtoUnitInfo::from(unit),
                id,
            },
            RawCategory::Ignored => ProtoCategory::Ignored,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ProtoMetaInfo {
    NodeId,
    CHLevel,
}

impl From<RawMetaInfo> for ProtoMetaInfo {
    fn from(raw_info: RawMetaInfo) -> ProtoMetaInfo {
        match raw_info {
            RawMetaInfo::NodeId => ProtoMetaInfo::NodeId,
            RawMetaInfo::CHLevel => ProtoMetaInfo::CHLevel,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig(pub Vec<RawCategory>);

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawCategory {
    Meta {
        info: RawMetaInfo,
        id: SimpleId,
    },
    Metric {
        unit: metrics::RawUnitInfo,
        id: SimpleId,
    },
    Ignored,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum RawMetaInfo {
    NodeId,
    CHLevel,
}
