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
    Metric {
        unit: metrics::proto::UnitInfo,
        id: SimpleId,
    },
    Ignored,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    SrcId,
    DstId,
    ShortcutIdx0,
    ShortcutIdx1,
}
