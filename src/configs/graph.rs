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
    use crate::{configs::MetricType, network::MetricIdx};

    #[derive(Debug)]
    pub struct Config {
        pub metric_types: Vec<MetricType>,
    }

    impl Config {
        pub fn metric_idx(&self, metric_type: &MetricType) -> Option<MetricIdx> {
            let idx = self
                .metric_types
                .iter()
                .filter(|mt| !mt.is_ignored())
                .position(|mt| mt.id() == metric_type.id());
            Some(MetricIdx::new(idx?))
        }

        pub fn metric_count(&self) -> usize {
            self.metric_types
                .iter()
                .filter(|mt| !mt.is_ignored())
                .count()
        }

        fn _get(&self, idx: usize) -> Option<&MetricType> {
            Some(self.metric_types.get(idx)?)
        }

        fn _push(&mut self, metric_type: MetricType) {
            self.metric_types.push(metric_type);
        }

        fn _remove(&mut self, idx: usize) -> MetricType {
            self.metric_types.remove(idx)
        }
    }
}
