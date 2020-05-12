use crate::{
    configs::{implementation::routing::raw, SimpleId},
    defaults::{self, capacity::DimVec},
};
use serde::Deserialize;
use std::{convert::TryFrom, path::PathBuf};

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "raw::Config")]
pub struct Config {
    pub route_pairs_file: Option<PathBuf>,
    pub is_ch_dijkstra: bool,
    pub metrics: DimVec<Entry>,
}

impl TryFrom<raw::Config> for Config {
    type Error = String;

    fn try_from(raw_cfg: raw::Config) -> Result<Config, String> {
        let mut metrics = DimVec::with_capacity(raw_cfg.routing.metrics.len());

        for raw_entry in raw_cfg.routing.metrics {
            metrics.push(Entry::try_from(raw_entry)?);
        }

        Ok(Config {
            route_pairs_file: raw_cfg.routing.route_pairs_file,
            is_ch_dijkstra: raw_cfg.routing.is_ch_dijkstra.unwrap_or(false),
            metrics,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "raw::Entry")]
pub struct Entry {
    pub id: SimpleId,
    pub alpha: f64,
    pub tolerated_scale: f64,
}

impl TryFrom<raw::Entry> for Entry {
    type Error = String;

    fn try_from(raw_entry: raw::Entry) -> Result<Entry, String> {
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
