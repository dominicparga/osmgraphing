use crate::configs::{
    implementation::parsing::generating::edges::{metrics, raw},
    SimpleId,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, from = "raw::Config")]
pub struct Config(pub Vec<Category>);

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        Config(raw_cfg.0.into_iter().map(Category::from).collect())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Meta {
        info: MetaInfo,
        id: SimpleId,
    },
    Custom {
        unit: metrics::proto::UnitInfo,
        id: SimpleId,
        default: f64,
    },
    Haversine {
        unit: metrics::proto::UnitInfo,
        id: SimpleId,
    },
    Copy {
        from: metrics::proto::Category,
        to: metrics::proto::Category,
    },
    Convert {
        from: metrics::proto::Category,
        to: metrics::proto::Category,
    },
    Calc {
        result: metrics::proto::Category,
        a: metrics::proto::Category,
        b: metrics::proto::Category,
    },
}

impl From<raw::Category> for Category {
    fn from(raw_category: raw::Category) -> Category {
        match raw_category {
            raw::Category::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            raw::Category::Custom { unit, id, default } => Category::Custom {
                unit: unit.unwrap_or(metrics::raw::UnitInfo::F64).into(),
                id,
                default: default.unwrap_or_default(),
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
