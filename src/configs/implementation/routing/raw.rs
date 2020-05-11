use crate::{configs::SimpleId, defaults};
use serde::Deserialize;
use std::{convert::TryFrom, path::PathBuf};

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub routing: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Content {
    pub route_pairs_file: Option<PathBuf>,
    pub is_ch_dijkstra: Option<bool>,
    pub metrics: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawEntry")]
pub struct Entry {
    pub id: SimpleId,
    pub alpha: f64,
    pub tolerated_scale: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct RawEntry {
    pub id: SimpleId,
    pub alpha: Option<f64>,
    pub tolerated_scale: Option<String>,
}

impl TryFrom<RawEntry> for Entry {
    type Error = String;

    fn try_from(raw_entry: RawEntry) -> Result<Entry, String> {
        let tolerated_scale = match &raw_entry.tolerated_scale {
            Some(snippet) => match snippet.to_ascii_lowercase().as_ref() {
                "inf" | "infinity" => Ok(std::f64::INFINITY),
                snippet => snippet
                    .parse::<f64>()
                    .map_err(|_| format!("Couln't parse f64-value {}", snippet)),
            },
            None => Ok(defaults::routing::TOLERATED_SCALE),
        }?;

        Ok(Entry {
            id: raw_entry.id,
            alpha: raw_entry.alpha.unwrap_or(defaults::routing::ALPHA),
            tolerated_scale,
        })
    }
}
