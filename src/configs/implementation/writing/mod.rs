use crate::{
    helpers,
    io::{SupportingFileExts, Writer},
};
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
        let raw_cfg = raw_cfg.writing;

        Config {
            map_file: raw_cfg.map_file,
            nodes: nodes::Config::from(raw_cfg.nodes),
            edges: edges::Config::from(raw_cfg.edges),
        }
    }
}

impl Config {
    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let file = {
            Config::find_supported_ext(path)?;
            helpers::open_file(path)?
        };

        let cfg: Config = match serde_yaml::from_reader(file) {
            Ok(cfg) => cfg,
            Err(msg) => return Err(format!("{}", msg)),
        };

        match Writer::find_supported_ext(&cfg.map_file) {
            Ok(_) => Ok(cfg),
            Err(msg) => Err(format!("Wrong writer-map-file: {}", msg)),
        }
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}
