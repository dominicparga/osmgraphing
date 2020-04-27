use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config(pub Vec<Category>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Id(SimpleId),
    Ignored,
}
