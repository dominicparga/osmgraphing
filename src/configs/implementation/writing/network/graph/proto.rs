use super::{edges, nodes, raw};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub map_file: PathBuf,
    pub nodes: nodes::proto::Config,
    pub edges: edges::proto::Config,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        let raw_cfg = raw_cfg.writing.graph;

        Config {
            map_file: raw_cfg.map_file,
            nodes: nodes::proto::Config::from(raw_cfg.nodes),
            edges: edges::proto::Config::from(raw_cfg.edges),
        }
    }
}
