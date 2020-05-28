use crate::{configs, io::SupportingFileExts, network::MetricIdx};
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
    pub workload_idx: MetricIdx,
    pub lane_count_idx: MetricIdx,
    pub distance_idx: MetricIdx,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn try_from_str(
        yaml_str: &str,
        parsing_cfg: &configs::parsing::Config,
    ) -> Result<Config, String> {
        let proto_cfg = {
            match serde_yaml::from_str(yaml_str) {
                Ok(proto_cfg) => proto_cfg,
                Err(e) => return Err(format!("{}", e)),
            }
        };
        Config::try_from_proto(proto_cfg, parsing_cfg)
    }

    pub fn from_str(yaml_str: &str, parsing_cfg: &configs::parsing::Config) -> Config {
        match Config::try_from_str(yaml_str, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    fn try_from_proto(
        proto_cfg: proto::Config,
        parsing_cfg: &configs::parsing::Config,
    ) -> Result<Config, String> {
        Ok(Config {
            results_dir: proto_cfg.results_dir,
            workload_idx: parsing_cfg.edges.metrics.idx_of(&proto_cfg.workload_id),
            lane_count_idx: parsing_cfg.edges.metrics.idx_of(&proto_cfg.lane_count_id),
            distance_idx: parsing_cfg.edges.metrics.idx_of(&proto_cfg.distance_id),
        })
    }

    fn _from_proto(proto_cfg: proto::Config, parsing_cfg: &configs::parsing::Config) -> Config {
        match Config::try_from_proto(proto_cfg, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(
        path: &P,
        parsing_cfg: &configs::parsing::Config,
    ) -> Result<Config, String> {
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
        Config::try_from_proto(proto_cfg, parsing_cfg)
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(
        path: &P,
        parsing_cfg: &configs::parsing::Config,
    ) -> Config {
        match Config::try_from_yaml(path, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}
