use serde::Deserialize;

use crate::configs::implementation::parsing::generating::{edges, nodes};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub nodes: nodes::raw::Config,
    pub edges: edges::raw::Config,
}
