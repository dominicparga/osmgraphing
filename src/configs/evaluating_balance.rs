use crate::{configs, defaults, helpers::err, io::SupportingFileExts};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct Config {
    pub seed: u64,
    pub results_dir: PathBuf,
    pub monitoring: super::balancing::MonitoringConfig,
    pub num_threads: usize,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn try_from_str(yaml_str: &str) -> err::Result<Config> {
        let proto_cfg: ProtoConfig = {
            match serde_yaml::from_str(yaml_str) {
                Ok(proto_cfg) => proto_cfg,
                Err(e) => {
                    return Err(err::Msg::from(format!(
                        "Serde couldn't parse yaml-str due to error: {}",
                        e
                    )))
                }
            }
        };
        Ok(Config::from(proto_cfg))
    }

    pub fn from_str(yaml_str: &str) -> Config {
        match Config::try_from_str(yaml_str) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

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

        let proto_cfg: ProtoConfig = match serde_yaml::from_reader(file) {
            Ok(proto_cfg) => proto_cfg,
            Err(e) => {
                return Err(err::Msg::from(format!(
                    "Serde couldn't read {} due to error: {}",
                    path.display(),
                    e
                )))
            }
        };
        Ok(Config::from(proto_cfg))
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        Config {
            seed: proto_cfg.seed.unwrap_or(defaults::SEED),
            results_dir: proto_cfg.results_dir,
            monitoring: super::balancing::MonitoringConfig::from(proto_cfg.monitoring),
            num_threads: proto_cfg
                .num_threads
                .unwrap_or(defaults::balancing::NUM_THREADS),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonitoringConfig {
    pub edges_info: configs::writing::network::edges::Config,
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub seed: Option<u64>,
    pub results_dir: PathBuf,
    pub monitoring: super::balancing::ProtoMonitoringConfig,
    pub num_threads: Option<usize>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        let raw_cfg = raw_cfg.evaluating_balance;

        ProtoConfig {
            seed: raw_cfg.seed,
            results_dir: raw_cfg.results_dir,
            monitoring: super::balancing::ProtoMonitoringConfig::from(raw_cfg.monitoring),
            num_threads: raw_cfg.num_threads,
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub evaluating_balance: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    pub seed: Option<u64>,
    #[serde(rename = "results-dir")]
    pub results_dir: PathBuf,
    pub monitoring: super::balancing::RawMonitoringConfig,
    #[serde(rename = "number_of_threads")]
    pub num_threads: Option<usize>,
}
