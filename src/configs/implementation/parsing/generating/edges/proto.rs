use crate::configs::{implementation::parsing::generating::edges::metrics, SimpleId};
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

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    SrcIdx,
    DstIdx,
    ShortcutIdx0,
    ShortcutIdx1,
}
