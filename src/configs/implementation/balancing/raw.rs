use crate::configs::implementation::balancing::metrics;
use serde::Deserialize;
use std::path::PathBuf;

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub balancing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Content {
    pub results_dir: PathBuf,
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
