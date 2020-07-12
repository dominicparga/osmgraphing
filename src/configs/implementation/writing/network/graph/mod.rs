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

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "WrappedProtoConfig")]
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

impl From<WrappedProtoConfig> for Config {
    fn from(proto_cfg: WrappedProtoConfig) -> Config {
        Config {
            map_file: proto_cfg.map_file,
            nodes: nodes::Config::from(proto_cfg.nodes),
            edges: edges::Config::from(proto_cfg.edges),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "WrappedRawConfig")]
pub struct WrappedProtoConfig {
    pub map_file: PathBuf,
    pub nodes: nodes::ProtoConfig,
    pub edges: edges::ProtoConfig,
}

impl From<WrappedRawConfig> for WrappedProtoConfig {
    fn from(raw_cfg: WrappedRawConfig) -> WrappedProtoConfig {
        let raw_cfg = raw_cfg.writing.graph;

        WrappedProtoConfig {
            map_file: raw_cfg.map_file,
            nodes: nodes::ProtoConfig::from(raw_cfg.nodes),
            edges: edges::ProtoConfig::from(raw_cfg.edges),
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct WrappedRawConfig {
    pub writing: RawConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConfig {
    graph: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "map-file")]
    map_file: PathBuf,
    nodes: nodes::RawConfig,
    edges: edges::RawConfig,
}
