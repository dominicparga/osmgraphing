use crate::configs::{implementation::writing::network::graph::edges::raw, SimpleId};

#[derive(Debug)]
pub struct Config {
    pub is_writing_shortcuts: bool,
    pub ids: Vec<Option<SimpleId>>,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        Config {
            is_writing_shortcuts: raw_cfg.is_writing_shortcuts,
            ids: raw_cfg
                .categories
                .into_iter()
                .map(|category| match category {
                    raw::Category::Id(id) => Some(id),
                    raw::Category::Ignored => None,
                })
                .collect(),
        }
    }
}
