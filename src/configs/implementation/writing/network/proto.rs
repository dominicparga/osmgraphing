use crate::{
    configs::implementation::writing::network::{
        edges::proto as edges, nodes::proto as nodes, raw,
    },
    io::SupportingFileExts,
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub map_file: PathBuf,
    pub is_writing_shortcuts: Option<bool>,
    pub nodes: nodes::Config,
    pub edges: edges::Config,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        let raw_cfg = raw_cfg.writing.graph;

        Config {
            map_file: raw_cfg.map_file,
            is_writing_shortcuts: raw_cfg.is_writing_shortcuts,
            nodes: nodes::Config::from(raw_cfg.nodes),
            edges: edges::Config::from(raw_cfg.edges),
        }
    }
}
