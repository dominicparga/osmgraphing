use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub is_writing_shortcuts: bool,
    #[serde(flatten)]
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Id(SimpleId),
    Ignored,
}
