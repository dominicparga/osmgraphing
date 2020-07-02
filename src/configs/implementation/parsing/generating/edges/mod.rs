use crate::configs::SimpleId;
use serde::Deserialize;
pub mod metrics;
pub mod proto;
pub mod raw;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Category {
    Meta {
        info: MetaInfo,
        id: SimpleId,
    },
    // out-of-place
    Custom {
        unit: metrics::UnitInfo,
        id: SimpleId,
        default: f64,
    },
    // out-of-place
    Haversine {
        unit: metrics::UnitInfo,
        id: SimpleId,
    },
    // out-of-place
    Copy {
        from: metrics::Category,
        to: metrics::Category,
    },
    // in-place
    Convert {
        from: metrics::Category,
        to: metrics::Category,
    },
    // out-of-place
    Calc {
        result: metrics::Category,
        a: metrics::Category,
        b: metrics::Category,
    },
}

impl From<proto::Category> for Category {
    fn from(proto_category: proto::Category) -> Category {
        match proto_category {
            proto::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            proto::Category::Custom { unit, id, default } => Category::Custom {
                unit: unit.into(),
                id,
                default,
            },
            proto::Category::Haversine { unit, id } => Category::Haversine {
                unit: unit.into(),
                id,
            },
            proto::Category::Copy { from, to } => Category::Copy {
                from: from.into(),
                to: to.into(),
            },
            proto::Category::Convert { from, to } => Category::Convert {
                from: from.into(),
                to: to.into(),
            },
            proto::Category::Calc { result, a, b } => Category::Calc {
                result: result.into(),
                a: a.into(),
                b: b.into(),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    EdgeId,
    SrcIdx,
    DstIdx,
    ShortcutIdx0,
    ShortcutIdx1,
}

impl From<proto::MetaInfo> for MetaInfo {
    fn from(proto_info: proto::MetaInfo) -> MetaInfo {
        match proto_info {
            proto::MetaInfo::EdgeId => MetaInfo::EdgeId,
            proto::MetaInfo::SrcIdx => MetaInfo::SrcIdx,
            proto::MetaInfo::DstIdx => MetaInfo::DstIdx,
            proto::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            proto::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}
