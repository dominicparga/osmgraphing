use crate::configs::SimpleId;
use serde::Deserialize;
pub mod metrics;
pub mod raw;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Category {
    Meta { info: MetaInfo, id: SimpleId },
}

impl From<raw::Category> for Category {
    fn from(raw_category: raw::Category) -> Category {
        match raw_category {
            raw::Category::Meta { info, id } => Category::Meta {
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
            raw::MetaInfo::NodeIdx => MetaInfo::NodeIdx,
        }
    }
}
