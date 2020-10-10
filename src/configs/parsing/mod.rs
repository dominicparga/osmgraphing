use crate::{
    helpers::err,
    io::{network::graph::Parser, SupportingFileExts},
};
use serde::Deserialize;
use std::{
    convert::TryFrom,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

pub mod edges;
pub mod generating;
pub mod nodes;
pub mod vehicles;

/// # Set config-values with yaml-file (TODO update this text)
///
/// You can change the configuration with an input-file (`*.yaml`).
/// With this `yaml`-config, the parser can be adjusted to parse (edge-)metrics in the order as provided by the config-file.
/// This can help especially with map-files in `fmi`-format, since the metrics are read sequentially.
/// But since `pbf`-files does not provide a column-based metric-list, but intrinsically by parsing `osm`-data, you can distinguish between default-metrics and custom-metrics via the key `category`.
/// Default-categories are described in `EdgeCategory`.
///
/// Internally, a default-metric uses provided calculation-rules to be calculated by other default-categories as well (like the duration from distance and maxspeed).
///
/// Keep in mind, that metrics (except for id) are stored as `f64` for better maintainability and efficiency.
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "ProtoConfig")]
pub struct Config {
    pub map_file: PathBuf,
    pub vehicles: vehicles::Config,
    pub nodes: nodes::Config,
    pub edges: edges::Config,
    pub generating: Option<generating::Config>,
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

        match Parser::find_supported_ext(&cfg.map_file) {
            Ok(_) => Ok(cfg),
            Err(msg) => Err(err::Msg::from(format!("Wrong parser-map-file: {}", msg))),
        }
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

impl TryFrom<ProtoConfig> for Config {
    type Error = err::Msg;

    fn try_from(proto_cfg: ProtoConfig) -> err::Result<Config> {
        Ok(Config {
            map_file: proto_cfg.map_file,
            vehicles: match proto_cfg.vehicles {
                Some(proto_vehicles) => vehicles::Config::from(proto_vehicles),
                None => vehicles::Config::default(),
            },
            nodes: nodes::Config::from(proto_cfg.nodes),
            edges: edges::Config::try_from(proto_cfg.edges)?,
            generating: proto_cfg.generating.map(generating::Config::from),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub map_file: PathBuf,
    pub vehicles: Option<vehicles::ProtoConfig>,
    pub nodes: nodes::ProtoConfig,
    pub edges: edges::ProtoConfig,
    pub generating: Option<generating::ProtoConfig>,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        let raw_cfg = raw_cfg.parsing;

        ProtoConfig {
            map_file: raw_cfg.map_file,
            vehicles: raw_cfg.vehicles.map(vehicles::ProtoConfig::from),
            nodes: nodes::ProtoConfig::from(raw_cfg.nodes),
            edges: edges::ProtoConfig::from(raw_cfg.edges),
            generating: raw_cfg.generating.map(generating::ProtoConfig::from),
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub parsing: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "map-file")]
    pub map_file: PathBuf,
    pub vehicles: Option<vehicles::RawConfig>,
    pub nodes: nodes::RawConfig,
    pub edges: edges::RawConfig,
    pub generating: Option<generating::RawConfig>,
}
