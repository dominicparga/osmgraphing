use crate::helpers;
pub use graph::{edges::EdgeCategory, nodes::NodeCategory};
use serde::Deserialize;
use std::{fmt, fmt::Display, path::Path};

mod raw;

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
#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub graph: graph::Config,
    pub export: Option<export::Config>,
    pub routing: Option<routing::Config>,
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

pub mod graph {
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Config {
        pub map_file: PathBuf,
        pub vehicles: vehicles::Config,
        pub nodes: nodes::Config,
        pub edges: edges::Config,
    }

    pub mod vehicles {
        use crate::network::VehicleCategory;

        #[derive(Debug)]
        pub struct Config {
            pub category: VehicleCategory,
            pub are_drivers_picky: bool,
        }
    }

    pub mod nodes {
        use serde::Deserialize;

        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
        pub enum NodeCategory {
            NodeId,
            Latitude,
            Longitude,
            Level,
            Ignore,
        }

        #[derive(Debug)]
        pub struct Config {
            categories: Vec<NodeCategory>,
        }

        impl Config {
            pub fn new(categories: Vec<NodeCategory>) -> Config {
                Config { categories }
            }

            pub fn categories(&self) -> &Vec<NodeCategory> {
                &self.categories
            }
        }
    }

    pub mod edges {
        use crate::{configs::SimpleId, defaults::DimVec, network::MetricIdx};
        use serde::Deserialize;
        use smallvec::smallvec;
        use std::{
            collections::BTreeMap,
            fmt::{self, Display},
        };

        #[derive(Debug)]
        pub struct Config {
            // store for order
            edge_categories: Vec<EdgeCategory>,
            // store for quick access
            metric_categories: DimVec<EdgeCategory>,
            are_metrics_provided: DimVec<bool>,
            ids: DimVec<SimpleId>,
            indices: BTreeMap<SimpleId, MetricIdx>,
            calc_rules: DimVec<DimVec<(EdgeCategory, MetricIdx)>>,
        }

        impl Config {
            pub fn new(
                edge_categories: Vec<EdgeCategory>,
                metric_categories: DimVec<EdgeCategory>,
                are_metrics_provided: DimVec<bool>,
                ids: DimVec<SimpleId>,
                indices: BTreeMap<SimpleId, MetricIdx>,
                calc_rules: DimVec<DimVec<(EdgeCategory, MetricIdx)>>,
            ) -> Config {
                Config {
                    edge_categories,
                    metric_categories,
                    are_metrics_provided,
                    ids,
                    indices,
                    calc_rules,
                }
            }
        }

        impl Config {
            pub fn edge_categories(&self) -> &Vec<EdgeCategory> {
                &self.edge_categories
            }

            pub fn metric_category(&self, idx: MetricIdx) -> EdgeCategory {
                match self.metric_categories.get(*idx) {
                    Some(metric_category) => *metric_category,
                    None => {
                        panic!("Idx {} for metric-category not found in config.", idx);
                    }
                }
            }

            pub fn dim(&self) -> usize {
                self.metric_categories.len()
            }

            pub fn is_metric_provided(&self, idx: MetricIdx) -> bool {
                match self.are_metrics_provided.get(*idx) {
                    Some(is_provided) => *is_provided,
                    None => {
                        panic!("Idx {} for info 'is-provided' not found in config.", idx);
                    }
                }
            }

            pub fn metric_idx(&self, id: &SimpleId) -> MetricIdx {
                match self.indices.get(id) {
                    Some(idx) => *idx,
                    None => {
                        panic!("Id {} not found in config.", id);
                    }
                }
            }

            pub fn calc_rules(&self, idx: MetricIdx) -> &DimVec<(EdgeCategory, MetricIdx)> {
                match self.calc_rules.get(*idx) {
                    Some(calc_rule) => calc_rule,
                    None => {
                        panic!("Idx {} for calc-rule not found in config.", idx);
                    }
                }
            }
        }

        /// Types of metrics to consider when parsing a map.
        ///
        /// - `NodeId`, which is not a metric per se and stored differently, but needed for `csv`-like `fmi`-format
        /// - `Meters` provided in meters, but internally stored as kilometers
        /// - `KilometersPerHour` in km/h
        /// - `Seconds`
        /// - `LaneCount`
        /// - `Custom`, which is just the plain f32-value
        /// - `Ignore`, which is used in `csv`-like `fmi`-maps to jump over columns
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

            pub fn expected_calc_rules(&self) -> DimVec<EdgeCategory> {
                match self {
                    EdgeCategory::KilometersPerHour => {
                        smallvec![EdgeCategory::Meters, EdgeCategory::Seconds]
                    }
                    EdgeCategory::Seconds => {
                        smallvec![EdgeCategory::Meters, EdgeCategory::KilometersPerHour]
                    }
                    EdgeCategory::Meters
                    | EdgeCategory::LaneCount
                    | EdgeCategory::Custom
                    | EdgeCategory::NodeId
                    | EdgeCategory::Ignore => smallvec![],
                }
            }
        }
    }
}

pub mod export {
    #[derive(Debug)]
    pub struct Config {
        // TODO implement export:150:Config
    }
}

pub mod routing {
    use crate::{defaults::DimVec, network::MetricIdx};

    #[derive(Debug)]
    pub struct Config {
        indices: DimVec<MetricIdx>,
        alphas: DimVec<f32>,
    }

    impl Config {
        pub fn new(indices: DimVec<MetricIdx>, alphas: DimVec<f32>) -> Config {
            Config { indices, alphas }
        }

        fn _push(&mut self, idx: MetricIdx, alpha: f32) {
            self.indices.push(idx);
            self.alphas.push(alpha);
        }

        pub fn alpha(&self, metric_idx: MetricIdx) -> f32 {
            let idx = match self.indices.iter().position(|i| i == &metric_idx) {
                Some(idx) => idx,
                None => {
                    panic!("Idx {} not found in config.", metric_idx);
                }
            };
            self.alphas[idx]
        }

        pub fn alphas(&self) -> &DimVec<f32> {
            &self.alphas
        }

        pub fn indices(&self) -> &DimVec<MetricIdx> {
            &self.indices
        }

        pub fn dim(&self) -> usize {
            self.indices.len()
        }

        pub fn from_str(
            yaml_str: &str,
            graph_cfg: &super::graph::Config,
        ) -> Result<Config, String> {
            let raw_cfg = super::raw::routing::Config::from_str(yaml_str)?;
            Ok(Config::from_raw(raw_cfg, graph_cfg))
        }

        pub fn from_raw(
            raw_cfg: super::raw::routing::Config,
            graph: &super::graph::Config,
        ) -> Config {
            let (indices, alphas) = raw_cfg
                .entries
                .into_iter()
                .map(|entry| {
                    (
                        graph.edges.metric_idx(&entry.id),
                        entry.alpha.unwrap_or(1.0),
                    )
                })
                .unzip();

            Config::new(indices, alphas)
        }
    }
}

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
