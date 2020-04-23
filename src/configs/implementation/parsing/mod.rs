use crate::{helpers, io::SupportingFileExts};
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub mod edges;
pub mod generating;
pub mod nodes;
pub mod raw;
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
#[serde(from = "raw::Config")]
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

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        Config {
            map_file: raw_cfg.map_file,
            vehicles: raw_cfg.vehicles.into(),
            nodes: raw_cfg.nodes.into(),
            edges: raw_cfg.edges.into(),
            generating: match raw_cfg.generating {
                Some(generating_cfg) => Some(generating_cfg.into()),
                None => None,
            },
        }
    }
}

impl Config {
    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Self, String> {
        let file = {
            Self::find_supported_ext(path)?;
            helpers::open_file(path)?
        };
        match serde_yaml::from_reader(file) {
            Ok(cfg) => Ok(cfg),
            Err(msg) => Err(format!("{}", msg)),
        }
    }
}
