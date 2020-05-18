use crate::{configs::SimpleId, io::SupportingFileExts};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};
pub mod proto;

#[derive(Clone, Debug)]
pub struct Config {
    pub results_dir: PathBuf,
    pub num_iterations: usize,
    pub metric_id: SimpleId,
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
            results_dir: proto_cfg.balancing.results_dir,
            num_iterations: proto_cfg.balancing.num_iterations,
            metric_id: proto_cfg.balancing.metric_id,
        })
    }

    fn _from_proto(proto_cfg: proto::Config) -> Config {
        match Config::try_from_proto(proto_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let file = {
            Config::find_supported_ext(path)?;
            OpenOptions::new().read(true).open(path).unwrap()
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
