use std::{fmt, fmt::Display};

pub mod constants {
    pub mod ids {
        pub const ID: &'static str = "id";
        pub const SRC: &'static str = "src";
        pub const DST: &'static str = "dst";
        pub const LENGTH: &'static str = "length";
        pub const MAXSPEED: &'static str = "maxspeed";
        pub const DURATION: &'static str = "duration";
        pub const LANE_COUNT: &'static str = "lane-count";
        pub const CUSTOM: &'static str = "custom";
        pub const IGNORE: &'static str = "ignore";
        pub const UNKNOWN: &'static str = "?";
    }
}

#[derive(Debug)]
pub enum VehicleType {
    Car,
    Bicycle,
    Pedestrian,
}

/// Types of metrics to consider when parsing a map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetricType {
    Id { id: String },
    Length { provided: bool },
    Maxspeed { provided: bool },
    Duration { provided: bool },
    LaneCount,
    Custom { id: String },
    Ignore { id: String },
}

impl MetricType {
    pub fn id(&self) -> &str {
        match self {
            MetricType::Id { id } => &id,
            MetricType::Length { provided: _ } => constants::ids::LENGTH,
            MetricType::Maxspeed { provided: _ } => constants::ids::MAXSPEED,
            MetricType::Duration { provided: _ } => constants::ids::DURATION,
            MetricType::LaneCount => constants::ids::LANE_COUNT,
            MetricType::Custom { id } => &id,
            MetricType::Ignore { id } => &id,
        }
    }
}

impl Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetricType::Id { id } => format!("{}({})", constants::ids::ID, id),
                MetricType::Length { provided } =>
                    format!("{} (provided: {})", constants::ids::LENGTH, provided),
                MetricType::Maxspeed { provided } =>
                    format!("{} (provided: {})", constants::ids::MAXSPEED, provided),
                MetricType::Duration { provided } =>
                    format!("{} (provided: {})", constants::ids::DURATION, provided),
                MetricType::LaneCount => format!("{}", constants::ids::LANE_COUNT),
                MetricType::Custom { id } => format!("{}({})", constants::ids::CUSTOM, id),
                MetricType::Ignore { id } => format!("{}({})", constants::ids::IGNORE, id),
            }
        )
    }
}

pub mod graph {
    use super::{edges, paths, VehicleType};

    /// Storing (default) settings for parsing the graph.
    ///
    /// # Configuration
    ///
    /// ## Default
    ///
    /// The default-configuration contains basic metrics of the graph:
    /// - length (in meters)
    /// - maxspeed (in km/h)
    ///
    ///
    /// ## Changing the defaults with yaml-file
    ///
    /// You can change the configuration with an input-file (`*.yaml`).
    /// With this `yaml`-config, the parser can be adjusted to parse (edge-)metrics in the order as provided by the config-file.
    /// This can help especially with map-files in `fmi`-format, since the metrics are read sequentially.
    /// But since `pbf`-files does not provide a column-based metric-list, but intrinsically by parsing `osm`-data, you can distinguish between default-metrics and custom-metrics via the key `type`.
    ///
    /// Default-types are
    /// - `id`, which is not a metric per se and stored differently, but needed for `csv`-like `fmi`-format
    /// - `length` in meters
    /// - `maxspeed` in km/h
    /// - `duration` in milliseconds
    /// - `lane-count`
    ///
    /// Internally, a default-metric uses its type as id and thus can be calculated by other default-types as well (like the duration from length and maxspeed).
    /// In case you are using a custom metric, you must specify an id.
    ///
    /// Keep in mind, that metrics (except for id) are stored as `u32` for better maintainability and efficiency.
    /// Note that you can convert `floats` into `u32` by moving the comma.
    ///
    ///
    /// ### Specifying routing
    ///
    /// Further, the metrics, which are used in the routing, can be listed in the routing-section with their previously defined id (or default-id via `type`).
    /// Comparisons are made using pareto-optimality, so there is no comparison between metrics.
    /// In case you'll use personlized-routing, default-preferences can be set with weights.
    /// The example below shows a routing-case, where the metric `length` is weighted with `169 / (169 + 331) = 33.8 %` while the metric `duration` is weighted with `331 / (169 + 331) = 66.2 %`.
    ///
    ///
    /// ### Supported structure
    ///
    /// The following `yaml`-structure is supported.
    /// The used values below are not the defaults.
    /// For the defaults, see `resources/configs/default.yaml`.
    ///
    /// Please note, that just a few metric-types can be used multiple times, namely:
    /// - type: `id`
    /// - type: `custom`
    /// - type: `ignore`
    ///
    ///
    /// Every metric will be stored in the graph, if mentioned in this `yaml`-file.
    /// If a metric is mentioned, but `provided` is false, it will be calculated (e.g. edge-length from node-coordinates and haversine).
    /// Please note, that metrics being calculated (like the duration from length and maxspeed) need the respective metrics to be calculated.
    ///
    /// ```yaml
    /// graph:
    ///   vehicles:
    ///     # car|bicycle|pedestrian
    ///     type: car
    ///     # Possible values: true|false
    ///     # Value `false` leads to more edges, because edges are added, which are okay but not suitable for this vehicle-type.
    ///     is-graph-suitable: false
    ///   edges:
    ///     # The order here matters if the map-file has a metric-order, like `fmi`-files.
    ///     # Each metric below will be stored in the graph.
    ///     # Metrics below, which have `provided=false`, will be calculated by other metrics and the result is being stored.
    ///     # All other metrics are calculated, if possible, when asked for.
    ///     #
    ///     # Default metrics are length and maxspeed.
    ///     metrics:
    ///     # not a metric per se but needed here due to `csv`-like `fmi`-format
    ///     - id: src-id
    ///       type: id
    ///     - type: length
    ///       # Possible values: true|false
    ///       # Value `false` leads to calculate the value via coordinates and haversine.
    ///       provided: false
    ///     - type: maxspeed
    ///       # Possible values: true|false
    ///       # Value `false` leads to calculate the value via length and duration.
    ///       provided: true
    ///     - type: duration
    ///       # Possible values: true|false
    ///       # Value `false` leads to calculate the value via length and maxspeed.
    ///       provided: false
    ///     - type: lane-count
    ///     - id: <String>
    ///       type: custom
    ///     - id: <String>
    ///       type: ignore
    ///
    /// routing: # example with two metrics and default-weights
    ///   metrics: [length, duration]
    ///   preferences:
    ///   - id: length
    ///     alpha: 169
    ///   - id: duration
    ///     alpha: 331
    /// ```
    pub struct Config {
        pub vehicle_type: VehicleType,
        pub edges: edges::Config,
        pub paths: paths::Config,
        pub is_graph_suitable: bool,
    }
}

pub mod edges {
    use super::MetricType;

    pub struct Config {
        pub metric_types: Vec<MetricType>,
    }

    impl Config {
        pub fn get(&self, idx: usize) -> Option<&MetricType> {
            Some(self.metric_types.get(idx)?)
        }

        pub fn push(&mut self, metric_type: MetricType) {
            self.metric_types.push(metric_type);
        }

        pub fn remove(&mut self, idx: usize) -> MetricType {
            self.metric_types.remove(idx)
        }
    }
}

pub mod paths {
    use std::path::PathBuf;

    pub struct Config {
        pub map_file: PathBuf,
    }
}

pub mod routing {
    pub struct Config {}
}
