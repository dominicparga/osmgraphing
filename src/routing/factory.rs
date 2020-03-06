pub mod dijkstra {
    use crate::{
        network::{HalfEdge, MetricIdx},
        routing::dijkstra::{bidirectional, unidirectional, Dijkstra},
    };

    pub fn unidirectional(metric_idx: MetricIdx) -> Box<dyn Dijkstra> {
        let cost_fn = move |edge: &HalfEdge| edge.metric(metric_idx).unwrap();
        Box::new(unidirectional::GenericDijkstra::new(cost_fn))
    }

    pub fn bidirectional(metric_idx: MetricIdx) -> Box<dyn Dijkstra> {
        let cost_fn = move |edge: &HalfEdge| edge.metric(metric_idx).unwrap();
        Box::new(bidirectional::GenericDijkstra::new(cost_fn))
    }
}
