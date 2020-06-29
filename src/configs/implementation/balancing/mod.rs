use crate::{configs::SimpleId, defaults, io::SupportingFileExts};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};
pub mod metrics;
pub mod proto;
pub mod raw;

#[derive(Clone, Debug)]
pub struct Config {
    pub results_dir: PathBuf,
    pub multi_ch_constructor: MultiChConstructor,
    pub num_iter: usize,
    pub iter_0_cfg: PathBuf,
    pub iter_i_cfg: PathBuf,
    pub workload_id: SimpleId,
    pub lane_count_id: SimpleId,
    pub distance_id: SimpleId,
    pub optimization: Optimization,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn try_from_str(yaml_str: &str) -> Result<Config, String> {
        let proto_cfg = {
            match serde_yaml::from_str(yaml_str) {
                Ok(proto_cfg) => proto_cfg,
                Err(e) => return Err(format!("{}", e)),
            }
        };
        Config::try_from_proto(proto_cfg)
    }

    pub fn from_str(yaml_str: &str) -> Config {
        match Config::try_from_str(yaml_str) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    fn try_from_proto(proto_cfg: proto::Config) -> Result<Config, String> {
        Ok(Config {
            results_dir: proto_cfg.results_dir,
            multi_ch_constructor: MultiChConstructor {
                dir: PathBuf::from(defaults::balancing::paths::multi_ch_constructor::DIR),
                contraction_ratio: String::from(defaults::balancing::CONTRACTION_RATIO),
            },
            num_iter: proto_cfg.num_iter,
            iter_0_cfg: proto_cfg.iter_0_cfg,
            iter_i_cfg: proto_cfg.iter_i_cfg,
            workload_id: proto_cfg.workload_id,
            lane_count_id: proto_cfg.lane_count_id,
            distance_id: proto_cfg.distance_id,
            optimization: Optimization::from(proto_cfg.optimization),
        })
    }

    fn _from_proto(proto_cfg: proto::Config) -> Config {
        match Config::try_from_proto(proto_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let path = path.as_ref();
        let file = {
            Config::find_supported_ext(path)?;
            OpenOptions::new()
                .read(true)
                .open(path)
                .expect(&format!("Couldn't open {}", path.display()))
        };

        let proto_cfg = match serde_yaml::from_reader(file) {
            Ok(proto_cfg) => proto_cfg,
            Err(e) => return Err(format!("{}", e)),
        };
        Config::try_from_proto(proto_cfg)
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiChConstructor {
    pub dir: PathBuf,
    pub contraction_ratio: String,
}

#[derive(Debug, Clone)]
pub enum Optimization {
    ExplicitEuler { correction: f64 },
}

impl From<proto::Optimization> for Optimization {
    fn from(proto_optimization: proto::Optimization) -> Optimization {
        match proto_optimization {
            proto::Optimization::ExplicitEuler { correction } => Optimization::ExplicitEuler {
                correction: correction,
            },
        }
    }
}
