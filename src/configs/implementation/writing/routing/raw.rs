use serde::Deserialize;
use std::path::PathBuf;

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub writing: AnotherConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AnotherConfig {
    pub route_pairs: Content,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    pub file: PathBuf,
    #[serde(flatten)]
    pub category: Category,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    #[serde(rename = "random_or_all")]
    RandomOrAll { seed: u64, max_count: usize },
}
