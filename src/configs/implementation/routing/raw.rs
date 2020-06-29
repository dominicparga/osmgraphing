use crate::configs::SimpleId;
use serde::Deserialize;
use std::path::PathBuf;

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub routing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "route-pairs-file")]
    pub route_pairs_file: Option<PathBuf>,
    #[serde(rename = "is_ch-dijkstra")]
    pub is_ch_dijkstra: Option<bool>,
    pub metrics: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub id: SimpleId,
    pub alpha: Option<f64>,
    #[serde(rename = "tolerated-scale")]
    pub tolerated_scale: Option<String>,
}
