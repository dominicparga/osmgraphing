mod edges;
pub use edges::EdgeIdx;
mod nodes;
pub use nodes::NodeIdx;

mod metrics;
// pub use metrics::MetricIdx;
#[derive(Copy, Clone, Debug)]
pub struct MetricIdx(pub usize);

impl std::ops::Deref for MetricIdx {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}
