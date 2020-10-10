use crate::{
    defaults::capacity::DimVec,
    helpers::{self, err},
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
    src_id: i64,
    dst_idx: NodeIdx,
    dst_id: i64,
    edges: Vec<EdgeIdx>,
    costs: Option<DimVec<f64>>,
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ src-id: {}, dst-id: {}, costs: {:?}, hop-distance: {:?} }}",
            self.src_id,
            self.dst_id,
            self.costs,
            self.edges.len()
        )
    }
}

impl Path {
    /// ATTENTION! This method does not calculate the path's cost.
    /// This can be done, e.g., with `calc_cost(...)` or `flatten(...)`.
    /// Accessing the costs without calculating them will lead to panics.
    pub fn new(
        src_idx: NodeIdx,
        src_id: i64,
        dst_idx: NodeIdx,
        dst_id: i64,
        edges: Vec<EdgeIdx>,
    ) -> Path {
        Path {
            src_idx,
            src_id,
            dst_idx,
            dst_id,
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

    /// ATTENTION! This method panics if the costs hasn't been calculated (e.g. `calc_cost(...)` or `flatten(...)`).
    pub fn costs(&self) -> &DimVec<f64> {
        self.costs
            .as_ref()
            .expect("Path's cost has to be calculated.")
    }

    /// Calculates the path's cost, but only if not calculated already.
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
        self.costs
            .as_ref()
            .expect("Costs have just been calculated.")
    }

    /// Flattens shortcuts, out-of-place, and calculates the flattened path's cost.
    pub fn try_flatten(self, graph: &Graph) -> err::Result<Path> {
        // setup new edges
        let mut flattened_path = Path {
            src_idx: self.src_idx,
            src_id: self.src_id,
            dst_idx: self.dst_idx,
            dst_id: self.dst_id,
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

                // max path-length contains all edges in a graph
                if old_edges.len() > fwd_edges.count() {
                    return Err("There is a cycle of shortcut-references in the graph.".into());
                }
            }

            // edge-idx is not a shortcut
            // -> push to flattened path
            flattened_path.edges.push(edge_idx);
            helpers::add_assign(
                flattened_path
                    .costs
                    .as_mut()
                    .expect("Flattened path should have calculated costs."),
                &graph.metrics()[edge_idx],
            );
        }

        flattened_path.edges.shrink_to_fit();
        Ok(flattened_path)
    }

    pub fn flatten(self, graph: &Graph) -> Path {
        match self.try_flatten(graph) {
            Ok(path) => path,
            Err(msg) => panic!("{}", msg),
        }
    }
}

impl Eq for Path {}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        // length before edges and edges last because of performance
        self.src_id == other.src_id
            && self.dst_id == other.dst_id
            // length is compared by comparing Vecs
            // -> see libcore in rust-lang/rust
            // && self.edges.len() == other.edges.len()
            && self.edges == other.edges
    }
}

impl IntoIterator for Path {
    type Item = EdgeIdx;
    type IntoIter = std::vec::IntoIter<EdgeIdx>;

    fn into_iter(self) -> std::vec::IntoIter<EdgeIdx> {
        self.edges.into_iter()
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a EdgeIdx;
    type IntoIter = std::slice::Iter<'a, EdgeIdx>;

    fn into_iter(self) -> std::slice::Iter<'a, EdgeIdx> {
        self.edges.iter()
    }
}

impl Path {
    pub fn iter(&self) -> std::slice::Iter<'_, EdgeIdx> {
        self.edges.iter()
    }
}

// pub struct PathIntoIter(std::vec::IntoIter<EdgeIdx>);
//
// impl Iterator for PathIntoIter {
//     type Item = EdgeIdx;
//
//     fn next(&mut self) -> Option<EdgeIdx> {
//         self.0.next()
//     }
// }
