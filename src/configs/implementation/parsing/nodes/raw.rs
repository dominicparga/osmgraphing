use crate::configs::{implementation::parsing::nodes::metrics, SimpleId};
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
        unit: metrics::raw::UnitInfo,
        id: SimpleId,
    },
    Ignored,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MetaInfo {
    NodeId,
    Level,
}
