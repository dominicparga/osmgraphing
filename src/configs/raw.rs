use crate::{
    configs::{EdgeCategory, SimpleId},
    defaults::DimVec,
    network::MetricIdx,
};
use log::warn;
use serde::Deserialize;
use smallvec::smallvec;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub parser: parser::Config,
    pub generator: Option<generator::Config>,
    #[serde(flatten)]
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
        use crate::configs::parser::nodes::NodeCategory;
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
        use crate::configs::{parser::edges::EdgeCategory, SimpleId};
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
        use crate::configs::parser::nodes::NodeCategory;
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
    pub struct Config {
        #[serde(rename = "routing")]
        pub entries: Vec<Entry>,
    }

    impl Config {
        pub fn from_str(yaml_str: &str) -> Result<Config, String> {
            /// This is needed to support the root-key `routing`:
            ///
            /// ```yaml
            /// routing: ...
            /// ```
            #[derive(Deserialize)]
            struct Wrapper {
                #[serde(rename = "routing")]
                entries: Vec<Entry>,
            }

            let wrapper: Result<Wrapper, _> = serde_yaml::from_str(yaml_str);
            match wrapper {
                Ok(wrapper) => Ok(Config {
                    entries: wrapper.entries,
                }),
                Err(e) => Err(format!("{}", e)),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Entry {
        pub id: SimpleId,
        pub alpha: Option<f32>,
    }
}

impl From<Config> for super::Config {
    fn from(raw_cfg: Config) -> super::Config {
        //----------------------------------------------------------------------------------------//
        // build super::parser::Config

        let cfg_parser = {
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
            if nodes.categories().contains(&super::NodeCategory::NodeIdx) {
                warn!(
                    "The config for parser::nodes contains the node-category {:?}, \
                     which is ignored.",
                    super::NodeCategory::NodeIdx
                );
            }

            // build super::parser::edges::Config
            let edges = {
                // init datastructures
                let mut edge_categories = Vec::with_capacity(raw_cfg.parser.edges.len());
                let mut edge_ids = Vec::with_capacity(raw_cfg.parser.edges.len());
                let mut metric_categories = DimVec::new();
                let mut metric_ids = DimVec::new();
                let mut are_metrics_provided = DimVec::new();
                let mut metric_indices = BTreeMap::new();
                let mut proto_calc_rules = DimVec::new();

                // Fill categories, ids and whether type is provided.
                // Further, create mapping: id -> idx.
                for entry in raw_cfg.parser.edges.into_iter() {
                    edge_categories.push(entry.category);
                    // add id if no duplicate
                    let entry_id = match entry.id {
                        Some(entry_id) => entry_id,
                        None => SimpleId(format!("{}", entry.category)),
                    };
                    // check whether id is duplicate, but ids of ignore are ignored :3
                    if entry.category != EdgeCategory::Ignore && edge_ids.contains(&entry_id) {
                        panic!("Config has duplicate id: {}", entry_id);
                    }
                    edge_ids.push(entry_id.clone());

                    // add metrics separatedly for better access-performance through metric-indices
                    if entry.category.is_metric() {
                        metric_ids.push(entry_id.clone());
                        metric_categories.push(entry.category);
                        are_metrics_provided.push(entry.is_provided.unwrap_or(true));

                        let metric_idx = MetricIdx(metric_indices.len());
                        metric_indices.insert(entry_id.clone(), metric_idx);
                        proto_calc_rules.push(entry.calc_rules);
                    } else {
                        if entry.calc_rules.is_some() {
                            panic!(
                                "Metric-category {} has calculation-rules given, \
                                 but is ignored and hence should not have any calculation-rule.",
                                entry.category
                            );
                        }
                    }
                }

                // add calculation-rules after everything else is already finished
                let mut calc_rules: DimVec<_> = smallvec![DimVec::new(); metric_categories.len()];
                for (metric_idx, opt_calc_rule) in proto_calc_rules.into_iter().enumerate() {
                    if let Some(calc_rule) = opt_calc_rule {
                        // implement as given
                        for other_id in calc_rule.into_iter() {
                            let other_idx = match metric_indices.get(&other_id) {
                                Some(idx) => *idx,
                                None => {
                                    panic!(
                                        "Calc-rule for metric of id {} has an unknown id {}.",
                                        metric_ids[metric_idx], other_id
                                    );
                                }
                            };
                            let other_type = metric_categories[*other_idx];
                            calc_rules[metric_idx].push((other_type, other_idx));
                        }
                    }

                    // check calc-rules for correctness
                    let category = metric_categories[metric_idx];
                    let expected_categories = category.expected_calc_rules();
                    // if no rules are provided -> error
                    // but if the value itself is provided -> no error
                    if calc_rules[metric_idx].len() == 0 && are_metrics_provided[metric_idx] {
                        continue;
                    }
                    if calc_rules[metric_idx].len() != expected_categories.len() {
                        panic!(
                            "Metric of category {} has {} calculation-rules, but should have {}.",
                            category,
                            calc_rules[metric_idx].len(),
                            expected_categories.len()
                        );
                    }
                    for expected_category in expected_categories.iter() {
                        if calc_rules[metric_idx]
                            .iter()
                            .map(|cr| cr.0)
                            .find(|c| c == expected_category)
                            .is_none()
                        {
                            panic!(
                                "Calculation-rules of metric-category {} \
                                 should contain {:?}, but doesn't.",
                                category, expected_categories
                            );
                        }
                    }
                }

                super::parser::edges::Config::new(
                    edge_categories,
                    edge_ids,
                    metric_categories,
                    are_metrics_provided,
                    metric_ids,
                    metric_indices,
                    calc_rules,
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
                &cfg_parser,
            )),
            None => None,
        };

        //----------------------------------------------------------------------------------------//
        // return finished config

        super::Config {
            parser: cfg_parser,
            generator: cfg_generator,
            routing: cfg_routing,
        }
    }
}
