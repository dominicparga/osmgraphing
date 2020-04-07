use crate::{
    defaults::capacity::DimVec,
    helpers,
    network::{EdgeIdx, Graph, NodeIdx},
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
    costs: Option<DimVec<f64>>,
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prettier_edges: Vec<_> = self.edges.iter().map(|edge_idx| **edge_idx).collect();
        write!(
            f,
            "{{ src-idx: {}, dst-idx: {}, costs: {:?}, edges: {:?} }}",
            self.src_idx, self.dst_idx, self.costs, prettier_edges
        )
    }
}

impl Path {
    /// ATTENTION! This method does not calculate the path's cost.
    /// This can be done, e.g., with `calc_cost(...)` or `flatten(...)`.
    /// Accessing the costs without calculating them will lead to panics.
    pub fn new(src_idx: NodeIdx, dst_idx: NodeIdx, edges: Vec<EdgeIdx>) -> Path {
        Path {
            src_idx,
            dst_idx,
            edges,
            costs: None,
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

    /// ATTENTION! This method panics if the costs hasn't been calculated (e.g. `calc_cost(...)` or `flatten(...)`).
    pub fn costs(&self) -> &DimVec<f64> {
        self.costs
            .as_ref()
            .expect("Path's cost has to be calculated.")
    }

    /// Calculates the path's cost, but only if not existent.
    pub fn calc_costs(&mut self, graph: &Graph) -> &DimVec<f64> {
        if self.costs.is_none() {
            let graph_metrics = graph.metrics();
            self.costs = Some(
                self.edges
                    .iter()
                    .map(|edge_idx| &graph_metrics[edge_idx])
                    .fold(smallvec![0.0; graph_metrics.dim()], |acc, m| {
                        helpers::add(&acc, m)
                    }),
            );
        }
        self.costs.as_ref().unwrap()
    }

    /// Flattens shortcuts, out-of-place, and calculates the flattened path's cost.
    pub fn flatten(self, graph: &Graph) -> Path {
        // setup new edges
        let mut flattened_path = Path {
            src_idx: self.src_idx,
            dst_idx: self.dst_idx,
            edges: Vec::with_capacity(self.edges.capacity()),
            costs: Some(smallvec![0.0; graph.metrics().dim()]),
        };

        // interpret old edges as stack, beginning with src
        let mut old_edges = self.edges;
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
            helpers::add_assign(
                flattened_path.costs.as_mut().unwrap(),
                &graph.metrics()[edge_idx],
            );
        }

        flattened_path.edges.shrink_to_fit();
        flattened_path
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
