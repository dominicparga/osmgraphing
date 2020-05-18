use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub route_count: SimpleId,
    pub lane_count: SimpleId,
    pub distance: SimpleId,
}
