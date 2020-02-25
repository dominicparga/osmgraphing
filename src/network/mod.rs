mod graph;
pub use graph::{
    building::{GraphBuilder, ProtoEdge, ProtoNode},
    EdgeIdx, Graph, HalfEdge, MetricIdx, Node, NodeIdx,
};
pub mod defaults;
pub use defaults::StreetType;
