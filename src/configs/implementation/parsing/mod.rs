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
#[derive(Debug, Deserialize)]
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

impl TryFrom<ProtoConfig> for Config {
    type Error = err::Msg;

    fn try_from(proto_cfg: ProtoConfig) -> err::Result<Config> {
        let proto_cfg = proto_cfg.parsing;

        Ok(Config {
            map_file: proto_cfg.map_file,
            vehicles: proto_cfg.vehicles.into(),
            nodes: proto_cfg.nodes.into(),
            edges: edges::Config::try_from(proto_cfg.edges)?,
            generating: match proto_cfg.generating {
                Some(generating_cfg) => Some(generating_cfg.into()),
                None => None,
            },
        })
    }
}

impl Config {
    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let path = path.as_ref();
        let file = {
            Config::find_supported_ext(path)?;
            OpenOptions::new()
                .read(true)
                .open(path)
                .expect(&format!("Couldn't open {}", path.display()))
        };

        let cfg: Config = match serde_yaml::from_reader(file) {
            Ok(cfg) => cfg,
            Err(msg) => return Err(format!("{}", msg)),
        };

        match Parser::find_supported_ext(&cfg.map_file) {
            Ok(_) => Ok(cfg),
            Err(msg) => Err(format!("Wrong parser-map-file: {}", msg)),
        }
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct ProtoConfig {
    pub parsing: ProtoContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProtoContent {
    pub map_file: PathBuf,
    pub vehicles: vehicles::ProtoConfig,
    pub nodes: nodes::ProtoConfig,
    pub edges: edges::ProtoConfig,
    pub generating: Option<generating::ProtoConfig>,
}
