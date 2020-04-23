use super::{parsing, routing, writing};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub parsing: parsing::raw::Config,
    pub writing: Option<writing::raw::Config>,
    pub routing: Option<routing::raw::Config>,
}
