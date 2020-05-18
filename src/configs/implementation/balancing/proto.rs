use crate::configs::{implementation::balancing::raw, SimpleId};
use serde::Deserialize;
use std::{convert::TryFrom, path::PathBuf};

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "raw::Config")]
pub struct Config {
    pub results_dir: PathBuf,
    pub num_iterations: usize,
    pub route_count_id: SimpleId,
    pub lane_count_id: SimpleId,
    pub distance_id: SimpleId,
}

impl TryFrom<raw::Config> for Config {
    type Error = String;

    fn try_from(raw_cfg: raw::Config) -> Result<Config, String> {
        Ok(Config {
            results_dir: raw_cfg.balancing.results_dir,
            num_iterations: raw_cfg.balancing.num_iterations,
            route_count_id: raw_cfg.balancing.metric_ids.route_count,
            lane_count_id: raw_cfg.balancing.metric_ids.lane_count,
            distance_id: raw_cfg.balancing.metric_ids.distance,
        })
    }
}
