use crate::{network::NodeIdx, units::Metric};
use std::collections::HashMap;

//------------------------------------------------------------------------------------------------//
// Path using Vec

#[derive(Clone)]
pub struct VecPath<M> {
    pub src_idx: NodeIdx,
    pub dst_idx: NodeIdx,
    pub cost: M,
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
// Path using HashMap

#[derive(Clone)]
pub struct HashPath<M>
where
    M: Metric,
{
    pub src_idx: NodeIdx,
    pub dst_idx: NodeIdx,
    pub cost: M,
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
