use crate::configs::SimpleId;
use serde::Deserialize;
pub mod metrics;

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

impl Category {
    pub fn is_metric(&self) -> bool {
        match self {
            Category::Meta { info: _, id: _ } | Category::Ignored => false,
            Category::Metric { unit: _, id: _ } => true,
        }
    }

    pub fn is_ignored(&self) -> bool {
        match self {
            Category::Meta { info: _, id: _ } | Category::Metric { unit: _, id: _ } => false,
            Category::Ignored => true,
        }
    }
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

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    EdgeId,
}

impl From<ProtoMetaInfo> for MetaInfo {
    fn from(proto_info: ProtoMetaInfo) -> MetaInfo {
        match proto_info {
            ProtoMetaInfo::EdgeId => MetaInfo::EdgeId,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "RawConfig", deny_unknown_fields)]
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
#[serde(rename_all = "lowercase")]
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
    EdgeId,
}

impl From<RawMetaInfo> for ProtoMetaInfo {
    fn from(raw_info: RawMetaInfo) -> ProtoMetaInfo {
        match raw_info {
            RawMetaInfo::EdgeId => ProtoMetaInfo::EdgeId,
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
    EdgeId,
}
