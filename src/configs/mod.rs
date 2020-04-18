use crate::{helpers, io::SupportingFileExts};
use serde::Deserialize;
use std::{fmt, fmt::Display, path::Path};

pub mod categories;
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
/// Internally, a default-metric uses provided calculation-rules to be calculated by other default-categories as well (like the duration from distance and maxspeed).
///
/// Keep in mind, that metrics (except for id) are stored as `f64` for better maintainability and efficiency.
///
///
/// ### Specifying routing (in the future)
///
/// Further, the metrics, which are used in the routing, can be listed in the routing-section with their previously defined id.
/// Comparisons are made using pareto-optimality, so there is no comparison between metrics.
/// In case you'll use personlized-routing, default-preferences can be set with weights.
/// The example below shows a routing-case, where the metric `distance` is weighted with `169 / (169 + 331) = 33.8 %` while the metric `duration` is weighted with `331 / (169 + 331) = 66.2 %`.
///
///
/// ### Supported structure
///
/// The supported `yaml`-structure can be seen in `resources/configs/schema.yaml`.
///
// Every metric (!= every category) will be stored in the graph, if mentioned in this `yaml`-file.
/// If a metric is mentioned, but `provided` is false, it will be calculated (e.g. edge-distance from node-coordinates and haversine).
/// Please note, that metrics being calculated (like the duration from distance and maxspeed) need the respective metrics to be calculated.
#[derive(Debug, Deserialize)]
#[serde(from = "raw::Config")]
pub struct Config {
    pub parsing: parsing::Config,
    pub writing: Option<writing::Config>,
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

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        // build sub-cfgs

        let parsing_cfg = parsing::Config::from(raw_cfg.parsing);

        let writing_cfg = match raw_cfg.writing {
            Some(raw_writing_cfg) => Some(writing::Config::from(raw_writing_cfg)),
            None => None,
        };

        let routing_cfg = match raw_cfg.routing {
            Some(raw_routing_cfg) => Some(routing::Config::from_raw(raw_routing_cfg, &parsing_cfg)),
            None => None,
        };

        // finish cfg

        Config {
            parsing: parsing_cfg,
            writing: writing_cfg,
            routing: routing_cfg,
        }
    }
}

pub mod parsing {
    use crate::configs::raw::parsing as raw;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Config {
        pub map_file: PathBuf,
        pub vehicles: vehicles::Config,
        pub nodes: nodes::Config,
        pub edges: edges::Config,
        pub generating: Option<generating::Config>,
    }

    impl From<raw::Config> for Config {
        fn from(raw_cfg: raw::Config) -> Config {
            Config {
                map_file: raw_cfg.map_file,
                vehicles: raw_cfg.vehicles.into(),
                nodes: raw_cfg.nodes.into(),
                edges: raw_cfg.edges.into(),
                generating: match raw_cfg.generating {
                    Some(generating_cfg) => Some(generating_cfg.into()),
                    None => None,
                },
            }
        }
    }

    pub mod vehicles {
        use crate::configs::raw::parsing::vehicles as raw;
        use crate::network::vehicles::Category as VehicleCategory;

        #[derive(Debug)]
        pub struct Config {
            pub category: VehicleCategory,
            pub are_drivers_picky: bool,
        }

        impl From<raw::Config> for Config {
            fn from(raw_cfg: raw::Config) -> Self {
                Config {
                    category: raw_cfg.category,
                    are_drivers_picky: raw_cfg.are_drivers_picky,
                }
            }
        }
    }

    pub mod nodes {
        use crate::configs::{categories::nodes, raw::parsing::nodes as raw};

        #[derive(Debug)]
        pub struct Config {
            pub categories: Vec<nodes::Category>,
        }

        impl From<raw::Config> for Config {
            fn from(raw_cfg: raw::Config) -> Config {
                let mut categories: Vec<nodes::Category> = Vec::new();

                for category in raw_cfg.0.into_iter() {
                    categories.push(category.into());
                }

                Config { categories }
            }
        }
    }

    pub mod edges {
        use crate::{
            configs::{categories::edges, raw::parsing::edges as raw},
            defaults::capacity::DimVec,
        };

        #[derive(Debug)]
        pub struct Config {
            // store all for order
            pub categories: Vec<edges::Category>,

            // store only metrics for quick access
            pub metrics: metrics::Config,
        }

        pub mod metrics {
            use crate::{
                configs::{categories::edges, SimpleId},
                defaults::capacity::DimVec,
            };

            #[derive(Debug)]
            pub struct Config {
                pub units: DimVec<edges::UnitInfo>,
                pub ids: DimVec<SimpleId>,
                // are_metrics_provided: DimVec<bool>,
                // calc_rules: DimVec<DimVec<(Category, MetricIdx)>>,
            }
        }

