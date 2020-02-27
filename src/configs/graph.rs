use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub map_file: PathBuf,
    pub vehicles: vehicles::Config,
    pub edges: edges::Config,
}

pub mod vehicles {
    use crate::configs::VehicleType;

    #[derive(Debug)]
    pub struct Config {
        pub vehicle_type: VehicleType,
        pub is_driver_picky: bool,
    }
}

pub mod edges {
    #[derive(Debug)]
    pub struct Config {
        pub metrics: metrics::Config,
    }

    pub mod metrics {
        use crate::{
            configs::{MetricCategory, MetricId},
            network::MetricIdx,
        };
        use std::collections::BTreeMap;

        pub struct Entry {
            pub category: MetricCategory,
            pub id: MetricId,
            pub is_provided: bool,
            pub calc_rules: Option<Vec<MetricId>>,
        }

        impl From<(MetricCategory, MetricId, bool)> for Entry {
            fn from((category, id, is_provided): (MetricCategory, MetricId, bool)) -> Entry {
                Entry {
                    category,
                    id,
                    is_provided,
                    calc_rules: None,
                }
            }
        }

        impl From<(MetricCategory, MetricId, bool, Vec<MetricId>)> for Entry {
            fn from(
                (category, id, is_provided, calc_rules): (
                    MetricCategory,
                    MetricId,
                    bool,
                    Vec<MetricId>,
                ),
            ) -> Entry {
                Entry {
                    category,
                    id,
                    is_provided,
                    calc_rules: Some(calc_rules),
                }
            }
        }

        #[derive(Debug)]
        pub struct Config {
            // store for order
            all_categories: Vec<MetricCategory>,
            // store for quick access
            categories: Vec<MetricCategory>,
            are_provided: Vec<bool>,
            indices: BTreeMap<MetricId, MetricIdx>,
            ids: Vec<MetricId>,
            calc_rules: Vec<Vec<(MetricCategory, MetricIdx)>>,
        }

        impl Config {
            pub fn create(metrics: Vec<Entry>) -> Result<Config, String> {
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
                            return Err(format!(
                                "Metric-category {} has calculation-rules given, \
                                 but is ignored and hence should not have any calculation-rule.",
                                entry.category
                            ));
                        }
                    } else {
                        categories.push(entry.category);
                        ids.push(entry.id.clone());
                        are_provided.push(entry.is_provided);

                        let metric_idx = MetricIdx(indices.len());
                        indices.insert(entry.id.clone(), metric_idx);
                        proto_calc_rules.push(entry.calc_rules);
                    }
                }

                // add calculation-rules after everything else is already finished
                let mut calc_rules = vec![Vec::with_capacity(2); categories.len()];
                for (metric_idx, opt_calc_rule) in proto_calc_rules.into_iter().enumerate() {
                    if let Some(calc_rule) = opt_calc_rule {
                        // implement as given
                        for other_id in calc_rule.into_iter() {
                            let other_idx = indices[&other_id];
                            let other_type = categories[*other_idx];
                            calc_rules[metric_idx].push((other_type, other_idx));
                        }
                    }
                }

                Ok(Config {
                    all_categories,
                    categories,
                    are_provided,
                    ids,
                    indices,
                    calc_rules,
                })
            }

            pub fn all_categories(&self) -> &Vec<MetricCategory> {
                &self.all_categories
            }

            pub fn category(&self, idx: MetricIdx) -> MetricCategory {
                self.categories[*idx]
            }

            pub fn count(&self) -> usize {
                self.categories.len()
            }

            pub fn is_provided(&self, idx: MetricIdx) -> bool {
                self.are_provided[*idx]
            }

            pub fn idx(&self, id: &MetricId) -> MetricIdx {
                self.indices[id]
            }

            pub fn id(&self, idx: MetricIdx) -> &MetricId {
                &self.ids[*idx]
            }

            pub fn calc_rules(&self, idx: MetricIdx) -> &Vec<(MetricCategory, MetricIdx)> {
                &self.calc_rules[*idx]
            }
        }
    }
}
