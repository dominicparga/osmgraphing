use crate::io::SupportingFileExts;
use std::path::PathBuf;
pub mod edges;
pub mod nodes;
pub mod raw;

#[derive(Debug)]
pub struct Config {
    pub map_file: PathBuf,
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
        Config {
            map_file: raw_cfg.map_file,
            nodes: nodes::Config::from(raw_cfg.nodes),
            edges: edges::Config::from(raw_cfg.edges),
        }
    }
}
