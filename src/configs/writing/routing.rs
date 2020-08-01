use crate::{
    defaults,
    helpers::err,
    io::{routing::Writer, SupportingFileExts},
};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
#[serde(from = "WrappedProtoConfig")]
pub struct Config {
    pub file: PathBuf,
    pub category: Category,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl From<WrappedProtoConfig> for Config {
    fn from(proto_cfg: WrappedProtoConfig) -> Config {
        Config {
            file: proto_cfg.file,
            category: Category::from(proto_cfg.category),
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
            Err(msg) => Err(err::Msg::from(format!("Wrong writer-routes-file: {}", msg))),
        }
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Category {
    RandomOrAll { seed: u64, max_count: usize },
}

impl From<ProtoCategory> for Category {
    fn from(raw_category: ProtoCategory) -> Category {
        match raw_category {
            ProtoCategory::RandomOrAll { seed, max_count } => {
                Category::RandomOrAll { seed, max_count }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "WrappedRawConfig", deny_unknown_fields)]
pub struct WrappedProtoConfig {
    pub file: PathBuf,
    #[serde(flatten)]
    pub category: ProtoCategory,
}

impl From<WrappedRawConfig> for WrappedProtoConfig {
    fn from(raw_cfg: WrappedRawConfig) -> WrappedProtoConfig {
        let raw_cfg = raw_cfg.writing.route_pairs;

        WrappedProtoConfig {
            file: raw_cfg.file,
            category: ProtoCategory::from(raw_cfg.category),
        }
    }
}

#[derive(Debug)]
pub enum ProtoCategory {
    RandomOrAll { seed: u64, max_count: usize },
}

impl From<RawCategory> for ProtoCategory {
    fn from(raw_category: RawCategory) -> ProtoCategory {
        match raw_category {
            RawCategory::RandomOrAll { seed, max_count } => ProtoCategory::RandomOrAll {
                seed: seed.unwrap_or(defaults::SEED),
                max_count,
            },
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct WrappedRawConfig {
    pub writing: RawConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RawConfig {
    pub route_pairs: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    pub file: PathBuf,
    #[serde(flatten)]
    pub category: RawCategory,
}

#[derive(Clone, Debug, Deserialize)]
pub enum RawCategory {
    #[serde(rename = "random_or_all")]
    RandomOrAll { seed: Option<u64>, max_count: usize },
}
