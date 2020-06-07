use crate::{
    configs::{parsing::generating, SimpleId},
    defaults::capacity::DimVec,
    helpers::err,
};
use serde::Deserialize;
pub mod metrics;
pub mod proto;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Config {
    // store all for order
    pub categories: Vec<Category>,

    // store only metrics for quick access
    pub metrics: metrics::Config,
}

impl TryFrom<proto::Config> for Config {
    type Error = err::Msg;

    fn try_from(proto_cfg: proto::Config) -> err::Result<Config> {
        // init datastructures

        let mut categories = Vec::with_capacity(proto_cfg.0.len());
        let mut metric_units = DimVec::new();
        let mut metric_ids = DimVec::new();

        // check if any id is duplicate

        for i in 0..proto_cfg.0.len() {
            // get i-th id

            let id_i = {
                match &proto_cfg.0[i] {
                    proto::Category::Ignored => continue,
                    proto::Category::Meta { info: _, id: id_i }
                    | proto::Category::Metric { unit: _, id: id_i } => id_i,
                }
            };

            for j in (i + 1)..proto_cfg.0.len() {
                // get j-th id

                let id_j = {
                    match &proto_cfg.0[j] {
                        proto::Category::Ignored => continue,
                        proto::Category::Meta { info: _, id: id_j }
                        | proto::Category::Metric { unit: _, id: id_j } => id_j,
                    }
                };

                // compare both ids

                if id_i == id_j {
                    return Err(format!("Config has duplicate id: {}", id_i).into());
                }
            }
        }

        // Fill categories, ids and create mapping: id -> idx

        for category in proto_cfg.0.into_iter() {
            // add category

            match &category {
                // add metrics separatedly
                // for better access-performance through metric-indices
                proto::Category::Metric { unit, id } => {
                    categories.push(category.clone().into());
                    metric_units.push(unit.clone().into());
                    metric_ids.push(id.clone());
                }
                proto::Category::Meta { info: _, id: _ } | proto::Category::Ignored => {
                    categories.push(category.clone().into())
                }
            }
        }

        Ok(Config {
            categories,
            metrics: metrics::Config {
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

impl From<proto::Category> for Category {
    fn from(proto_category: proto::Category) -> Category {
        match proto_category {
            proto::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            proto::Category::Metric { unit, id } => Category::Metric {
                unit: unit.into(),
                id,
            },
            proto::Category::Ignored => Category::Ignored,
        }
    }
}

/// The generating-categories specify, how a metric is generated, but it will be stored as any other parsed category, why this implementation is needed.
impl From<generating::edges::Category> for Category {
    fn from(gen_category: generating::edges::Category) -> Category {
        match gen_category {
            generating::edges::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            generating::edges::Category::Custom {
                unit,
                id,
                default: _,
            }
            | generating::edges::Category::Haversine { unit, id } => Category::Metric {
                unit: unit.into(),
                id,
            },
            generating::edges::Category::Copy { from: _, to }
            | generating::edges::Category::Convert { from: _, to }
            | generating::edges::Category::Calc {
                result: to,
                a: _,
                b: _,
            } => Category::Metric {
                unit: to.unit.into(),
                id: to.id,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    SrcId,
    SrcIdx,
    DstId,
    DstIdx,
    ShortcutIdx0,
    ShortcutIdx1,
}

impl From<proto::MetaInfo> for MetaInfo {
    fn from(proto_info: proto::MetaInfo) -> MetaInfo {
        match proto_info {
            proto::MetaInfo::SrcId => MetaInfo::SrcId,
            proto::MetaInfo::DstId => MetaInfo::DstId,
            proto::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            proto::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}

impl From<generating::edges::MetaInfo> for MetaInfo {
    fn from(gen_info: generating::edges::MetaInfo) -> MetaInfo {
        match gen_info {
            generating::edges::MetaInfo::SrcIdx => MetaInfo::SrcIdx,
            generating::edges::MetaInfo::DstIdx => MetaInfo::DstIdx,
            generating::edges::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            generating::edges::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}
