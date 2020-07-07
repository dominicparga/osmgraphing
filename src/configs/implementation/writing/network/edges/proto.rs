use super::raw;
use crate::configs::SimpleId;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub map_file: PathBuf,
    pub is_writing_shortcuts: Option<bool>,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        let raw_cfg = raw_cfg.writing.edges_info;

        Config {
            map_file: raw_cfg.map_file,
            is_writing_shortcuts: raw_cfg.is_writing_shortcuts,
            ids: raw_cfg
                .ids
                .into_iter()
                .map(|category| match category {
                    raw::Category::Id(id) => Some(id),
                    raw::Category::Ignored => None,
                })
                .collect(),
        }
    }
}
