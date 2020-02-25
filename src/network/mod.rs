mod graph;
pub use graph::{
    building::{GraphBuilder, ProtoNode, UnfinishedEdge},
    EdgeIdx, Graph, HalfEdge, MetricIdx, Node, NodeIdx,
};
pub mod defaults;
pub use defaults::StreetType;
