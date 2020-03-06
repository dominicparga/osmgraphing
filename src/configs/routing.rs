use crate::{defaults::DimVec, network::MetricIdx};
use log::error;

#[derive(Debug)]
pub struct Config {
    indices: DimVec<MetricIdx>,
    alphas: DimVec<f32>,
}

impl Config {
    pub fn new(metric_count: usize) -> Config {
        Config {
            indices: DimVec::with_capacity(metric_count),
            alphas: DimVec::with_capacity(metric_count),
        }
    }

    pub fn alpha(&self, idx: MetricIdx) -> f32 {
        match self.alphas.get(*idx) {
            Some(alpha) => *alpha,
            None => {
                error!("Idx {} not found in config.", idx);
                std::process::exit(1);
            }
        }
    }

    pub fn indices(&self) -> &DimVec<MetricIdx> {
        &self.indices
    }

    pub fn push(&mut self, idx: MetricIdx, alpha: f32) {
        self.indices.push(idx);
        self.alphas.push(alpha);
    }
}
