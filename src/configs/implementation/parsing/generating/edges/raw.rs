use super::metrics;
use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config(pub Vec<Category>);

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Meta {
        info: MetaInfo,
        id: SimpleId,
    },
    Custom {
        unit: Option<metrics::raw::UnitInfo>,
        id: SimpleId,
        default: Option<f64>,
    },
    Haversine {
        unit: metrics::raw::UnitInfo,
        id: SimpleId,
    },
    Copy {
        from: metrics::raw::Category,
        to: metrics::raw::Category,
    },
    Convert {
        from: metrics::raw::Category,
        to: metrics::raw::Category,
    },
    Calc {
        result: metrics::raw::Category,
        a: metrics::raw::Category,
        b: metrics::raw::Category,
    },
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    EdgeId,
    SrcIdx,
    DstIdx,
}
