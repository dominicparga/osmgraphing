use super::metrics;
use serde::Deserialize;
use std::path::PathBuf;

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub balancing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "results-dir")]
    pub results_dir: PathBuf,
    #[serde(rename = "iter-0-cfg")]
    pub iter_0_cfg: PathBuf,
    #[serde(rename = "iter-i-cfg")]
    pub iter_i_cfg: PathBuf,
    #[serde(rename = "new_graph-dim")]
    pub new_graph_dim: usize,
    pub number_of_iterations: usize,
    #[serde(rename = "metric-ids")]
    pub metric_ids: metrics::raw::Config,
    #[serde(flatten)]
    pub optimization: Optimization,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Optimization {
    ExplicitEuler {
        #[serde(rename = "correction")]
        correction: f64,
    },
    // some kind of correction-function:
    // interpolating linear between point-pairs given in a file?
}
