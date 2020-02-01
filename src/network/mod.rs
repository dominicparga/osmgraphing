mod graph;
pub use graph::{
    building::{GraphBuilder, ProtoEdge, ProtoNode},
    EdgeIdx, Graph, HalfEdge, Node, NodeIdx,
};
pub mod defaults;
pub use defaults::StreetType;
