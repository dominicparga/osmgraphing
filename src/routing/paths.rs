use crate::{
    defaults::capacity::DimVec,
    helpers,
    network::{EdgeIdx, Graph, MetricIdx, NodeIdx},
};
use smallvec::smallvec;
use std::{
    cmp::{Eq, PartialEq},
    fmt::{self, Display},
};

/// A path from a src to a dst storing all edges in between.
#[derive(Clone, Debug)]
pub struct Path {
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    edges: Vec<EdgeIdx>,
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prettier_edges: Vec<_> = self.edges.iter().map(|edge_idx| **edge_idx).collect();
        write!(
            f,
            "{{ src-idx: {}, dst-idx: {}, edges: {:?} }}",
            self.src_idx, self.dst_idx, prettier_edges
        )
    }
}

impl Path {
    pub fn new(src_idx: NodeIdx, dst_idx: NodeIdx, edges: Vec<EdgeIdx>) -> Path {
        Path {
            src_idx,
            dst_idx,
            edges,
        }
    }

    pub fn src_idx(&self) -> NodeIdx {
        self.src_idx
    }

    pub fn dst_idx(&self) -> NodeIdx {
        self.dst_idx
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Flattens shortcuts, out-of-place
    pub fn flatten(self, graph: &Graph) -> Path {
        // setup new edges
        let mut flattened_path = Path {
            src_idx: self.src_idx,
            dst_idx: self.dst_idx,
            edges: Vec::with_capacity(self.edges.capacity()),
        };

        // interpret old edges as stack, beginning with src
        let mut old_edges = self.edges;
        old_edges.shrink_to_fit();
        old_edges.reverse();

        let fwd_edges = graph.fwd_edges();
        while let Some(mut edge_idx) = old_edges.pop() {
            // if edge is shortcut
            // -> push on old-edges
            while let Some(sc_edges) = fwd_edges.sc_edges(edge_idx) {
                old_edges.push(sc_edges[1]);
                edge_idx = sc_edges[0];
            }

            // edge-idx is not a shortcut
            // -> push to flattened path
            flattened_path.edges.push(edge_idx);
        }

        flattened_path
    }

    pub fn calc_cost(&self, metric_indices: &[MetricIdx], graph: &Graph) -> DimVec<f64> {
        self.edges
            .iter()
            .map(|edge_idx| graph.metrics().get_more(metric_indices, *edge_idx))
            .fold(smallvec![0.0; metric_indices.len()], |acc, m| {
                helpers::add(&acc, &m)
            })
    }
}

impl Eq for Path {}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.src_idx() == other.src_idx()
            && self.dst_idx() == other.dst_idx()
            && self.edges == other.edges
    }
}
