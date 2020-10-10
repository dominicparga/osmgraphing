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
    pub routing_algo: RoutingAlgo,
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
                Err(e) => {
                    return Err(err::Msg::from(format!(
                        "Serde couldn't parse yaml-str due to error: {}",
                        e
                    )))
                }
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
            routing_algo: RoutingAlgo::from(proto_cfg.routing_algo),
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
            match OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(err::Msg::from(format!(
                        "Couldn't open {} due to error: {}",
                        path.display(),
                        e
                    )))
                }
            }
        };

        let proto_cfg = match serde_yaml::from_reader(file) {
            Ok(proto_cfg) => proto_cfg,
            Err(e) => {
                return Err(err::Msg::from(format!(
                    "Serde couldn't read {} due to error: {}",
                    path.display(),
                    e
                )))
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoutingAlgo {
    Dijkstra,
    CHDijkstra,
    #[cfg(feature = "gpl-3.0")]
    Explorator {
        algo: ExploratorAlgo,
    },
}

impl RoutingAlgo {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(feature = "gpl-3.0")]
impl From<ExploratorAlgo> for RoutingAlgo {
    fn from(algo: ExploratorAlgo) -> RoutingAlgo {
        match algo {
            ExploratorAlgo::Dijkstra => RoutingAlgo::Dijkstra,
            ExploratorAlgo::CHDijkstra => RoutingAlgo::CHDijkstra,
        }
    }
}

impl From<ProtoRoutingAlgo> for RoutingAlgo {
    fn from(proto_routing_algo: ProtoRoutingAlgo) -> RoutingAlgo {
        match proto_routing_algo {
            ProtoRoutingAlgo::Dijkstra => RoutingAlgo::Dijkstra,
            ProtoRoutingAlgo::CHDijkstra => RoutingAlgo::CHDijkstra,
            #[cfg(feature = "gpl-3.0")]
            ProtoRoutingAlgo::Explorator { algo } => RoutingAlgo::Explorator {
                algo: ExploratorAlgo::from(algo),
            },
        }
    }
}

#[cfg(feature = "gpl-3.0")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExploratorAlgo {
    Dijkstra,
    CHDijkstra,
}

#[cfg(feature = "gpl-3.0")]
impl From<ProtoExploratorAlgo> for ExploratorAlgo {
    fn from(proto_algo: ProtoExploratorAlgo) -> ExploratorAlgo {
        match proto_algo {
            ProtoExploratorAlgo::Dijkstra => ExploratorAlgo::Dijkstra,
            ProtoExploratorAlgo::CHDijkstra => ExploratorAlgo::CHDijkstra,
        }
    }
}

/// Don't deny unknown fields to allow multiple configs in one yaml-file.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub route_pairs_file: Option<PathBuf>,
    pub routing_algo: ProtoRoutingAlgo,
    pub metrics: DimVec<ProtoEntry>,
}

impl TryFrom<RawConfig> for ProtoConfig {
    type Error = String;

    fn try_from(raw_cfg: RawConfig) -> Result<ProtoConfig, String> {
        let raw_cfg = raw_cfg.routing;

        let mut metrics = DimVec::with_capacity(raw_cfg.metrics.len());

        for raw_entry in raw_cfg.metrics {
            metrics.push(ProtoEntry::try_from(raw_entry)?);
        }

        Ok(ProtoConfig {
            route_pairs_file: raw_cfg.route_pairs_file,
            routing_algo: ProtoRoutingAlgo::from(raw_cfg.routing_algo),
            metrics,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ProtoRoutingAlgo {
    Dijkstra,
    CHDijkstra,
    #[cfg(feature = "gpl-3.0")]
    Explorator {
        algo: ProtoExploratorAlgo,
    },
}

impl From<RawRoutingAlgo> for ProtoRoutingAlgo {
    fn from(raw_routing_algo: RawRoutingAlgo) -> ProtoRoutingAlgo {
        match raw_routing_algo {
            RawRoutingAlgo::Dijkstra => ProtoRoutingAlgo::Dijkstra,
            RawRoutingAlgo::CHDijkstra => ProtoRoutingAlgo::CHDijkstra,
            #[cfg(feature = "gpl-3.0")]
            RawRoutingAlgo::Explorator { algo } => ProtoRoutingAlgo::Explorator {
                algo: ProtoExploratorAlgo::from(algo),
            },
        }
    }
}

#[cfg(feature = "gpl-3.0")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProtoExploratorAlgo {
    Dijkstra,
    CHDijkstra,
}

#[cfg(feature = "gpl-3.0")]
impl From<RawExploratorAlgo> for ProtoExploratorAlgo {
    fn from(raw_algo: RawExploratorAlgo) -> ProtoExploratorAlgo {
        match raw_algo {
            RawExploratorAlgo::Dijkstra => ProtoExploratorAlgo::Dijkstra,
            RawExploratorAlgo::CHDijkstra => ProtoExploratorAlgo::CHDijkstra,
        }
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
    #[serde(rename = "algorithm")]
    pub routing_algo: RawRoutingAlgo,
    pub metrics: Vec<RawEntry>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RawRoutingAlgo {
    Dijkstra,
    CHDijkstra,
    #[cfg(feature = "gpl-3.0")]
    Explorator {
        algo: RawExploratorAlgo,
    },
}

#[cfg(feature = "gpl-3.0")]
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RawExploratorAlgo {
    Dijkstra,
    CHDijkstra,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawEntry {
    pub id: SimpleId,
    pub alpha: Option<f64>,
    #[serde(rename = "tolerated-scale")]
    pub tolerated_scale: Option<String>,
}
