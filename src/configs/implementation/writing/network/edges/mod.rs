pub mod proto;
pub mod raw;
use crate::configs::SimpleId;

#[derive(Debug)]
pub struct Config {
    pub ids: Vec<Option<SimpleId>>,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
        Config { ids: proto_cfg.ids }
    }
}
