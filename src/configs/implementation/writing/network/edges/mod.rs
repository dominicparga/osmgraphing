use crate::{
    configs::{writing::network::graph, SimpleId},
    defaults,
    helpers::err,
    io::{network::edges::Writer, SupportingFileExts},
};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};
pub mod proto;
pub mod raw;

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "proto::Config")]
pub struct Config {
    pub map_file: PathBuf,
    pub is_writing_shortcuts: bool,
    pub is_writing_header: bool,
    pub ids: Vec<Option<SimpleId>>,
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
            is_writing_shortcuts: proto_cfg
                .is_writing_shortcuts
                .unwrap_or(defaults::parsing::IS_USING_SHORTCUTS),
            is_writing_header: true,
            ids: proto_cfg.ids,
        }
    }
}

impl From<graph::Config> for Config {
    fn from(graph_cfg: graph::Config) -> Config {
        Config {
            map_file: graph_cfg.map_file,
            is_writing_shortcuts: graph_cfg.edges.is_writing_shortcuts,
            is_writing_header: false,
            ids: graph_cfg.edges.ids,
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
