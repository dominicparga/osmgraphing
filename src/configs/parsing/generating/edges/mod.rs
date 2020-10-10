use crate::{configs::SimpleId, defaults};
use serde::Deserialize;
use std::path::PathBuf;
pub mod merge;
pub mod metrics;

#[derive(Clone, Debug, Deserialize)]
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
    // in-place
    Merge {
        from: PathBuf,
        is_file_with_header: bool,
        edge_id: SimpleId,
        edges_info: Vec<merge::Category>,
    },
}

impl From<ProtoCategory> for Category {
    fn from(proto_category: ProtoCategory) -> Category {
        match proto_category {
            ProtoCategory::Meta { info, id } => Category::Meta {
                info: info.into(),
                id,
            },
            ProtoCategory::Custom { unit, id, default } => Category::Custom {
                unit: unit.into(),
                id,
                default,
            },
            ProtoCategory::Haversine { unit, id } => Category::Haversine {
                unit: unit.into(),
                id,
            },
            ProtoCategory::Copy { from, to } => Category::Copy {
                from: from.into(),
                to: to.into(),
            },
            ProtoCategory::Convert { from, to } => Category::Convert {
                from: from.into(),
                to: to.into(),
            },
            ProtoCategory::Calc { result, a, b } => Category::Calc {
                result: result.into(),
                a: a.into(),
                b: b.into(),
            },
            ProtoCategory::Merge {
                from,
                is_file_with_header,
                edge_id,
                edges_info,
            } => Category::Merge {
                from,
                is_file_with_header: is_file_with_header
                    .unwrap_or(defaults::writing::IS_WRITING_WITH_HEADER),
                edge_id,
                edges_info: edges_info.into_iter().map(merge::Category::from).collect(),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    EdgeId,
    SrcIdx,
    SrcLat,
    SrcLon,
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
            ProtoMetaInfo::SrcIdx => MetaInfo::SrcIdx,
            ProtoMetaInfo::SrcLat => MetaInfo::SrcLat,
            ProtoMetaInfo::SrcLon => MetaInfo::SrcLon,
            ProtoMetaInfo::DstIdx => MetaInfo::DstIdx,
            ProtoMetaInfo::DstLat => MetaInfo::DstLat,
            ProtoMetaInfo::DstLon => MetaInfo::DstLon,
            ProtoMetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
            ProtoMetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, from = "RawConfig")]
pub struct ProtoConfig(pub Vec<ProtoCategory>);

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig(raw_cfg.0.into_iter().map(ProtoCategory::from).collect())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProtoCategory {
    Meta {
        info: ProtoMetaInfo,
        id: SimpleId,
    },
    Custom {
        unit: metrics::ProtoUnitInfo,
        id: SimpleId,
        default: f64,
    },
    Haversine {
        unit: metrics::ProtoUnitInfo,
        id: SimpleId,
    },
    Copy {
        from: metrics::ProtoCategory,
        to: metrics::ProtoCategory,
    },
    Convert {
        from: metrics::ProtoCategory,
        to: metrics::ProtoCategory,
    },
    Calc {
        result: metrics::ProtoCategory,
        a: metrics::ProtoCategory,
        b: metrics::ProtoCategory,
    },
    Merge {
        from: PathBuf,
        is_file_with_header: Option<bool>,
        edge_id: SimpleId,
        edges_info: Vec<merge::ProtoCategory>,
    },
}

impl From<RawCategory> for ProtoCategory {
    fn from(raw_category: RawCategory) -> ProtoCategory {
        match raw_category {
            RawCategory::Meta { info, id } => ProtoCategory::Meta {
                info: ProtoMetaInfo::from(info),
                id,
            },
            RawCategory::Custom { unit, id, default } => ProtoCategory::Custom {
                unit: unit.unwrap_or(metrics::RawUnitInfo::F64).into(),
                id,
                default: default.unwrap_or_default(),
            },
            RawCategory::Haversine { unit, id } => ProtoCategory::Haversine {
                unit: metrics::ProtoUnitInfo::from(unit),
                id,
            },
            RawCategory::Copy { from, to } => ProtoCategory::Copy {
                from: metrics::ProtoCategory::from(from),
                to: metrics::ProtoCategory::from(to),
            },
            RawCategory::Convert { from, to } => ProtoCategory::Convert {
                from: metrics::ProtoCategory::from(from),
                to: metrics::ProtoCategory::from(to),
            },
            RawCategory::Calc { result, a, b } => ProtoCategory::Calc {
                result: metrics::ProtoCategory::from(result),
                a: metrics::ProtoCategory::from(a),
                b: metrics::ProtoCategory::from(b),
            },
            RawCategory::Merge {
                from,
                is_file_with_header,
                edge_id,
                edges_info,
            } => ProtoCategory::Merge {
                from,
                is_file_with_header,
                edge_id,
                edges_info: edges_info
                    .into_iter()
                    .map(merge::ProtoCategory::from)
                    .collect(),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ProtoMetaInfo {
    EdgeId,
    SrcIdx,
    SrcLat,
    SrcLon,
    DstIdx,
    DstLat,
    DstLon,
    ShortcutIdx0,
    ShortcutIdx1,
}

impl From<RawMetaInfo> for ProtoMetaInfo {
    fn from(raw_info: RawMetaInfo) -> ProtoMetaInfo {
        match raw_info {
            RawMetaInfo::EdgeId => ProtoMetaInfo::EdgeId,
            RawMetaInfo::SrcIdx => ProtoMetaInfo::SrcIdx,
            RawMetaInfo::SrcLat => ProtoMetaInfo::SrcLat,
            RawMetaInfo::SrcLon => ProtoMetaInfo::SrcLon,
            RawMetaInfo::DstIdx => ProtoMetaInfo::DstIdx,
            RawMetaInfo::DstLat => ProtoMetaInfo::DstLat,
            RawMetaInfo::DstLon => ProtoMetaInfo::DstLon,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig(pub Vec<RawCategory>);

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawCategory {
    Meta {
        info: RawMetaInfo,
        id: SimpleId,
    },
    Custom {
        unit: Option<metrics::RawUnitInfo>,
        id: SimpleId,
        default: Option<f64>,
    },
    Haversine {
        unit: metrics::RawUnitInfo,
        id: SimpleId,
    },
    Copy {
        from: metrics::RawCategory,
        to: metrics::RawCategory,
    },
    Convert {
        from: metrics::RawCategory,
        to: metrics::RawCategory,
    },
    Calc {
        result: metrics::RawCategory,
        a: metrics::RawCategory,
        b: metrics::RawCategory,
    },
    Merge {
        from: PathBuf,
        #[serde(rename = "with_header-line")]
        is_file_with_header: Option<bool>,
        #[serde(rename = "edge-id")]
        edge_id: SimpleId,
        #[serde(rename = "edges-info")]
        edges_info: Vec<merge::RawCategory>,
    },
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum RawMetaInfo {
    EdgeId,
    SrcIdx,
    SrcLat,
    SrcLon,
    DstIdx,
    DstLat,
    DstLon,
}
