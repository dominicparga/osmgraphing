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
    Meta {
        info: MetaInfo,
        id: SimpleId,
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

impl From<raw::Category> for Category {
    fn from(raw_category: raw::Category) -> Category {
        match raw_category {
            raw::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            raw::Category::Haversine { unit, id } => Category::Haversine {
                unit: unit.into(),
                id,
            },
            raw::Category::Copy { from, to } => Category::Copy {
                from: from.into(),
                to: to.into(),
            },
            raw::Category::Convert { from, to } => Category::Convert {
                from: from.into(),
                to: to.into(),
            },
            raw::Category::Calc { result, a, b } => Category::Calc {
                result: result.into(),
                a: a.into(),
                b: b.into(),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    SrcIdx,
    DstIdx,
    ShortcutIdx0,
    ShortcutIdx1,
}

impl From<raw::MetaInfo> for MetaInfo {
    fn from(raw_info: raw::MetaInfo) -> MetaInfo {
        match raw_info {
            raw::MetaInfo::SrcIdx => MetaInfo::SrcIdx,
            raw::MetaInfo::DstIdx => MetaInfo::DstIdx,
            raw::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            raw::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}