        impl From<raw::Config> for Config {
            fn from(raw_cfg: raw::Config) -> Config {
                // init datastructures

                let mut categories = Vec::with_capacity(raw_cfg.0.len());
                let mut metric_units = DimVec::new();
                let mut metric_ids = DimVec::new();

                // check if any id is duplicate

                for i in 0..raw_cfg.0.len() {
                    // get i-th id

                    let id_i = {
                        match &raw_cfg.0[i] {
                            raw::Category::Ignored => continue,
                            raw::Category::Meta { info: _, id: id_i }
                            | raw::Category::Metric { unit: _, id: id_i } => id_i,
                        }
                    };

                    for j in (i + 1)..raw_cfg.0.len() {
                        // get j-th id

                        let id_j = {
                            match &raw_cfg.0[j] {
                                raw::Category::Ignored => continue,
                                raw::Category::Meta { info: _, id: id_j }
                                | raw::Category::Metric { unit: _, id: id_j } => id_j,
                            }
                        };

                        // compare both ids

                        if id_i == id_j {
                            panic!("Config has duplicate id: {}", id_i);
                        }
                    }
                }

                // Fill categories, ids and create mapping: id -> idx

                for category in raw_cfg.0.into_iter() {
                    // add category

                    match &category {
                        // add metrics separatedly
                        // for better access-performance through metric-indices
                        raw::Category::Metric { unit, id } => {
                            categories.push(category.clone().into());
                            metric_units.push(unit.clone().into());
                            metric_ids.push(id.clone());
                        }
                        raw::Category::Meta { info: _, id: _ } | raw::Category::Ignored => {
                            categories.push(category.clone().into())
                        }
                    }
                }

                Config {
                    categories,
                    metrics: metrics::Config {
                        units: metric_units,
                        ids: metric_ids,
                    },
                }
            }
        }
    }

    pub mod generating {
        use crate::configs::raw::parsing::generating as raw;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct Config {
            pub nodes: nodes::Config,
            pub edges: edges::Config,
        }

        impl From<raw::Config> for Config {
            fn from(raw_cfg: raw::Config) -> Config {
                Config {
                    nodes: nodes::Config {
                        categories: raw_cfg
                            .nodes
                            .0
                            .into_iter()
                            .map(|raw_category| raw_category.into())
                            .collect(),
                    },
                    edges: edges::Config {
                        categories: raw_cfg
                            .edges
                            .0
                            .into_iter()
                            .map(|raw_category| raw_category.into())
                            .collect(),
                    },
                }
            }
        }

        pub mod nodes {
            use crate::configs::categories::nodes;
            use serde::Deserialize;

            #[derive(Debug, Deserialize)]
            pub struct Config {
                pub categories: Vec<nodes::Category>,
            }
        }

        pub mod edges {
            use crate::configs::categories::edges;
            use serde::Deserialize;

            #[derive(Debug, Deserialize)]
            pub struct Config {
                pub categories: Vec<edges::Category>,
            }
        }
    }
}

pub mod writing {
    use crate::configs::raw::writing as raw;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Config {
        pub map_file: PathBuf,
        pub nodes: nodes::Config,
        pub edges: edges::Config,
    }

    impl From<raw::Config> for Config {
        fn from(raw_cfg: raw::Config) -> Config {
            Config {
                map_file: raw_cfg.map_file,
                nodes: nodes::Config::from(raw_cfg.nodes),
                edges: edges::Config::from(raw_cfg.edges),
            }
        }
    }

    pub mod nodes {
        use crate::configs::{raw::writing::nodes as raw, SimpleId};

        #[derive(Debug)]
        pub struct Config {
            pub ids: Vec<Option<SimpleId>>,
        }

        impl From<raw::Config> for Config {
            fn from(raw_cfg: raw::Config) -> Config {
                Config {
                    ids: raw_cfg
                        .0
                        .into_iter()
                        .map(|category| match category {
                            raw::Category::Id(id) => Some(id),
                            raw::Category::Ignored => None,
                        })
                        .collect(),
                }
            }
        }
    }

    pub mod edges {
        use crate::configs::{raw::writing::edges as raw, SimpleId};

        #[derive(Debug)]
        pub struct Config {
            pub ids: Vec<Option<SimpleId>>,
        }

        impl From<raw::Config> for Config {
            fn from(raw_cfg: raw::Config) -> Config {
                Config {
                    ids: raw_cfg
                        .0
                        .into_iter()
                        .map(|category| match category {
                            raw::Category::Id(id) => Some(id),
                            raw::Category::Ignored => None,
                        })
                        .collect(),
                }
            }
        }
    }
}

pub mod routing {
    use crate::{
        configs::{parsing, raw::routing as raw},
        defaults::{self, capacity::DimVec},
    };
    use smallvec::smallvec;

    #[derive(Clone, Debug)]
    pub struct Config {
        pub is_ch_dijkstra: bool,
        pub alphas: DimVec<f64>,
    }

    impl Config {
        pub fn from_str(yaml_str: &str, parsing_cfg: &parsing::Config) -> Result<Config, String> {
            let raw_cfg = raw::Config::from_str(yaml_str)?;
            Ok(Config::from_raw(raw_cfg, parsing_cfg))
        }

        pub fn from_raw(raw_cfg: raw::Config, parsing_cfg: &parsing::Config) -> Config {
            let mut alphas = smallvec![0.0; parsing_cfg.edges.metrics.units.len()];

            for entry in raw_cfg.metrics.into_iter() {
                let alpha = entry.alpha.unwrap_or(defaults::routing::ALPHA);

                if let Some(metric_idx) = parsing_cfg
                    .edges
                    .metrics
                    .ids
                    .iter()
                    .position(|id| id == &entry.id)
                {
                    alphas[metric_idx] = alpha;
                } else {
                    panic!(
                        "The given id {} should get alpha {}, but doesn't exist.",
                        entry.id, alpha
                    );
                }
            }

            Config {
                is_ch_dijkstra: raw_cfg.is_ch_dijkstra.unwrap_or(false),
                alphas,
            }
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
