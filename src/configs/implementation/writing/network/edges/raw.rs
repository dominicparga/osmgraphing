use crate::configs::SimpleId;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub writing: AnotherConfig,
}

#[derive(Debug, Deserialize)]
pub struct AnotherConfig {
    #[serde(rename = "edges-info")]
    pub edges_info: Content,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "edges-info")]
pub struct Content {
    #[serde(rename = "map-file")]
    pub map_file: PathBuf,
    #[serde(rename = "with_shortcuts")]
    pub is_writing_shortcuts: Option<bool>,
    pub ids: Vec<Category>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Id(SimpleId),
    Ignored,
}
