use crate::{
    configs::implementation::parsing::vehicles::raw, network::vehicles::Category as VehicleCategory,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "raw::Config")]
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
