use crate::{network::NodeIdx, units::Metric};
use std::collections::HashMap;

//------------------------------------------------------------------------------------------------//

/// A path from a src to a dst storing predecessors and successors.
///
/// The implementation bases either on vectors or on hashmaps.
/// Some words about it without doing a benchmark:
/// - Since the vector-approach stores two fully allocated vectors, it probably consumes more memory than the hashmap-approach.
/// - Just by looking at resulting times of long paths (~670 km) in Germany, the hashmap-approach seems to be slightly better in performance, but both approaches take around 7 seconds for it.
#[derive(Clone)]
pub struct Path<M>
where
    M: Metric,
{
    // core: paths::VecPath<M>,
    core: HashPath<M>,
}

impl<M> Path<M>
where
    M: Metric,
{
    fn new(src_idx: NodeIdx, dst_idx: NodeIdx) -> Self {
        let core = HashPath::new(src_idx, dst_idx);
        Path { core }
    }

    /// Capacity is `graph.nodes().count()`, which is only used, if the path bases on vectors.
    /// If it bases on hashmaps, it ignores the given capacity.
    pub fn with_capacity(src_idx: NodeIdx, dst_idx: NodeIdx, _capacity: usize) -> Self {
        // let core = VecPath::with_capacity(src_idx, dst_idx, capacity);
        Path::new(src_idx, dst_idx)
    }

    pub fn src_idx(&self) -> NodeIdx {
        self.core.src_idx
    }

    pub fn dst_idx(&self) -> NodeIdx {
        self.core.dst_idx
    }

    pub fn cost(&self) -> M {
        self.core.cost
    }

    pub fn cost_mut(&mut self) -> &mut M {
        &mut (self.core.cost)
    }

    pub fn add_pred_succ(&mut self, pred: NodeIdx, succ: NodeIdx) {
        self.core.add_pred_succ(pred, succ)
    }

    /// Return idx of predecessor-node
    pub fn pred_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        self.core.pred_node_idx(idx)
    }

    /// Return idx of successor-node
    pub fn succ_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        self.core.succ_node_idx(idx)
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Clone)]
pub struct VecPath<M> {
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    cost: M,
    predecessors: Vec<Option<NodeIdx>>,
    successors: Vec<Option<NodeIdx>>,
}

#[allow(dead_code)]
impl<M> VecPath<M>
where
    M: Metric,
{
    pub fn with_capacity(src_idx: NodeIdx, dst_idx: NodeIdx, capacity: usize) -> Self {
        VecPath {
            src_idx,
            dst_idx,
            cost: M::zero(),
            predecessors: vec![None; capacity],
            successors: vec![None; capacity],
        }
    }

    pub fn add_pred_succ(&mut self, pred: NodeIdx, succ: NodeIdx) {
        self.predecessors[succ.to_usize()] = Some(pred);
        self.successors[pred.to_usize()] = Some(succ);
    }

    pub fn pred_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        *(self.predecessors.get(idx.to_usize())?)
    }

    pub fn succ_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        *(self.successors.get(idx.to_usize())?)
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Clone)]
pub struct HashPath<M>
where
    M: Metric,
{
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    cost: M,
    predecessors: HashMap<NodeIdx, NodeIdx>,
    successors: HashMap<NodeIdx, NodeIdx>,
}

impl<M> HashPath<M>
where
    M: Metric,
{
    pub fn new(src_idx: NodeIdx, dst_idx: NodeIdx) -> Self {
        HashPath {
            src_idx,
            dst_idx,
            cost: M::zero(),
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    pub fn add_pred_succ(&mut self, pred: NodeIdx, succ: NodeIdx) {
        self.predecessors.insert(succ, pred);
        self.successors.insert(pred, succ);
    }

    pub fn pred_node_idx(&self, idx: NodeIdx) -> Option<NodeIdx> {
        Some(*(self.predecessors.get(&idx)?))
    }

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
