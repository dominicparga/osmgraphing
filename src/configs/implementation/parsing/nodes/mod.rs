use crate::configs::{parsing::generating::nodes as gen, SimpleId};
use serde::Deserialize;
pub mod metrics;
pub mod proto;

#[derive(Debug)]
pub struct Config {
    pub categories: Vec<Category>,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
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
    CHLevel,
}

impl From<proto::MetaInfo> for MetaInfo {
    fn from(proto_info: proto::MetaInfo) -> MetaInfo {
        match proto_info {
            proto::MetaInfo::NodeId => MetaInfo::NodeId,
            proto::MetaInfo::Level => MetaInfo::CHLevel,
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
