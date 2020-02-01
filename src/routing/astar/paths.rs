use crate::network::NodeIdx;
use crate::units::Metric;
use std::collections::HashMap;

//------------------------------------------------------------------------------------------------//
// Path using Vec

#[derive(Clone)]
pub struct VecPath {
    pub src_idx: usize,
    pub dst_idx: usize,
    pub cost: u32,
    predecessors: Vec<Option<usize>>,
    successors: Vec<Option<usize>>,
}
#[allow(dead_code)]
impl VecPath {
    pub fn with_capacity(src_idx: usize, dst_idx: usize, capacity: usize) -> Self {
        VecPath {
            src_idx,
            dst_idx,
            cost: 0,
            predecessors: vec![None; capacity],
            successors: vec![None; capacity],
        }
    }

    pub fn add_pred_succ(&mut self, pred: usize, succ: usize) {
        self.predecessors[succ] = Some(pred);
        self.successors[pred] = Some(succ);
    }

    pub fn pred_node_idx(&self, idx: usize) -> Option<usize> {
        *(self.predecessors.get(idx)?)
    }
    pub fn succ_node_idx(&self, idx: usize) -> Option<usize> {
        *(self.successors.get(idx)?)
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
