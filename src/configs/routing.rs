use crate::{configs::graph, defaults::DimVec, network::MetricIdx};
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Config {
    indices: DimVec<MetricIdx>,
    alphas: DimVec<f32>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            indices: DimVec::new(),
            alphas: DimVec::new(),
        }
    }

    pub fn from_str(yaml_str: &str, graph_cfg: &graph::Config) -> Result<Config, String> {
        #[derive(Deserialize)]
        struct Entries {
            routing: Vec<super::Entry>,
        }

        let entries: Result<Entries, _> = serde_yaml::from_str(yaml_str);
        match entries {
            Ok(entries) => Ok(Config::from_entries(entries.routing, graph_cfg)),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn with_capacity(metric_count: usize) -> Config {
        Config {
            indices: DimVec::with_capacity(metric_count),
            alphas: DimVec::with_capacity(metric_count),
        }
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

    pub fn push(&mut self, idx: MetricIdx, alpha: f32) {
        self.indices.push(idx);
        self.alphas.push(alpha);
    }

    pub fn dim(&self) -> usize {
        self.indices.len()
    }
}
