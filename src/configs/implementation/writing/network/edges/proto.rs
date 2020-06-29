use crate::configs::{implementation::writing::network::edges::raw, SimpleId};

#[derive(Debug)]
pub struct Config {
    pub ids: Vec<Option<SimpleId>>,
}

impl From<raw::Config> for Config {
    fn from(raw_cfg: raw::Config) -> Config {
        Config {
            ids: raw_cfg
                .0
                .into_iter()
                .map(|category| match category {
                    raw::Category::Id(id) => Some(id),
                    raw::Category::Ignored => None,
                })
                .collect(),
        }
    }
}
