use crate::{
    configs::parsing,
    defaults::{self, capacity::DimVec},
    helpers,
    io::SupportingFileExts,
};
use smallvec::smallvec;
use std::path::Path;
pub mod raw;

/// # Specifying routing (TODO update text)
///
/// Further, the metrics, which are used in the routing, can be listed in the routing-section with their previously defined id.
/// Comparisons are made using pareto-optimality, so there is no comparison between metrics.
/// In case you'll use personlized-routing, default-preferences can be set with weights.
/// The example below shows a routing-case, where the metric `distance` is weighted with `169 / (169 + 331) = 33.8 %` while the metric `duration` is weighted with `331 / (169 + 331) = 66.2 %`.
#[derive(Clone, Debug)]
pub struct Config {
    pub is_ch_dijkstra: bool,
    pub alphas: DimVec<f64>,
    pub tolerated_scales: DimVec<f64>,
}

impl SupportingFileExts for Config {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}

impl Config {
    pub fn try_from_str(yaml_str: &str, parsing_cfg: &parsing::Config) -> Result<Config, String> {
        let raw_cfg = {
            match serde_yaml::from_str(yaml_str) {
                Ok(raw_cfg) => raw_cfg,
                Err(e) => return Err(format!("{}", e)),
            }
        };
        Config::try_from_raw(raw_cfg, parsing_cfg)
    }

    pub fn from_str(yaml_str: &str, parsing_cfg: &parsing::Config) -> Config {
        match Config::try_from_str(yaml_str, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn try_from_raw(
        raw_cfg: raw::Config,
        parsing_cfg: &parsing::Config,
    ) -> Result<Config, String> {
        let raw_cfg = raw_cfg.routing;
        let dim = parsing_cfg.edges.metrics.units.len();

        // Alpha is 0.0 because non-mentioned id will not be considered.
        let mut alphas = smallvec![0.0; dim];
        // Same argument holds for the toleration.
        let mut tolerated_scales = smallvec![std::f64::INFINITY; dim];

        for entry in raw_cfg.metrics.into_iter() {
            let alpha = entry.alpha.unwrap_or(defaults::routing::ALPHA);
            let tolerated_scale = entry
                .tolerated_scale
                .unwrap_or(defaults::routing::TOLERATED_SCALE);

            if let Some(metric_idx) = parsing_cfg
                .edges
                .metrics
                .ids
                .iter()
                .position(|id| id == &entry.id)
            {
                alphas[metric_idx] = alpha;
                tolerated_scales[metric_idx] = tolerated_scale;
            } else {
                return Err(format!(
                    "The given id {} should get alpha {}, but doesn't exist.",
                    entry.id, alpha
                ));
            }
        }

        Ok(Config {
            is_ch_dijkstra: raw_cfg.is_ch_dijkstra.unwrap_or(false),
            alphas,
            tolerated_scales,
        })
    }

    pub fn from_raw(raw_cfg: raw::Config, parsing_cfg: &parsing::Config) -> Config {
        match Config::try_from_raw(raw_cfg, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn try_from_yaml<P: AsRef<Path> + ?Sized>(
        path: &P,
        parsing_cfg: &parsing::Config,
    ) -> Result<Config, String> {
        let file = {
            Config::find_supported_ext(path)?;
            helpers::open_file(path)?
        };

        let raw_cfg = match serde_yaml::from_reader(file) {
            Ok(raw_cfg) => raw_cfg,
            Err(e) => return Err(format!("{}", e)),
        };
        Config::try_from_raw(raw_cfg, parsing_cfg)
    }

    pub fn from_yaml<P: AsRef<Path> + ?Sized>(path: &P, parsing_cfg: &parsing::Config) -> Config {
        match Config::try_from_yaml(path, parsing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => panic!("{}", msg),
        }
    }
}
