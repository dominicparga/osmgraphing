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

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "WrappedProtoConfig")]
pub struct Config {
    pub file: PathBuf,
    pub is_writing_shortcuts: bool,
    pub is_writing_header: bool,
    pub is_denormalizing: bool,
    pub ids: Vec<Option<SimpleId>>,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl From<WrappedProtoConfig> for Config {
    fn from(proto_cfg: WrappedProtoConfig) -> Config {
        Config::from(proto_cfg.writing)
    }
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        Config {
            file: proto_cfg.file,
            is_writing_shortcuts: proto_cfg
                .is_writing_shortcuts
                .unwrap_or(defaults::parsing::IS_USING_SHORTCUTS),
            is_writing_header: defaults::writing::IS_WRITING_WITH_HEADER,
            is_denormalizing: proto_cfg
                .is_denormalizing
                .unwrap_or(defaults::writing::WILL_DENORMALIZE_METRICS_BY_MEAN),
            ids: proto_cfg.ids,
        }
    }
}

/// Used when writing graph's edges
impl From<graph::Config> for Config {
    fn from(graph_cfg: graph::Config) -> Config {
        Config {
            file: graph_cfg.map_file,
            is_writing_shortcuts: graph_cfg.edges.is_writing_shortcuts,
            is_writing_header: false,
            is_denormalizing: graph_cfg.edges.is_denormalizing,
            ids: graph_cfg.edges.ids,
        }
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

        match Writer::find_supported_ext(&cfg.file) {
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

#[derive(Debug, Deserialize)]
#[serde(from = "WrappedRawConfig")]
pub struct WrappedProtoConfig {
    pub writing: ProtoConfig,
}

impl From<WrappedRawConfig> for WrappedProtoConfig {
    fn from(raw_cfg: WrappedRawConfig) -> WrappedProtoConfig {
        WrappedProtoConfig {
            writing: ProtoConfig::from(raw_cfg.writing),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawConfig")]
pub struct ProtoConfig {
    pub file: PathBuf,
    pub is_writing_shortcuts: Option<bool>,
    pub is_denormalizing: Option<bool>,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        let raw_cfg = raw_cfg.edges_info;

        ProtoConfig {
            file: raw_cfg.file,
            is_writing_shortcuts: raw_cfg.is_writing_shortcuts,
            is_denormalizing: raw_cfg.is_denormalizing,
            ids: raw_cfg
                .ids
                .into_iter()
                .map(|category| match category {
                    RawCategory::Id(id) => Some(id),
                    RawCategory::Ignored => None,
                })
                .collect(),
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
    #[serde(rename = "edges-info")]
    pub edges_info: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "file")]
    pub file: PathBuf,
    #[serde(rename = "with_shortcuts")]
    pub is_writing_shortcuts: Option<bool>,
    #[serde(rename = "will_denormalize_metrics_by_mean")]
    pub is_denormalizing: Option<bool>,
    pub ids: Vec<RawCategory>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum RawCategory {
    Id(SimpleId),
    Ignored,
}
