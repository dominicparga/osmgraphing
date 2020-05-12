use serde::Deserialize;
use std::path::PathBuf;

use crate::configs::implementation::parsing::{edges, generating, nodes, vehicles};

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub parsing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Content {
    pub map_file: PathBuf,
    pub vehicles: vehicles::proto::Config,
    pub nodes: nodes::proto::Config,
    pub edges: edges::proto::Config,
    pub generating: Option<generating::proto::Config>,
}
