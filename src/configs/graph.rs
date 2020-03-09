use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub map_file: PathBuf,
    pub vehicles: vehicles::Config,
    pub edges: edges::Config,
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
}

pub mod edges {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub metrics: metrics::Config,
    }

    pub mod metrics {
        use crate::{
            configs::{EdgeCategory, MetricId},
            network::MetricIdx,
        };
        use serde::Deserialize;
        use std::collections::BTreeMap;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct Entry {
            pub category: EdgeCategory,
            pub id: Option<MetricId>,
            pub is_provided: Option<bool>,
            pub calc_rules: Option<Vec<MetricId>>,
        }

        impl From<(EdgeCategory, MetricId, bool)> for Entry {
            fn from((category, id, is_provided): (EdgeCategory, MetricId, bool)) -> Entry {
                Entry {
                    category,
                    id: Some(id),
                    is_provided: Some(is_provided),
                    calc_rules: None,
                }
            }
        }

        impl From<(EdgeCategory, MetricId, bool, Vec<MetricId>)> for Entry {
            fn from(
                (category, id, is_provided, calc_rules): (
                    EdgeCategory,
                    MetricId,
                    bool,
                    Vec<MetricId>,
                ),
            ) -> Entry {
                Entry {
                    category,
                    id: Some(id),
                    is_provided: Some(is_provided),
                    calc_rules: Some(calc_rules),
                }
            }
        }

        #[derive(Debug, Deserialize)]
        #[serde(from = "Vec<Entry>")]
        pub struct Config {
            // store for order
            all_categories: Vec<EdgeCategory>,
            // store for quick access
            categories: Vec<EdgeCategory>,
            are_provided: Vec<bool>,
            indices: BTreeMap<MetricId, MetricIdx>,
            ids: Vec<MetricId>,
            calc_rules: Vec<Vec<(EdgeCategory, MetricIdx)>>,
        }

        impl Config {
            pub fn all_categories(&self) -> &Vec<EdgeCategory> {
                &self.all_categories
            }

            pub fn category(&self, idx: MetricIdx) -> EdgeCategory {
                match self.categories.get(*idx) {
                    Some(category) => *category,
                    None => {
                        panic!("Idx {} for category not found in config.", idx);
                    }
                }
            }

            pub fn count(&self) -> usize {
                self.categories.len()
            }

            pub fn is_provided(&self, idx: MetricIdx) -> bool {
                match self.are_provided.get(*idx) {
                    Some(is_provided) => *is_provided,
                    None => {
                        panic!("Idx {} for info 'is-provided' not found in config.", idx);
                    }
                }
            }

            pub fn idx(&self, id: &MetricId) -> MetricIdx {
                match self.indices.get(id) {
                    Some(idx) => *idx,
                    None => {
                        panic!("Id {} not found in config.", id);
                    }
                }
            }

            pub fn id(&self, idx: MetricIdx) -> &MetricId {
                match self.ids.get(*idx) {
                    Some(id) => id,
                    None => {
                        panic!("Idx {} for metric-id not found in config.", idx);
                    }
                }
            }

            pub fn calc_rules(&self, idx: MetricIdx) -> &Vec<(EdgeCategory, MetricIdx)> {
                match self.calc_rules.get(*idx) {
                    Some(calc_rule) => calc_rule,
                    None => {
                        panic!("Idx {} for calc-rule not found in config.", idx);
                    }
                }
            }
        }

        impl From<Vec<Entry>> for Config {
            fn from(metrics: Vec<Entry>) -> Config {
                // init datastructures
                let mut all_categories = Vec::with_capacity(metrics.len());
                let mut categories = Vec::with_capacity(metrics.len());
                let mut ids = Vec::with_capacity(metrics.len());
                let mut are_provided = Vec::with_capacity(metrics.len());
                let mut indices = BTreeMap::new();
                let mut proto_calc_rules = Vec::with_capacity(metrics.len());

                // Fill categories, ids and whether type is provided.
                // Further, create mapping: id -> idx.
                for entry in metrics.into_iter() {
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
                            None => MetricId(format!("{}", entry.category)),
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
                let mut calc_rules = vec![Vec::with_capacity(2); categories.len()];
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

                Config {
                    all_categories,
                    categories,
                    are_provided,
                    ids,
                    indices,
                    calc_rules,
                }
            }
        }
    }
}
