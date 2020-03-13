use crate::{helpers, io::SupportingFileExts};
pub use parser::{edges::EdgeCategory, nodes::NodeCategory};
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
    pub parser: parser::Config,
    pub generator: Option<generator::Config>,
    pub routing: Option<routing::Config>,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, String> {
        let file = {
            Config::find_supported_ext(path)?;
            helpers::open_file(path)?
        };
        match serde_yaml::from_reader(file) {
            Ok(cfg) => Ok(cfg),
            Err(msg) => Err(format!("{}", msg)),
        }
    }
}

pub mod parser {
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
            NodeIdx,
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
        use crate::{configs::SimpleId, defaults::capacity::DimVec, network::MetricIdx};
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
            edge_ids: Vec<SimpleId>,
            // store for quick access
            metric_categories: DimVec<EdgeCategory>,
            are_metrics_provided: DimVec<bool>,
            metric_ids: DimVec<SimpleId>,
            metric_indices: BTreeMap<SimpleId, MetricIdx>,
            calc_rules: DimVec<DimVec<(EdgeCategory, MetricIdx)>>,
        }

        impl Config {
            pub fn new(
                edge_categories: Vec<EdgeCategory>,
                edge_ids: Vec<SimpleId>,
                metric_categories: DimVec<EdgeCategory>,
                are_metrics_provided: DimVec<bool>,
                metric_ids: DimVec<SimpleId>,
                metric_indices: BTreeMap<SimpleId, MetricIdx>,
                calc_rules: DimVec<DimVec<(EdgeCategory, MetricIdx)>>,
            ) -> Config {
                Config {
                    edge_categories,
                    edge_ids,
                    metric_categories,
                    are_metrics_provided,
                    metric_ids,
                    metric_indices,
                    calc_rules,
                }
            }
        }

        impl Config {
            pub fn edge_categories(&self) -> &Vec<EdgeCategory> {
                &self.edge_categories
            }

            /// For metrics, use `metric_category(idx)`, since it is faster.
            pub fn edge_category(&self, id: &SimpleId) -> &EdgeCategory {
                match self.edge_ids.iter().position(|i| i == id) {
                    Some(idx) => &self.edge_categories[idx],
                    None => {
                        panic!("Id {} not found in config.", id);
                    }
                }
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
                match self.metric_indices.get(id) {
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
            SrcId,
            DstId,
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
                    EdgeCategory::Custom
                    | EdgeCategory::SrcId
                    | EdgeCategory::DstId
                    | EdgeCategory::Ignore => false,
                }
            }

            pub fn is_metric(&self) -> bool {
                match self {
                    EdgeCategory::SrcId | EdgeCategory::DstId | EdgeCategory::Ignore => false,
                    EdgeCategory::Meters
                    | EdgeCategory::KilometersPerHour
                    | EdgeCategory::Seconds
                    | EdgeCategory::LaneCount
                    | EdgeCategory::Custom => true,
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
                    | EdgeCategory::SrcId
                    | EdgeCategory::DstId
                    | EdgeCategory::Ignore => smallvec![],
                }
            }
        }
    }
}

pub mod generator {
    use super::{NodeCategory, SimpleId};
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Config {
        pub map_file: PathBuf,
        pub nodes: Vec<NodeCategory>,
        pub edges: Vec<SimpleId>,
    }
}

pub mod routing {
    use crate::{defaults::capacity::DimVec, network::MetricIdx};

    #[derive(Debug)]
    pub struct Config {
        metric_indices: DimVec<MetricIdx>,
        alphas: DimVec<f32>,
    }

    impl Config {
        pub fn new(metric_indices: DimVec<MetricIdx>, alphas: DimVec<f32>) -> Config {
            Config {
                metric_indices,
                alphas,
            }
        }

        fn _push(&mut self, idx: MetricIdx, alpha: f32) {
            self.metric_indices.push(idx);
            self.alphas.push(alpha);
        }

        pub fn alpha(&self, metric_idx: MetricIdx) -> f32 {
            let idx = match self.metric_indices.iter().position(|i| i == &metric_idx) {
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

        pub fn metric_indices(&self) -> &DimVec<MetricIdx> {
            &self.metric_indices
        }

        pub fn dim(&self) -> usize {
            self.metric_indices.len()
        }

        pub fn from_str(
            yaml_str: &str,
            cfg_graph: &super::parser::Config,
        ) -> Result<Config, String> {
            let raw_cfg = super::raw::routing::Config::from_str(yaml_str)?;
            Ok(Config::from_raw(raw_cfg, cfg_graph))
        }

        pub fn from_raw(
            raw_cfg: super::raw::routing::Config,
            cfg_parser: &super::parser::Config,
        ) -> Config {
            let (metric_indices, alphas) = raw_cfg
                .entries
                .into_iter()
                .map(|entry| {
                    (
                        cfg_parser.edges.metric_idx(&entry.id),
                        entry.alpha.unwrap_or(1.0),
                    )
                })
                .unzip();

            Config::new(metric_indices, alphas)
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
