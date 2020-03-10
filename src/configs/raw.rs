use crate::{configs::SimpleId, defaults::DimVec, network::MetricIdx};
use serde::Deserialize;
use smallvec::smallvec;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub graph: graph::Config,
    // pub export: export::Config,
    #[serde(flatten)]
    pub routing: Option<routing::Config>,
}

pub mod graph {
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub map_file: PathBuf,
        pub vehicles: vehicles::Config,
        // pub nodes: Vec<nodes::Entry>,
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
        use crate::configs::graph::nodes::NodeCategory;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Entry {
            category: NodeCategory,
        }
    }

    pub mod edges {
        use crate::configs::{graph::edges::EdgeCategory, SimpleId};
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

pub mod export {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub nodes: Vec<nodes::Entry>,
        pub edges: Vec<edges::Entry>,
    }

    pub mod nodes {
        use crate::configs::SimpleId;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Entry {
            pub id: SimpleId,
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
        // build super::graph::Config

        let graph = {
            // build super::graph::vehicles::Config
            let vehicles = super::graph::vehicles::Config {
                category: raw_cfg.graph.vehicles.category,
                are_drivers_picky: raw_cfg.graph.vehicles.are_drivers_picky,
            };

            // build super::graph::nodes::Config
            let nodes = super::graph::nodes::Config {};

            // build super::graph::edges::Config
            let edges = {
                // init datastructures
                let mut all_categories = Vec::with_capacity(raw_cfg.graph.edges.len());
                let mut categories = DimVec::new();
                let mut ids = DimVec::new();
                let mut are_provided = DimVec::new();
                let mut indices = BTreeMap::new();
                let mut proto_calc_rules = DimVec::new();

                // Fill categories, ids and whether type is provided.
                // Further, create mapping: id -> idx.
                for entry in raw_cfg.graph.edges.into_iter() {
                    all_categories.push(entry.category);

                    if entry.category.is_ignored() {
                        if entry.calc_rules.is_some() {
                            panic!(
                                "Metric-category {} has calculation-rules given, \
                                 but is ignored and hence should not have any calculation-rule.",
                                entry.category
                            );
                        }
                    } else {
                        let entry_id = match entry.id {
                            Some(entry_id) => entry_id,
                            None => SimpleId(format!("{}", entry.category)),
                        };
                        ids.push(entry_id.clone());
                        categories.push(entry.category);
                        are_provided.push(entry.is_provided.unwrap_or(true));

                        let metric_idx = MetricIdx(indices.len());
                        if indices.insert(entry_id.clone(), metric_idx).is_some() {
                            panic!("Config has duplicate id: {}", entry_id);
                        }
                        proto_calc_rules.push(entry.calc_rules);
                    }
                }

                // add calculation-rules after everything else is already finished
                let mut calc_rules: DimVec<_> = smallvec![DimVec::new(); categories.len()];
                for (metric_idx, opt_calc_rule) in proto_calc_rules.into_iter().enumerate() {
                    if let Some(calc_rule) = opt_calc_rule {
                        // implement as given
                        for other_id in calc_rule.into_iter() {
                            let other_idx = match indices.get(&other_id) {
                                Some(idx) => *idx,
                                None => {
                                    panic!(
                                        "Calc-rule for metric of id {} has an unknown id {}.",
                                        ids[metric_idx], other_id
                                    );
                                }
                            };
                            let other_type = categories[*other_idx];
                            calc_rules[metric_idx].push((other_type, other_idx));
                        }
                    }

                    // check calc-rules for correctness
                    let category = categories[metric_idx];
                    let expected_categories = category.expected_calc_rules();
                    // if no rules are provided -> error
                    // but if the value itself is provided -> no error
                    if calc_rules[metric_idx].len() == 0 && are_provided[metric_idx] {
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
                            panic!("Calculation-rules of metric-category {} should contain {:?}, but doesn't.", category, expected_categories);
                        }
                    }
                }

                super::graph::edges::Config::new(
                    all_categories,
                    categories,
                    are_provided,
                    ids,
                    indices,
                    calc_rules,
                )
            };

            super::graph::Config {
                map_file: raw_cfg.graph.map_file,
                vehicles: vehicles,
                nodes: nodes,
                edges: edges,
            }
        };

        //----------------------------------------------------------------------------------------//
        // build super::export::Config

        let export = super::export::Config {};

        //----------------------------------------------------------------------------------------//
        // build super::routing::Config

        let routing = match raw_cfg.routing {
            Some(routing) => Some(super::routing::Config::from_raw(routing, &graph)),
            None => None,
        };

        //----------------------------------------------------------------------------------------//
        // return finished config

        super::Config {
            graph,
            export,
            routing,
        }
    }
}
