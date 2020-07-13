use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RawConfig {
    pub workload: SimpleId,
    pub lane_count: SimpleId,
    pub distance: SimpleId,
}
