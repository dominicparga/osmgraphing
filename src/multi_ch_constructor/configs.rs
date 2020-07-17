use crate::{io::SupportingFileExts, multi_ch_constructor::defaults};
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "ProtoConfig")]
pub struct Config {
    pub fmi_graph: PathBuf,
    pub ch_fmi_graph: PathBuf,
    pub contraction_ratio: String,
    pub dim: usize,
    pub num_threads: usize,
    pub is_printing_osm_ids: bool,
    pub is_using_external_edge_ids: bool,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn try_from_str(yaml_str: &str) -> Result<Config, String> {
        let proto_cfg: ProtoConfig = {
            match serde_yaml::from_str(yaml_str) {
                Ok(proto_cfg) => proto_cfg,
                Err(e) => return Err(format!("{}", e)),
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

    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let path = path.as_ref();
        let file = {
            Config::find_supported_ext(path)?;
            OpenOptions::new()
                .read(true)
                .open(path)
                .expect(&format!("Couldn't open {}", path.display()))
        };

        let proto_cfg: ProtoConfig = match serde_yaml::from_reader(file) {
            Ok(proto_cfg) => proto_cfg,
            Err(e) => return Err(format!("{}", e)),
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
            fmi_graph: proto_cfg.fmi_graph,
            ch_fmi_graph: proto_cfg.ch_fmi_graph,
            contraction_ratio: proto_cfg
                .contraction_ratio
                .unwrap_or(String::from(defaults::CONTRACTION_RATIO)),
            dim: proto_cfg.dim,
            num_threads: proto_cfg.num_threads.unwrap_or(defaults::NUM_THREADS),
            is_printing_osm_ids: proto_cfg.is_printing_osm_ids,
            is_using_external_edge_ids: proto_cfg.is_using_external_edge_ids,
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub fmi_graph: PathBuf,
    pub ch_fmi_graph: PathBuf,
    pub contraction_ratio: Option<String>,
    pub dim: usize,
    pub num_threads: Option<usize>,
    pub is_printing_osm_ids: bool,
    pub is_using_external_edge_ids: bool,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        let raw_cfg = raw_cfg.multi_ch_constructor;

        ProtoConfig {
            fmi_graph: raw_cfg.fmi_graph,
            ch_fmi_graph: raw_cfg.ch_fmi_graph,
            contraction_ratio: raw_cfg.contraction_ratio,
            dim: raw_cfg.dim,
            num_threads: raw_cfg.num_threads,
            is_printing_osm_ids: raw_cfg.is_printing_osm_ids,
            is_using_external_edge_ids: raw_cfg.is_using_external_edge_ids,
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct RawConfig {
    #[serde(rename = "multi-ch-constructor")]
    pub multi_ch_constructor: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "fmi-graph")]
    pub fmi_graph: PathBuf,
    #[serde(rename = "contracted-graph")]
    pub ch_fmi_graph: PathBuf,
    #[serde(rename = "dimension")]
    pub dim: usize,
    #[serde(rename = "contraction-ratio")]
    pub contraction_ratio: Option<String>,
    #[serde(rename = "number_of_threads")]
    pub num_threads: Option<usize>,
    #[serde(rename = "is_printing_osm-ids")]
    pub is_printing_osm_ids: bool,
    #[serde(rename = "is_using_external_edge-ids")]
    pub is_using_external_edge_ids: bool,
}
