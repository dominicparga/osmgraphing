pub mod proto;
pub mod raw;
use crate::{configs::SimpleId, defaults};

#[derive(Clone, Debug)]
pub struct Config {
    pub is_writing_shortcuts: bool,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<proto::Config> for Config {
    fn from(proto_cfg: proto::Config) -> Config {
        Config {
            is_writing_shortcuts: proto_cfg
                .is_writing_shortcuts
                .unwrap_or(defaults::parsing::IS_USING_SHORTCUTS),
            ids: proto_cfg.ids,
        }
    }
}
