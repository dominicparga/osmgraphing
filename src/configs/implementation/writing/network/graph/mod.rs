use crate::{
    helpers::err,
    io::{network::graph::Writer, SupportingFileExts},
};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};
pub mod edges;
pub mod nodes;
pub mod proto;
pub mod raw;

#[derive(Debug, Deserialize)]
#[serde(from = "proto::Config")]
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

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
        Config {
            map_file: proto_cfg.map_file,
            nodes: nodes::Config::from(proto_cfg.nodes),
            edges: edges::Config::from(proto_cfg.edges),
        }
    }
}

impl Config {
    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> err::Result<Config> {
        let path = path.as_ref();
        let file = {
            Config::find_supported_ext(path)?;
            OpenOptions::new()
                .read(true)
                .open(path)
                .expect(&format!("Couldn't open {}", path.display()))
        };

        let cfg: Config = match serde_yaml::from_reader(file) {
            Ok(cfg) => cfg,
            Err(e) => return Err(err::Msg::from(format!("{}", e))),
        };

        match Writer::find_supported_ext(&cfg.map_file) {
            Ok(_) => Ok(cfg),
            Err(msg) => Err(err::Msg::from(format!("Wrong writer-map-file: {}", msg))),
        }
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}
