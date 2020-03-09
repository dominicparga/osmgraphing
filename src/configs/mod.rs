use crate::helpers;
use serde::Deserialize;
use std::{fmt, fmt::Display, path::Path};

pub mod graph;
pub mod routing;

/// Storing (default) settings for parsing the graph.
///
/// # Configuration
///
/// ## Set config-values with yaml-file
///
/// You can change the configuration with an input-file (`*.yaml`).
/// With this `yaml`-config, the parser can be adjusted to parse (edge-)metrics in the order as provided by the config-file.
/// This can help especially with map-files in `fmi`-format, since the metrics are read sequentially.
/// But since `pbf`-files does not provide a column-based metric-list, but intrinsically by parsing `osm`-data, you can distinguish between default-metrics and custom-metrics via the key `category`.
/// Default-categories are described in `EdgeCategory`.
///
/// Internally, a default-metric uses provided calculation-rules to be calculated by other default-categories as well (like the duration from length and maxspeed).
///
/// Keep in mind, that metrics (except for id) are stored as `f32` for better maintainability and efficiency.
///
///
/// ### Specifying routing (in the future)
///
/// Further, the metrics, which are used in the routing, can be listed in the routing-section with their previously defined id.
/// Comparisons are made using pareto-optimality, so there is no comparison between metrics.
/// In case you'll use personlized-routing, default-preferences can be set with weights.
/// The example below shows a routing-case, where the metric `length` is weighted with `169 / (169 + 331) = 33.8 %` while the metric `duration` is weighted with `331 / (169 + 331) = 66.2 %`.
///
///
/// ### Supported structure
///
/// The supported `yaml`-structure can be seen in `resources/configs/schema.yaml`.
///
// Every metric (!= every category) will be stored in the graph, if mentioned in this `yaml`-file.
/// If a metric is mentioned, but `provided` is false, it will be calculated (e.g. edge-length from node-coordinates and haversine).
/// Please note, that metrics being calculated (like the duration from length and maxspeed) need the respective metrics to be calculated.
///
#[derive(Debug, Deserialize)]
#[serde(from = "ProtoConfig")]
pub struct Config {
    pub graph: graph::Config,
    #[serde(skip)] // thanks to ProtoConfig
    pub routing: routing::Config,
}

impl Config {
    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let file = {
            helpers::is_file_ext_supported(path, &["yaml"])?;
            helpers::open_file(path)?
        };
        match serde_yaml::from_reader(file) {
            Ok(cfg) => Ok(cfg),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Config {
        let routing_cfg = routing::Config::from_entries(proto_cfg.routing, &proto_cfg.graph);
        Config {
            graph: proto_cfg.graph,
            routing: routing_cfg,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProtoConfig {
    graph: graph::Config,
    #[serde(default)]
    routing: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    id: MetricId,
    alpha: Option<f32>,
}

impl routing::Config {
    fn from_entries(entries: Vec<Entry>, graph_cfg: &graph::Config) -> routing::Config {
        // create super-config's structures
        let mut routing = routing::Config::with_capacity(graph_cfg.edges.metrics.count());

        // translate ids into indices
        for Entry {
            id: metric_id,
            alpha,
        } in entries.iter()
        {
            let metric_idx = graph_cfg.edges.metrics.idx(metric_id);
            routing.push(metric_idx, alpha.unwrap_or(1.0));
        }

        // return
        routing
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(from = "String")]
pub struct MetricId(pub String);

impl From<String> for MetricId {
    fn from(id: String) -> MetricId {
        MetricId(id)
    }
}

impl From<&str> for MetricId {
    fn from(id: &str) -> MetricId {
        MetricId(id.to_owned())
    }
}

impl Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Types of metrics to consider when parsing a map.
///
/// - `id`, which is not a metric per se and stored differently, but needed for `csv`-like `fmi`-format
/// - `length` in meters
/// - `maxspeed` in km/h
/// - `duration` in milliseconds
/// - `lane-count`
/// - `custom`, which is just the plain f32-value
/// - `ignore`, which is used in `csv`-like `fmi`-maps to jump over columns
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum EdgeCategory {
    Meters,
    KilometersPerHour,
    Seconds,
    LaneCount,
    Custom,
    NodeId,
    Ignore,
}

impl Display for EdgeCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl EdgeCategory {
    pub fn must_be_positive(&self) -> bool {
        match self {
            EdgeCategory::Meters
            | EdgeCategory::KilometersPerHour
            | EdgeCategory::Seconds
            | EdgeCategory::LaneCount => true,
            EdgeCategory::Custom | EdgeCategory::NodeId | EdgeCategory::Ignore => false,
        }
    }

    pub fn is_ignored(&self) -> bool {
        match self {
            EdgeCategory::NodeId | EdgeCategory::Ignore => true,
            EdgeCategory::Meters
            | EdgeCategory::KilometersPerHour
            | EdgeCategory::Seconds
            | EdgeCategory::LaneCount
            | EdgeCategory::Custom => false,
        }
    }

    fn expected_calc_rules(&self) -> Vec<EdgeCategory> {
        match self {
            EdgeCategory::KilometersPerHour => {
                vec![EdgeCategory::Meters, EdgeCategory::Seconds]
            }
            EdgeCategory::Seconds => {
                vec![EdgeCategory::Meters, EdgeCategory::KilometersPerHour]
            }
            EdgeCategory::Meters
            | EdgeCategory::LaneCount
            | EdgeCategory::Custom
            | EdgeCategory::NodeId
            | EdgeCategory::Ignore => vec![],
        }
    }
}
