use super::{edges, nodes};
use serde::Deserialize;
use std::path::PathBuf;

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub writing: AnotherConfig,
}

#[derive(Debug, Deserialize)]
pub struct AnotherConfig {
    pub graph: Content,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "map-file")]
    pub map_file: PathBuf,
    pub nodes: nodes::raw::Config,
    pub edges: edges::raw::Config,
}
