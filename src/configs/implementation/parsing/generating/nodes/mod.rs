use crate::configs::SimpleId;
use serde::Deserialize;
pub mod metrics;
pub mod proto;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Category {
    Meta { info: MetaInfo, id: SimpleId },
}

impl From<proto::Category> for Category {
    fn from(proto_category: proto::Category) -> Category {
        match proto_category {
            proto::Category::Meta { info, id } => Category::Meta {
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
            proto::MetaInfo::NodeIdx => MetaInfo::NodeIdx,
            proto::MetaInfo::CHLevel => MetaInfo::CHLevel,
        }
    }
}
