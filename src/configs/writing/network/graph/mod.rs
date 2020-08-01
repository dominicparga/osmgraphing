use super::edges;
use crate::{
    defaults,
    helpers::err,
    io::{network::graph::Writer, SupportingFileExts},
};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};
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
            match OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(err::Msg::from(format!(
                        "Couldn't open {} due to error: {}",
                        path.display(),
                        e
                    )))
                }
            }
        };

        let cfg: Config = match serde_yaml::from_reader(file) {
            Ok(cfg) => cfg,
            Err(e) => {
                return Err(err::Msg::from(format!(
                    "Serde couldn't read {} due to error: {}",
                    path.display(),
                    e
                )))
            }
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
            map_file: proto_cfg.map_file.clone(),
            nodes: nodes::Config::from(proto_cfg.nodes),
            edges: edges::Config {
                file: proto_cfg.map_file,
                is_writing_shortcuts: proto_cfg
                    .edges
                    .is_writing_shortcuts
                    .unwrap_or(defaults::parsing::IS_USING_SHORTCUTS),
                is_writing_header: false,
                is_denormalizing: proto_cfg
                    .edges
                    .is_denormalizing
                    .unwrap_or(defaults::writing::WILL_DENORMALIZE_METRICS_BY_MEAN),
                ids: proto_cfg.edges.ids,
            },
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
            map_file: raw_cfg.map_file.clone(),
            nodes: nodes::ProtoConfig::from(raw_cfg.nodes),
            edges: edges::ProtoConfig::from(edges::RawConfig {
                edges_info: edges::RawContent {
                    file: raw_cfg.map_file,
                    is_writing_shortcuts: raw_cfg.edges.is_writing_shortcuts,
                    is_denormalizing: raw_cfg.edges.is_denormalizing,
                    ids: raw_cfg.edges.ids,
                },
            }),
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct WrappedRawConfig {
    pub writing: RawConfig,
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct RawConfig {
    graph: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "map-file")]
    map_file: PathBuf,
    nodes: nodes::RawConfig,
    edges: raw_edges::Config,
}

// TODO module raw exporting stuff for pub(crate) etc.
mod raw_edges {
    use super::edges::RawCategory;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Config {
        #[serde(rename = "with_shortcuts")]
        pub is_writing_shortcuts: Option<bool>,
        #[serde(rename = "will_denormalize_metrics_by_mean")]
        pub is_denormalizing: Option<bool>,
        pub ids: Vec<RawCategory>,
    }
}
