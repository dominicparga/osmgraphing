use crate::configs::SimpleId;
pub mod proto;
pub mod raw;

#[derive(Debug)]
pub struct Config {
    pub ids: Vec<Option<SimpleId>>,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
        Config { ids: proto_cfg.ids }
    }
}
