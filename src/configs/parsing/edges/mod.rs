use crate::{
    configs::{parsing::generating, SimpleId},
    defaults::{self, capacity::DimVec},
    helpers::err,
};
use serde::Deserialize;
pub mod metrics;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct Config {
    // store all for order
    pub categories: Vec<Category>,

    // store only metrics for quick access
    pub metrics: metrics::Config,
}

impl TryFrom<ProtoConfig> for Config {
    type Error = err::Msg;

    fn try_from(proto_cfg: ProtoConfig) -> err::Result<Config> {
        // init datastructures

        let mut categories = Vec::with_capacity(proto_cfg.categories.len());
        let mut metric_units = DimVec::new();
        let mut metric_ids = DimVec::new();

        // check if any id is duplicate

        for i in 0..proto_cfg.categories.len() {
            // get i-th id

            let id_i = {
                match &proto_cfg.categories[i] {
                    ProtoCategory::Ignored => continue,
                    ProtoCategory::Meta { info: _, id: id_i }
                    | ProtoCategory::Metric { unit: _, id: id_i } => id_i,
                }
            };

            for j in (i + 1)..proto_cfg.categories.len() {
                // get j-th id

                let id_j = {
                    match &proto_cfg.categories[j] {
                        ProtoCategory::Ignored => continue,
                        ProtoCategory::Meta { info: _, id: id_j }
                        | ProtoCategory::Metric { unit: _, id: id_j } => id_j,
                    }
                };

                // compare both ids

                if id_i == id_j {
                    return Err(format!("Config has duplicate id: {}", id_i).into());
                }
            }
        }

        // Fill categories, ids and create mapping: id -> idx

        for category in proto_cfg.categories.into_iter() {
            // add category

            match &category {
                // add metrics separatedly
                // for better access-performance through metric-indices
                ProtoCategory::Metric { unit, id } => {
                    categories.push(category.clone().into());
                    metric_units.push(unit.clone().into());
                    metric_ids.push(id.clone());
                }
                ProtoCategory::Meta { info: _, id: _ } | ProtoCategory::Ignored => {
                    categories.push(category.clone().into())
                }
            }
        }

        Ok(Config {
            categories,
            metrics: metrics::Config {
                are_normalized: proto_cfg
                    .are_metrics_normalized
                    .unwrap_or(defaults::parsing::WILL_NORMALIZE_METRICS_BY_MEAN),
                units: metric_units,
                ids: metric_ids,
            },
        })
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
    SrcId,
    SrcIdx,
    SrcLat,
    SrcLon,
    DstId,
    DstIdx,
    DstLat,
    DstLon,
    ShortcutIdx0,
    ShortcutIdx1,
}

impl From<ProtoMetaInfo> for MetaInfo {
    fn from(proto_info: ProtoMetaInfo) -> MetaInfo {
        match proto_info {
            ProtoMetaInfo::EdgeId => MetaInfo::EdgeId,
            ProtoMetaInfo::SrcId => MetaInfo::SrcId,
            ProtoMetaInfo::DstId => MetaInfo::DstId,
            ProtoMetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            ProtoMetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}

impl From<generating::edges::MetaInfo> for MetaInfo {
    fn from(gen_info: generating::edges::MetaInfo) -> MetaInfo {
        match gen_info {
            generating::edges::MetaInfo::EdgeId => MetaInfo::EdgeId,
            generating::edges::MetaInfo::SrcIdx => MetaInfo::SrcIdx,
            generating::edges::MetaInfo::SrcLat => MetaInfo::SrcLat,
            generating::edges::MetaInfo::SrcLon => MetaInfo::SrcLon,
            generating::edges::MetaInfo::DstIdx => MetaInfo::DstIdx,
            generating::edges::MetaInfo::DstLat => MetaInfo::DstLat,
            generating::edges::MetaInfo::DstLon => MetaInfo::DstLon,
            generating::edges::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            generating::edges::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawConfig", deny_unknown_fields)]
pub struct ProtoConfig {
    pub are_metrics_normalized: Option<bool>,
    pub categories: Vec<ProtoCategory>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig {
            are_metrics_normalized: raw_cfg.are_metrics_normalized,
            categories: raw_cfg.data.into_iter().map(ProtoCategory::from).collect(),
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
    SrcId,
    DstId,
    ShortcutIdx0,
    ShortcutIdx1,
}

impl From<RawMetaInfo> for ProtoMetaInfo {
    fn from(raw_info: RawMetaInfo) -> ProtoMetaInfo {
        match raw_info {
            RawMetaInfo::EdgeId => ProtoMetaInfo::EdgeId,
            RawMetaInfo::SrcId => ProtoMetaInfo::SrcId,
            RawMetaInfo::DstId => ProtoMetaInfo::DstId,
            RawMetaInfo::ShortcutIdx0 => ProtoMetaInfo::ShortcutIdx0,
            RawMetaInfo::ShortcutIdx1 => ProtoMetaInfo::ShortcutIdx1,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    #[serde(rename = "will_normalize_metrics_by_mean")]
    are_metrics_normalized: Option<bool>,
    data: Vec<RawCategory>,
}

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
    SrcId,
    DstId,
    ShortcutIdx0,
    ShortcutIdx1,
}
