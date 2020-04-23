use crate::{helpers, io::SupportingFileExts};
use serde::Deserialize;
use std::{fmt, fmt::Display, path::Path};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(from = "String")]
pub struct SimpleId(pub String);

impl From<String> for SimpleId {
    fn from(id: String) -> SimpleId {
        SimpleId(id)
    }
}

impl From<&str> for SimpleId {
    fn from(id: &str) -> SimpleId {
        SimpleId(id.to_owned())
    }
}

impl Display for SimpleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod parsing;
pub mod raw;
pub mod routing;
pub mod writing;

/// Storing (default) settings for parsing the graph.
///
/// # Configuration
///
/// ### Supported structure
///
/// The supported `yaml`-structure can be seen in `resources/configs/schema.yaml`.
///
// Every metric (!= every category) will be stored in the graph, if mentioned in this `yaml`-file.
/// If a metric is mentioned, but `provided` is false, it will be calculated (e.g. edge-distance from node-coordinates and haversine).
/// Please note, that metrics being calculated (like the duration from distance and maxspeed) need the respective metrics to be calculated.
#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub parsing: parsing::Config,
    pub writing: Option<writing::Config>,
    pub routing: Option<routing::Config>,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        // build sub-cfgs

        let parsing_cfg = parsing::Config::from(raw_cfg.parsing);

        let writing_cfg = match raw_cfg.writing {
            Some(raw_writing_cfg) => Some(writing::Config::from(raw_writing_cfg)),
            None => None,
        };

        let routing_cfg = match raw_cfg.routing {
            Some(raw_routing_cfg) => Some(routing::Config::from_raw(raw_routing_cfg, &parsing_cfg)),
            None => None,
        };

        // finish cfg

        Config {
            parsing: parsing_cfg,
            writing: writing_cfg,
            routing: routing_cfg,
        }
    }
}

impl Config {
    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let file = {
            Config::find_supported_ext(path)?;
            helpers::open_file(path)?
        };
        match serde_yaml::from_reader(file) {
            Ok(cfg) => Ok(cfg),
            Err(msg) => Err(format!("{}", msg)),
        }
    }
}
