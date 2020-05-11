use crate::{
    helpers,
    io::{routing::Writer, SupportingFileExts},
};
use serde::Deserialize;
use std::path::{Path, PathBuf};
pub mod raw;

#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub file: PathBuf,
    pub category: Category,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        let raw_cfg = raw_cfg.writing.route_pairs;

        Config {
            file: raw_cfg.file,
            category: raw_cfg.category.into(),
        }
    }
}

impl Config {
    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let file = {
            Config::find_supported_ext(path)?;
            helpers::open_file(path)?
        };

        let cfg: Config = match serde_yaml::from_reader(file) {
            Ok(cfg) => cfg,
            Err(msg) => return Err(format!("{}", msg)),
        };

        match Writer::find_supported_ext(&cfg.file) {
            Ok(_) => Ok(cfg),
            Err(msg) => Err(format!("Wrong writer-routes-file: {}", msg)),
        }
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Config {
        match Config::try_from_yaml(path) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

#[derive(Debug)]
pub enum Category {
    Random { seed: u64, count: usize },
}

impl From<raw::Category> for Category {
    fn from(raw_category: raw::Category) -> Category {
        match raw_category {
            raw::Category::Random { seed, count } => Category::Random { seed, count },
        }
    }
}
