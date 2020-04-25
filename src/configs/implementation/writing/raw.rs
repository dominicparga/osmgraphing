use super::{edges, nodes};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub writing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Content {
    pub map_file: PathBuf,
    pub nodes: nodes::raw::Config,
    pub edges: edges::raw::Config,
}
