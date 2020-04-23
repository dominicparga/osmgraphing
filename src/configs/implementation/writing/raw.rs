use super::{edges, nodes};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub map_file: PathBuf,
    pub nodes: nodes::raw::Config,
    pub edges: edges::raw::Config,
}
