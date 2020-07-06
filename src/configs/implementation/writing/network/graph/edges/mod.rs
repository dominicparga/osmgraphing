pub mod proto;
pub mod raw;
use crate::configs::SimpleId;

#[derive(Debug)]
pub struct Config {
    pub is_writing_shortcuts: bool,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
        Config {
            is_writing_shortcuts: proto_cfg.is_writing_shortcuts,
            ids: proto_cfg.ids,
        }
    }
}
