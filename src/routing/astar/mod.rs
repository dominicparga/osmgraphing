//------------------------------------------------------------------------------------------------//
// other modules

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::network::{Edge, Graph, Node};

//------------------------------------------------------------------------------------------------//
// own modules

mod paths;

//------------------------------------------------------------------------------------------------//
// Path

#[derive(Clone)]
pub struct Path {
    core: paths::HashPath,
}
impl Path {
    fn from(src_idx: usize, dst_idx: usize, _graph: &Graph) -> Self {
        // let core = paths::VecPath::with_capacity(src_idx, dst_idx, graph.node_count());
        let core = paths::HashPath::new(src_idx, dst_idx);
        Path { core }
    }

    //--------------------------------------------------------------------------------------------//

    pub fn src_idx(&self) -> usize {
        self.core.src_idx
    }
    pub fn dst_idx(&self) -> usize {
        self.core.dst_idx
    }

    pub fn cost(&self) -> u32 {
        self.core.cost
    }

    /// Return idx of predecessor-node
    pub fn pred_node_idx(&self, idx: usize) -> Option<usize> {
        self.core.pred_node_idx(idx)
    }
    /// Return idx of successor-node
    pub fn succ_node_idx(&self, idx: usize) -> Option<usize> {
        self.core.succ_node_idx(idx)
    }
}

//------------------------------------------------------------------------------------------------//
// CostNode

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
// Astar

pub trait Astar {
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path>;
}

//------------------------------------------------------------------------------------------------//
// GenericAstar

pub struct GenericAstar<C, E>
where
    C: Fn(&Edge) -> u32,
    E: Fn(&Node, &Node) -> u32,
{
    cost_fn: C,
    estimate_fn: E,
    costs: Vec<u32>,
    predecessors: Vec<Option<usize>>,
    queue: BinaryHeap<CostNode>, // max-heap, but CostNode's natural order is reversed
}
impl<C: Fn(&Edge) -> u32, E: Fn(&Node, &Node) -> u32> GenericAstar<C, E> {
    pub fn from(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E> {
        GenericAstar {
            cost_fn,
            estimate_fn,
            costs: vec![std::u32::MAX; 0],
            predecessors: vec![None; 0],
            queue: BinaryHeap::new(),
        }
    }

    fn resize(&mut self, new_len: usize) {
        let old_len = self.costs.len();
        let min_len = std::cmp::min(old_len, new_len);
        for i in 0..min_len {
            self.costs[i] = std::u32::MAX;
            self.predecessors[i] = None;
        }
        self.costs.resize(new_len, std::u32::MAX);
        self.predecessors.resize(new_len, None);

        self.queue.clear();
    }
}
impl<C, E> Astar for GenericAstar<C, E>
where
    C: Fn(&Edge) -> u32,
    E: Fn(&Node, &Node) -> u32,
{
    /// Note:
    /// This method uses the graph-structure (offset-array) and works with idx instead of id for better performance.
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path> {
        //----------------------------------------------------------------------------------------//
        // initialization-stuff

        self.resize(graph.nodes.count());

        //----------------------------------------------------------------------------------------//
        // compute

        // prepare first iteration
        self.queue.push(CostNode {
            idx: src.idx(),
            cost: 0,
            estimation: 0,
            pred_idx: None,
        });
        self.costs[src.idx()] = 0;

        //----------------------------------------------------------------------------------------//
        // search for shortest path

        while let Some(current) = self.queue.pop() {
            //------------------------------------------------------------------------------------//
            // if shortest path found
            // -> create path

            if current.idx == dst.idx() {
                let mut cur_idx = current.idx;

                let mut path = Path::from(src.idx(), dst.idx(), &graph);
                path.core.cost = current.cost;
                while let Some(pred_idx) = self.predecessors[cur_idx] {
                    path.core.add_pred_succ(pred_idx, cur_idx);
                    cur_idx = pred_idx;
                }
                // predecessor of src is not set
                // successor of dst is not set
                return Some(path);
            }

            //------------------------------------------------------------------------------------//
            // first occurrence has lowest cost
            // -> check if current has already been visited

            if current.cost > self.costs[current.idx] {
                continue;
            }

            //------------------------------------------------------------------------------------//
            // update costs and add predecessors
            // of nodes, which are dst of current's leaving edges

            let leaving_edges = match graph.fwd_edges.starting_from(current.idx) {
                Some(e) => e,
                None => continue,
            };
            for leaving_edge in leaving_edges {
                let new_cost = current.cost + (self.cost_fn)(leaving_edge);
                if new_cost < self.costs[leaving_edge.dst_idx()] {
                    self.predecessors[leaving_edge.dst_idx()] = Some(current.idx);
                    self.costs[leaving_edge.dst_idx()] = new_cost;

                    let leaving_edge_dst = graph
                        .nodes
                        .get(leaving_edge.dst_idx())
                        .expect("Edge-node should exist since graph should be consistent.");
                    let estimation = (self.estimate_fn)(leaving_edge_dst, dst);
                    self.queue.push(CostNode {
                        idx: leaving_edge.dst_idx(),
                        cost: new_cost,
                        estimation: estimation,
                        pred_idx: Some(current.idx),
                    });
                }
            }
        }

        None
    }
}
