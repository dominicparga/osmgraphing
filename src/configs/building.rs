use crate::configs::{self, MetricId};
use serde::Deserialize;

impl From<Config> for super::Config {
    fn from(proto_cfg: Config) -> super::Config {
        // create super-config's structures
        let graph = proto_cfg.graph;
        let mut routing = configs::routing::Config::new(graph.edges.metrics.count());

        // translate ids into indices
        for Entry {
            id: metric_id,
            alpha,
        } in proto_cfg.routing.iter()
        {
            let metric_idx = graph.edges.metrics.idx(metric_id);
            routing.push(metric_idx, alpha.unwrap_or(1.0));
        }

        // return
        super::Config { graph, routing }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    graph: configs::graph::Config,
    routing: Vec<Entry>, //BTreeMap<MetricId, Option<f32>>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    id: MetricId,
    alpha: Option<f32>,
}
