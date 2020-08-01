use crate::{
    configs::{self, SimpleId},
    defaults,
    helpers::err,
    io::SupportingFileExts,
    multi_ch_constructor,
};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct Config {
    pub results_dir: PathBuf,
    pub multi_ch_constructor: multi_ch_constructor::Config,
    pub iter_0_cfg: PathBuf,
    pub iter_i_cfg: PathBuf,
    pub optimization: Optimization,
    pub num_iter: usize,
    pub monitoring: MonitoringConfig,
    pub num_threads: usize,
    pub seed: u64,
    pub min_new_metric: Option<f64>,
    pub is_err_when_metric_is_zero: bool,
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
            results_dir: proto_cfg.results_dir,
            multi_ch_constructor: proto_cfg.multi_ch_constructor,
            // +1 because analysing last graph needs one iteration as well
            num_iter: proto_cfg.num_metric_updates + 1,
            iter_0_cfg: proto_cfg.iter_0_cfg,
            iter_i_cfg: proto_cfg.iter_i_cfg,
            monitoring: MonitoringConfig::from(proto_cfg.monitoring),
            optimization: Optimization::from(proto_cfg.optimization),
            num_threads: proto_cfg
                .num_threads
                .unwrap_or(defaults::balancing::NUM_THREADS),
            seed: proto_cfg.seed.unwrap_or(defaults::SEED),
            min_new_metric: proto_cfg.min_new_metric,
            is_err_when_metric_is_zero: proto_cfg
                .is_err_when_metric_is_zero
                .unwrap_or(defaults::balancing::IS_ERR_WHEN_METRIC_IS_ZERO),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Optimization {
    pub metric_id: SimpleId,
    pub method: OptimizationMethod,
}

impl From<ProtoOptimization> for Optimization {
    fn from(proto_optimization: ProtoOptimization) -> Optimization {
        Optimization {
            metric_id: proto_optimization.metric_id,
            method: OptimizationMethod::from(proto_optimization.method),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OptimizationMethod {
    ExplicitEuler { correction: f64 },
    Averaging,
}

impl From<ProtoOptimizationMethod> for OptimizationMethod {
    fn from(proto_method: ProtoOptimizationMethod) -> OptimizationMethod {
        match proto_method {
            ProtoOptimizationMethod::ExplicitEuler { correction } => {
                OptimizationMethod::ExplicitEuler { correction }
            }
            ProtoOptimizationMethod::Averaging => OptimizationMethod::Averaging,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonitoringConfig {
    pub edges_info: configs::writing::network::edges::Config,
}

impl From<ProtoMonitoringConfig> for MonitoringConfig {
    fn from(proto_cfg: ProtoMonitoringConfig) -> MonitoringConfig {
        MonitoringConfig {
            edges_info: proto_cfg.edges_info,
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub results_dir: PathBuf,
    pub multi_ch_constructor: multi_ch_constructor::Config,
    pub num_metric_updates: usize,
    pub iter_0_cfg: PathBuf,
    pub iter_i_cfg: PathBuf,
    pub monitoring: ProtoMonitoringConfig,
    pub optimization: ProtoOptimization,
    pub num_threads: Option<usize>,
    pub seed: Option<u64>,
    pub min_new_metric: Option<f64>,
    pub is_err_when_metric_is_zero: Option<bool>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        let raw_cfg = raw_cfg.balancing;

        ProtoConfig {
            results_dir: raw_cfg.results_dir,
            multi_ch_constructor: raw_cfg.multi_ch_constructor,
            num_metric_updates: raw_cfg.number_of_metric_updates,
            iter_0_cfg: raw_cfg.iter_0_cfg,
            iter_i_cfg: raw_cfg.iter_i_cfg,
            monitoring: ProtoMonitoringConfig::from(raw_cfg.monitoring),
            optimization: ProtoOptimization::from(raw_cfg.optimization),
            num_threads: raw_cfg.num_threads,
            seed: raw_cfg.seed,
            min_new_metric: raw_cfg.min_new_metric,
            is_err_when_metric_is_zero: raw_cfg.is_err_when_metric_is_zero,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoMonitoringConfig {
    pub edges_info: configs::writing::network::edges::Config,
}

impl From<RawMonitoringConfig> for ProtoMonitoringConfig {
    fn from(raw_cfg: RawMonitoringConfig) -> ProtoMonitoringConfig {
        ProtoMonitoringConfig {
            edges_info: configs::writing::network::edges::Config::from(raw_cfg.edges_info),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoOptimization {
    metric_id: SimpleId,
    method: ProtoOptimizationMethod,
}

impl From<RawOptimization> for ProtoOptimization {
    fn from(raw_optimization: RawOptimization) -> ProtoOptimization {
        ProtoOptimization {
            metric_id: raw_optimization.metric_id,
            method: ProtoOptimizationMethod::from(raw_optimization.method),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ProtoOptimizationMethod {
    ExplicitEuler { correction: f64 },
    Averaging,
}

impl From<RawOptimizationMethod> for ProtoOptimizationMethod {
    fn from(raw_method: RawOptimizationMethod) -> ProtoOptimizationMethod {
        match raw_method {
            RawOptimizationMethod::ExplicitEuler { correction } => {
                ProtoOptimizationMethod::ExplicitEuler { correction }
            }
            RawOptimizationMethod::Averaging => ProtoOptimizationMethod::Averaging,
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub balancing: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "results-dir")]
    pub results_dir: PathBuf,
    #[serde(rename = "iter-0-cfg")]
    pub iter_0_cfg: PathBuf,
    #[serde(rename = "iter-i-cfg")]
    pub iter_i_cfg: PathBuf,
    #[serde(flatten)]
    pub multi_ch_constructor: multi_ch_constructor::Config,
    #[serde(rename = "number_of_metric-updates")]
    pub number_of_metric_updates: usize,
    pub monitoring: RawMonitoringConfig,
    #[serde(rename = "optimizing_with")]
    pub optimization: RawOptimization,
    #[serde(rename = "number_of_threads")]
    pub num_threads: Option<usize>,
    pub seed: Option<u64>,
    pub min_new_metric: Option<f64>,
    #[serde(rename = "throw_err_when_new_metric_is_zero")]
    pub is_err_when_metric_is_zero: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMonitoringConfig {
    #[serde(flatten)]
    edges_info: configs::writing::network::edges::ProtoConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawOptimization {
    #[serde(rename = "metric-id")]
    metric_id: SimpleId,
    method: RawOptimizationMethod,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RawOptimizationMethod {
    #[serde(rename = "explicit_euler")]
    ExplicitEuler {
        #[serde(rename = "correction")]
        correction: f64,
    },
    #[serde(rename = "averaging")]
    Averaging,
    // some kind of correction-function:
    // interpolating linear between point-pairs given in a file?
}
