use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use log::debug;

use crate::network::Graph;

//------------------------------------------------------------------------------------------------//
// nodes

#[derive(Copy, Clone)]
struct CostNode {
    pub idx: usize,
    pub cost: u32,
    pub estimation: u32,
    pub pred_idx: Option<usize>,
}
impl Ord for CostNode {
    fn cmp(&self, other: &CostNode) -> Ordering {
        // (1) cost in float, but cmp uses only m, which is ok
        // (2) inverse order since BinaryHeap is max-heap, but min-heap is needed
        (other.cost + other.estimation)
            .cmp(&(self.cost + self.estimation))
            .then_with(|| other.idx.cmp(&self.idx))
    }
}
impl PartialOrd for CostNode {
    fn partial_cmp(&self, other: &CostNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for CostNode {}
impl PartialEq for CostNode {
    fn eq(&self, other: &CostNode) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

//------------------------------------------------------------------------------------------------//
// Dijkstra's type of path

#[derive(Clone)]
pub struct Path {
    src_idx: usize,
    dst_idx: usize,
    cost: u32,
    predecessors: HashMap<usize, usize>,
    successors: HashMap<usize, usize>,
}
impl Path {
    fn new(src_idx: usize, dst_idx: usize) -> Path {
        Path {
            src_idx,
            dst_idx,
            cost: 0,
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    pub fn src_idx(&self) -> usize {
        return self.src_idx;
    }

    pub fn dst_idx(&self) -> usize {
        return self.dst_idx;
    }

    pub fn cost(&self) -> u32 {
        return self.cost;
    }

    /// Return idx of predecessor
    pub fn predecessor(&self, idx: usize) -> Option<usize> {
        match self.predecessors.get(&idx) {
            Some(&pred) => Some(pred),
            None => None,
        }
    }

    /// Return idx of successor
    pub fn successor(&self, idx: usize) -> Option<usize> {
        match self.successors.get(&idx) {
            Some(&succ) => Some(succ),
            None => None,
        }
    }
}

//------------------------------------------------------------------------------------------------//
// Dijkstra

pub fn compute_shortest_path(src_id: i64, dst_id: i64, graph: &Graph) -> Option<Path> {
    //--------------------------------------------------------------------------------------------//
    // initialization-stuff

    let mut visited: Vec<bool> = vec![false; graph.node_count()];
    let mut predecessors: Vec<Option<usize>> = vec![None; graph.node_count()];
    let mut queue = BinaryHeap::new(); // max-heap, but CostNode's natural order is reversed

    // use graph-structure (offset-array) and work with idx instead of id
    let src_idx = match graph.node_idx_from(src_id) {
        Ok(idx) => idx,
        Err(_) => {
            debug!("Src-id {} is not in graph.", src_id);
            return None;
        }
    };
    let dst_idx = match graph.node_idx_from(dst_id) {
        Ok(idx) => idx,
        Err(_) => {
            debug!("Dst-id {} is not in graph.", dst_id);
            return None;
        }
    };

    //--------------------------------------------------------------------------------------------//
    // compute

    // prepare first iteration
    queue.push(CostNode {
        idx: src_idx,
        cost: 0,
        estimation: 0,
        pred_idx: None,
    });

    // while let Some(current) = queue.pop().filter(|current| predecessors.contains_key(&current.idx)) {
    while let Some(current) = queue.pop() {
        // first occurrence has lowest cost
        // -> check if current has already been visited
        if visited[current.idx] {
            continue;
        } else {
            visited[current.idx] = true;
        }
        predecessors[current.idx] = current.pred_idx;

        // if shortest path found
        // -> create path
        if current.idx == dst_idx {
            let mut cur_idx = current.idx;

            let mut path = Path::new(src_idx, dst_idx);
            path.cost = current.cost;
            while let Some(pred_idx) = predecessors[cur_idx] {
                path.predecessors.insert(cur_idx, pred_idx);
                path.successors.insert(pred_idx, cur_idx);
                cur_idx = pred_idx;
            }
            // predecessor of src is not set
            // successor of dst is not set
            return Some(path);
        }

        if let Some(leaving_edges) = graph.leaving_edges(current.idx) {
            // update cost and add predecessors
            // to nodes, that are dst of current's leaving edges
            for leaving_edge in leaving_edges {
                let new_cost = current.cost + leaving_edge.meters();

                if predecessors[leaving_edge.dst_idx()].is_none() {
                    queue.push(CostNode {
                        idx: leaving_edge.dst_idx(),
                        cost: new_cost,
                        estimation: 0,
                        pred_idx: Some(current.idx),
                    });
                }
            }
        }
    }

    None
}
