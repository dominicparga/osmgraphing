use crate::network::vehicles::Category as VehicleCategory;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Config {
    pub category: VehicleCategory,
    pub are_drivers_picky: bool,
}

impl From<ProtoConfig> for Config {
    fn from(proto_cfg: ProtoConfig) -> Self {
        Config {
            category: proto_cfg.category,
            are_drivers_picky: proto_cfg.are_drivers_picky,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct ProtoConfig {
    pub category: VehicleCategory,
    pub are_drivers_picky: bool,
}

impl From<RawConfig> for ProtoConfig {
    fn from(raw_cfg: RawConfig) -> ProtoConfig {
        ProtoConfig {
            category: raw_cfg.category,
            are_drivers_picky: raw_cfg.are_drivers_picky,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    pub category: VehicleCategory,
    pub are_drivers_picky: bool,
}
