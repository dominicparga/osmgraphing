use crate::{helpers, io::SupportingFileExts};
use serde::Deserialize;
use std::path::{Path, PathBuf};
pub mod edges;
pub mod nodes;
pub mod raw;

#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
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

impl Config {
    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Self, String> {
        let file = {
            Self::find_supported_ext(path)?;
            helpers::open_file(path)?
        };
        match serde_yaml::from_reader(file) {
            Ok(cfg) => Ok(cfg),
            Err(msg) => Err(format!("{}", msg)),
        }
    }
}
