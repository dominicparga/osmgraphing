use crate::{helpers::ApproxEq, network::NodeIdx};
use std::collections::HashMap;

/// A path from a src to a dst storing predecessors and successors.
///
/// The implementation bases either on vectors or on hashmaps.
/// Some words about it without doing a benchmark:
/// - Since the vector-approach stores two fully allocated vectors, it probably consumes more memory than the hashmap-approach.
/// - Just by looking at resulting times of long paths (~670 km) in Germany, the hashmap-approach seems to be slightly better in performance, but both approaches take around 7 seconds for it.
/// - The hashmap is faster for small routes (isle-of-man, 2.9 ms vs 2.6 ms)
#[derive(Clone, Debug)]
pub struct Path<M> {
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    cost: M,
    predecessors: HashMap<NodeIdx, NodeIdx>,
    successors: HashMap<NodeIdx, NodeIdx>,
}

impl<M> Path<M> {
    pub fn new(src_idx: NodeIdx, dst_idx: NodeIdx, cost_max: M) -> Self {
        Path {
            src_idx,
            dst_idx,
            cost: cost_max,
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    pub fn src_idx(&self) -> NodeIdx {
        self.src_idx
    }

    pub fn dst_idx(&self) -> NodeIdx {
        self.dst_idx
    }

    pub fn cost(&self) -> &M {
        &self.cost
    }

    pub fn cost_mut(&mut self) -> &mut M {
        &mut (self.cost)
    }

    pub fn add_pred_succ(&mut self, pred: NodeIdx, succ: NodeIdx) {
        self.predecessors.insert(succ, pred);
        self.successors.insert(pred, succ);
    }

    /// Return idx of predecessor-node
    pub fn pred_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        Some(*(self.predecessors.get(&idx)?))
    }

    /// Return idx of successor-node
    pub fn succ_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        Some(*(self.successors.get(&idx)?))
    }

    // impl Into<Vec<usize>> for Path {
    //     fn into(self) -> Vec<usize> {
    //         let mut nodes = vec![];
    //         if self.successors.len() > 0 {
    //             let mut current = self.src_idx;
    //             nodes.push(current);
    //             while let Some(succ) = self.successor(current) {
    //                 nodes.push(succ);
    //                 current = succ;
    //             }
    //         }
    //         nodes
    //     }
    // }
}

impl<M> ApproxEq for Path<M>
where
    M: ApproxEq,
{
    fn approx_eq(&self, other: &Path<M>) -> bool {
        self.src_idx() == other.src_idx()
            && self.dst_idx() == other.dst_idx()
            && self.cost().approx_eq(other.cost())
    }
}
