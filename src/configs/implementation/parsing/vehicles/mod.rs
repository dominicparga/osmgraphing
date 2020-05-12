use crate::network::vehicles::Category as VehicleCategory;
pub mod proto;

#[derive(Debug)]
pub struct Config {
    pub category: VehicleCategory,
    pub are_drivers_picky: bool,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Self {
        Config {
            category: proto_cfg.category,
            are_drivers_picky: proto_cfg.are_drivers_picky,
        }
    }
}
