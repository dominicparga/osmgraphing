use crate::configs::{implementation::balancing::raw, SimpleId};
use serde::Deserialize;
use std::{convert::TryFrom, path::PathBuf};

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "raw::Config")]
pub struct Config {
    pub results_dir: PathBuf,
    pub num_iter: usize,
    pub iter_0_cfg: PathBuf,
    pub iter_i_cfg: PathBuf,
    pub workload_id: SimpleId,
    pub lane_count_id: SimpleId,
    pub distance_id: SimpleId,
    pub optimization: Optimization,
}

impl TryFrom<raw::Config> for Config {
    type Error = String;

    fn try_from(raw_cfg: raw::Config) -> Result<Config, String> {
        Ok(Config {
            results_dir: raw_cfg.balancing.results_dir,
            num_iter: raw_cfg.balancing.number_of_iterations,
            iter_0_cfg: raw_cfg.balancing.iter_0_cfg,
            iter_i_cfg: raw_cfg.balancing.iter_i_cfg,
            workload_id: raw_cfg.balancing.metric_ids.workload,
            lane_count_id: raw_cfg.balancing.metric_ids.lane_count,
            distance_id: raw_cfg.balancing.metric_ids.distance,
            optimization: Optimization::from(raw_cfg.balancing.optimization),
        })
    }
}

#[derive(Debug)]
pub enum Optimization {
    ExplicitEuler { correction: f64 },
}

impl From<raw::Optimization> for Optimization {
    fn from(raw_optimization: raw::Optimization) -> Optimization {
        match raw_optimization {
            raw::Optimization::ExplicitEuler { correction } => Optimization::ExplicitEuler {
                correction: correction,
            },
        }
    }
}
