use crate::configs::SimpleId;
use serde::Deserialize;
use std::path::PathBuf;

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub routing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Content {
    pub route_pairs_file: Option<PathBuf>,
    pub is_ch_dijkstra: Option<bool>,
    pub metrics: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Entry {
    pub id: SimpleId,
    pub alpha: Option<f64>,
    #[serde(deserialize_with = "helpers::deserialize_f64_from_str")]
    pub tolerated_scale: Option<f64>,
}
