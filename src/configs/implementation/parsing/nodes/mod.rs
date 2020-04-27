use crate::configs::{parsing::generating::nodes as gen, SimpleId};
use serde::Deserialize;
pub mod metrics;
pub mod raw;

#[derive(Debug)]
pub struct Config {
    pub categories: Vec<Category>,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        let mut categories: Vec<Category> = Vec::new();

        for category in raw_cfg.0.into_iter() {
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

impl From<gen::Category> for Category {
    fn from(gen_category: gen::Category) -> Category {
        match gen_category {
            gen::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    NodeId,
    NodeIdx,
    Level,
}

impl From<raw::MetaInfo> for MetaInfo {
    fn from(raw_info: raw::MetaInfo) -> MetaInfo {
        match raw_info {
            raw::MetaInfo::NodeId => MetaInfo::NodeId,
            raw::MetaInfo::Level => MetaInfo::Level,
        }
    }
}

impl From<gen::MetaInfo> for MetaInfo {
    fn from(gen_info: gen::MetaInfo) -> MetaInfo {
        match gen_info {
            gen::MetaInfo::NodeId => MetaInfo::NodeId,
            gen::MetaInfo::NodeIdx => MetaInfo::NodeIdx,
            gen::MetaInfo::Level => MetaInfo::Level,
        }
    }
}
