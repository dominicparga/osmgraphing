use crate::{
    configs::{parsing::generating, SimpleId},
    defaults::capacity::DimVec,
};
use serde::Deserialize;
pub mod metrics;
pub mod raw;

#[derive(Debug)]
pub struct Config {
    // store all for order
    pub categories: Vec<Category>,

    // store only metrics for quick access
    pub metrics: metrics::Config,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        // init datastructures

        let mut categories = Vec::with_capacity(raw_cfg.0.len());
        let mut metric_units = DimVec::new();
        let mut metric_ids = DimVec::new();

        // check if any id is duplicate

        for i in 0..raw_cfg.0.len() {
            // get i-th id

            let id_i = {
                match &raw_cfg.0[i] {
                    raw::Category::Ignored => continue,
                    raw::Category::Meta { info: _, id: id_i }
                    | raw::Category::Metric { unit: _, id: id_i } => id_i,
                }
            };

            for j in (i + 1)..raw_cfg.0.len() {
                // get j-th id

                let id_j = {
                    match &raw_cfg.0[j] {
                        raw::Category::Ignored => continue,
                        raw::Category::Meta { info: _, id: id_j }
                        | raw::Category::Metric { unit: _, id: id_j } => id_j,
                    }
                };

                // compare both ids

                if id_i == id_j {
                    panic!("Config has duplicate id: {}", id_i);
                }
            }
        }

        // Fill categories, ids and create mapping: id -> idx

        for category in raw_cfg.0.into_iter() {
            // add category

            match &category {
                // add metrics separatedly
                // for better access-performance through metric-indices
                raw::Category::Metric { unit, id } => {
                    categories.push(category.clone().into());
                    metric_units.push(unit.clone().into());
                    metric_ids.push(id.clone());
                }
                raw::Category::Meta { info: _, id: _ } | raw::Category::Ignored => {
                    categories.push(category.clone().into())
                }
            }
        }

        Config {
            categories,
            metrics: metrics::Config {
                units: metric_units,
                ids: metric_ids,
            },
        }
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

impl From<raw::Category> for Category {
    fn from(raw_category: raw::Category) -> Category {
        match raw_category {
            raw::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            raw::Category::Metric { unit, id } => Category::Metric {
                unit: unit.into(),
                id,
            },
            raw::Category::Ignored => Category::Ignored,
        }
    }
}

impl From<generating::edges::Category> for Category {
    fn from(gen_category: generating::edges::Category) -> Category {
        match gen_category {
            generating::edges::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            generating::edges::Category::Haversine { unit, id } => Category::Metric {
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

impl From<raw::MetaInfo> for MetaInfo {
    fn from(raw_info: raw::MetaInfo) -> MetaInfo {
        match raw_info {
            raw::MetaInfo::SrcId => MetaInfo::SrcId,
            raw::MetaInfo::DstId => MetaInfo::DstId,
            raw::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            raw::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
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
