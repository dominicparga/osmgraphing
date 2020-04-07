use crate::{configs::SimpleId, defaults::capacity::DimVec};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub parser: parser::Config,
    pub generator: Option<generator::Config>,
    pub routing: Option<routing::Config>,
}

pub mod parser {
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub map_file: PathBuf,
        pub vehicles: vehicles::Config,
        #[serde(flatten)]
        pub nodes: nodes::Config,
        pub edges: Vec<edges::Entry>,
    }

    pub mod vehicles {
        use crate::network::VehicleCategory;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct Config {
            pub category: VehicleCategory,
            pub are_drivers_picky: bool,
        }
    }

    pub mod nodes {
        use crate::configs::parser::NodeCategory;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Config {
            #[serde(rename = "nodes")]
            pub categories: Vec<Entry>,
        }

        #[derive(Debug, Deserialize)]
        pub struct Entry {
            pub category: NodeCategory,
        }
    }

    pub mod edges {
        use crate::configs::{parser::EdgeCategory, SimpleId};
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct Entry {
            pub category: EdgeCategory,
            pub id: Option<SimpleId>,
            pub is_provided: Option<bool>,
            pub calc_rules: Option<Vec<SimpleId>>,
        }
    }
}

pub mod generator {
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub map_file: PathBuf,
        pub nodes: Vec<nodes::Entry>,
        pub edges: Vec<edges::Entry>,
    }

    pub mod nodes {
        use crate::configs::generator::NodeCategory;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Entry {
            pub category: NodeCategory,
        }
    }

    pub mod edges {
        use crate::configs::SimpleId;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Entry {
            pub id: SimpleId,
        }
    }
}

pub mod routing {
    use crate::configs::SimpleId;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub is_ch_dijkstra: Option<bool>,
        pub metrics: Vec<Entry>,
    }

    impl Config {
        pub fn from_str(yaml_str: &str) -> Result<Config, String> {
            #[derive(Deserialize)]
            struct Wrapper {
                routing: Config,
            }

            let wrapper: Result<Wrapper, _> = serde_yaml::from_str(yaml_str);
            match wrapper {
                Ok(wrapper) => Ok(wrapper.routing),
                Err(e) => Err(format!("{}", e)),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Entry {
        pub id: SimpleId,
        pub alpha: Option<f64>,
    }
}

impl From<Config> for super::Config {
    fn from(raw_cfg: Config) -> super::Config {
        //----------------------------------------------------------------------------------------//
        // build super::parser::Config

        let parser_cfg = {
            // build super::parser::vehicles::Config
            let vehicles = super::parser::vehicles::Config {
                category: raw_cfg.parser.vehicles.category,
                are_drivers_picky: raw_cfg.parser.vehicles.are_drivers_picky,
            };

            // build super::parser::nodes::Config
            let nodes = super::parser::nodes::Config::new(
                raw_cfg
                    .parser
                    .nodes
                    .categories
                    .into_iter()
                    .map(|entry| entry.category)
                    .collect(),
            );

            // build super::parser::edges::Config
            let edges = {
                // init datastructures
                let mut categories = Vec::with_capacity(raw_cfg.parser.edges.len());
                let mut ids = Vec::with_capacity(raw_cfg.parser.edges.len());
                let mut metric_categories = DimVec::new();
                let mut metric_ids = DimVec::new();

                // Fill categories, ids and whether type is provided.
                // Further, create mapping: id -> idx.
                for entry in raw_cfg.parser.edges.into_iter() {
                    categories.push(entry.category);

                    // add id if no duplicate

                    let entry_id = match entry.id {
                        Some(entry_id) => entry_id,
                        None => SimpleId(format!("{}", entry.category)),
                    };

                    // Allow only category Ignore to be called 'Ignore'
                    // to allow multiple ignored values without the need of an id.
                    // Further check whether id is duplicate, but ids of ignore are ignored :3
                    if entry.category != super::parser::EdgeCategory::Ignore {
                        if ids.contains(&entry_id)
                            || entry_id
                                == SimpleId(format!("{}", super::parser::EdgeCategory::Ignore))
                        {
                            panic!("Config has duplicate id: {}", entry_id);
                        }
                    }
                    ids.push(entry_id.clone());

                    // add metrics separatedly for better access-performance through metric-indices
                    if entry.category.is_metric() {
                        metric_ids.push(entry_id.clone());
                        metric_categories.push(entry.category);
                    }
                }

                super::parser::edges::Config::new(
                    categories,
                    ids,
                    super::parser::edges::metrics::Config {
                        categories: metric_categories,
                        ids: metric_ids,
                    },
                )
            };

            super::parser::Config {
                map_file: raw_cfg.parser.map_file,
                vehicles: vehicles,
                nodes: nodes,
                edges: edges,
            }
        };

        //----------------------------------------------------------------------------------------//
        // build super::export::Config

        let cfg_generator = match raw_cfg.generator {
            Some(raw_cfg_generator) => Some(super::generator::Config {
                map_file: raw_cfg_generator.map_file,
                nodes: raw_cfg_generator
                    .nodes
                    .into_iter()
                    .map(|entry| entry.category)
                    .collect(),
                edges: raw_cfg_generator
                    .edges
                    .into_iter()
                    .map(|entry| entry.id)
                    .collect(),
            }),
            None => None,
        };

        //----------------------------------------------------------------------------------------//
        // build super::routing::Config

        let cfg_routing = match raw_cfg.routing {
            Some(raw_cfg_routing) => Some(super::routing::Config::from_raw(
                raw_cfg_routing,
                &parser_cfg,
            )),
            None => None,
        };

        //----------------------------------------------------------------------------------------//
        // return finished config

        super::Config {
            parser: parser_cfg,
            generator: cfg_generator,
            routing: cfg_routing,
        }
    }
}
