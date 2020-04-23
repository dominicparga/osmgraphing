use serde::Deserialize;
use std::path::PathBuf;

use super::{edges, generating, nodes, vehicles};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub map_file: PathBuf,
    pub vehicles: vehicles::raw::Config,
    pub nodes: nodes::raw::Config,
    pub edges: edges::raw::Config,
    pub generating: Option<generating::raw::Config>,
}
