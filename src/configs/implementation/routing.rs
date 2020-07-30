use crate::{
    configs::{self, SimpleId},
    defaults::{self, capacity::DimVec},
    helpers::err,
    io::SupportingFileExts,
};
use serde::Deserialize;
use smallvec::smallvec;
use std::{
    convert::TryFrom,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

/// # Specifying routing (TODO update text)
///
/// Further, the metrics, which are used in the routing, can be listed in the routing-section with their previously defined id.
/// Comparisons are made using pareto-optimality, so there is no comparison between metrics.
/// In case you'll use personlized-routing, default-preferences can be set with weights.
/// The example below shows a routing-case, where the metric `distance` is weighted with `169 / (169 + 331) = 33.8 %` while the metric `duration` is weighted with `331 / (169 + 331) = 66.2 %`.
#[derive(Clone, Debug)]
pub struct Config {
    pub route_pairs_file: Option<PathBuf>,
    pub is_ch_dijkstra: bool,
    pub alphas: DimVec<f64>,
    pub tolerated_scales: DimVec<f64>,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn try_from_str(
        yaml_str: &str,
        parsing_cfg: &configs::parsing::Config,
    ) -> err::Result<Config> {
        let proto_cfg = {
            match serde_yaml::from_str(yaml_str) {
                Ok(proto_cfg) => proto_cfg,
                Err(e) => return Err(format!("{}", e).into()),
            }
        };
        Config::try_from_proto(proto_cfg, parsing_cfg)
    }

    pub fn from_str(yaml_str: &str, parsing_cfg: &configs::parsing::Config) -> Config {
        match Config::try_from_str(yaml_str, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    fn try_from_proto(
        proto_cfg: ProtoConfig,
        parsing_cfg: &configs::parsing::Config,
    ) -> err::Result<Config> {
        let dim = parsing_cfg.edges.metrics.units.len();

        // Alpha is 0.0 because non-mentioned id will not be considered.
        let mut alphas = smallvec![0.0; dim];
        // Same argument holds for the toleration.
        let mut tolerated_scales = smallvec![defaults::routing::TOLERATED_SCALE_INF; dim];

        for entry in proto_cfg.metrics.into_iter() {
            let metric_idx = parsing_cfg.edges.metrics.try_idx_of(&entry.id)?;
            alphas[*metric_idx] = entry.alpha;
            tolerated_scales[*metric_idx] = entry.tolerated_scale;
        }

        Ok(Config {
            route_pairs_file: proto_cfg.route_pairs_file,
            is_ch_dijkstra: proto_cfg.is_ch_dijkstra,
            alphas,
            tolerated_scales,
        })
    }

    fn _from_proto(proto_cfg: ProtoConfig, parsing_cfg: &configs::parsing::Config) -> Config {
        match Config::try_from_proto(proto_cfg, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(
        path: &P,
        parsing_cfg: &configs::parsing::Config,
    ) -> err::Result<Config> {
        let path = path.as_ref();
        let file = {
            Config::find_supported_ext(path)?;
            OpenOptions::new()
                .read(true)
                .open(path)
                .expect(&format!("Couldn't open {}", path.display()))
        };

        let proto_cfg = match serde_yaml::from_reader(file) {
            Ok(proto_cfg) => proto_cfg,
            Err(e) => {
                return Err(format!("Couldn't open {} due to error: {}", path.display(), e).into())
            }
        };
        Config::try_from_proto(proto_cfg, parsing_cfg)
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(
        path: &P,
        parsing_cfg: &configs::parsing::Config,
    ) -> Config {
        match Config::try_from_yaml(path, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub route_pairs_file: Option<PathBuf>,
    pub is_ch_dijkstra: bool,
    pub metrics: DimVec<ProtoEntry>,
}

impl TryFrom<RawConfig> for ProtoConfig {
    type Error = String;

    fn try_from(raw_cfg: RawConfig) -> Result<ProtoConfig, String> {
        let mut metrics = DimVec::with_capacity(raw_cfg.routing.metrics.len());

        for raw_entry in raw_cfg.routing.metrics {
            metrics.push(ProtoEntry::try_from(raw_entry)?);
        }

        Ok(ProtoConfig {
            route_pairs_file: raw_cfg.routing.route_pairs_file,
            is_ch_dijkstra: raw_cfg.routing.is_ch_dijkstra.unwrap_or(false),
            metrics,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawEntry")]
pub struct ProtoEntry {
    pub id: SimpleId,
    pub alpha: f64,
    pub tolerated_scale: f64,
}

impl TryFrom<RawEntry> for ProtoEntry {
    type Error = String;

    fn try_from(raw_entry: RawEntry) -> Result<ProtoEntry, String> {
        let tolerated_scale = match &raw_entry.tolerated_scale {
            Some(snippet) => match snippet.to_ascii_lowercase().as_ref() {
                "inf" | "infinity" => Ok(defaults::routing::TOLERATED_SCALE_INF),
                snippet => snippet
                    .parse::<f64>()
                    .map_err(|_| format!("Couln't parse f64-value {}", snippet)),
            },
            None => Ok(defaults::routing::TOLERATED_SCALE),
        }?;

        Ok(ProtoEntry {
            id: raw_entry.id,
            alpha: raw_entry.alpha.unwrap_or(defaults::routing::ALPHA),
            tolerated_scale,
        })
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub routing: RawContent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawContent {
    #[serde(rename = "route-pairs-file")]
    pub route_pairs_file: Option<PathBuf>,
    #[serde(rename = "is_ch-dijkstra")]
    pub is_ch_dijkstra: Option<bool>,
    pub metrics: Vec<RawEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawEntry {
    pub id: SimpleId,
    pub alpha: Option<f64>,
    #[serde(rename = "tolerated-scale")]
    pub tolerated_scale: Option<String>,
}
