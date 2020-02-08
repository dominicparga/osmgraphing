//------------------------------------------------------------------------------------------------//
// own modules

mod paths;

//------------------------------------------------------------------------------------------------//
// other modules

use crate::{
    network::{Graph, HalfEdge, Node, NodeIdx},
    units::Metric,
};
use std::{cmp::Ordering, collections::BinaryHeap, ops::Add};

//------------------------------------------------------------------------------------------------//
// Path

/// A path from a src to a dst storing predecessors and successors.
///
/// The implementation bases either on vectors or on hashmaps.
/// Some words about it without doing a benchmark:
/// - Since the vector-approach stores two fully allocated vectors, it probably consumes more memory than the hashmap-approach.
/// - Just by looking at resulting times of long paths (~600 km) in Germany, the hashmap-approach seems to be slightly better in performance, but both approaches take around 7 seconds for it.
#[derive(Clone)]
pub struct Path<M>
where
    M: Metric,
{
    // core: paths::VecPath<M>,
    core: paths::HashPath<M>,
}

impl<M> Path<M>
where
    M: Metric,
{
    fn from(src_idx: NodeIdx, dst_idx: NodeIdx, _graph: &Graph) -> Self {
        // let core = paths::VecPath::with_capacity(src_idx, dst_idx, graph.nodes().count());
        let core = paths::HashPath::new(src_idx, dst_idx);
        Path { core }
    }

    //--------------------------------------------------------------------------------------------//

    pub fn src_idx(&self) -> NodeIdx {
        self.core.src_idx
    }

    pub fn dst_idx(&self) -> NodeIdx {
        self.core.dst_idx
    }

    pub fn cost(&self) -> M {
        self.core.cost
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
// CostNode

#[derive(Copy, Clone)]
struct CostNode<M>
where
    M: Metric,
{
    pub idx: NodeIdx,
    pub cost: M,
    pub estimation: M,
    pub pred_idx: Option<NodeIdx>,
}

impl<M> Ord for CostNode<M>
where
    M: Metric + Ord + Add<M, Output = M>,
{
    fn cmp(&self, other: &CostNode<M>) -> Ordering {
        // (1) cost in float, but cmp uses only m, which is ok
        // (2) inverse order since BinaryHeap is max-heap, but min-heap is needed
        (other.cost + other.estimation)
            .cmp(&(self.cost + self.estimation))
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

impl<M> PartialOrd for CostNode<M>
where
    M: Metric + Ord + Add<M, Output = M>,
{
    fn partial_cmp(&self, other: &CostNode<M>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M> Eq for CostNode<M> where M: Metric + Ord + Add<M, Output = M> {}

impl<M> PartialEq for CostNode<M>
where
    M: Metric + Ord + Add<M, Output = M>,
{
    fn eq(&self, other: &CostNode<M>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

//------------------------------------------------------------------------------------------------//
// Astar

pub trait Astar<M>
where
    M: Metric,
{
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path<M>>
    where
        M: Metric;
}

//------------------------------------------------------------------------------------------------//
// GenericAstar

/// Cost-function, Estimation-function and Metric
pub struct GenericAstar<C, E, M>
where
    C: Fn(&HalfEdge) -> M,
    E: Fn(&Node, &Node) -> M,
    M: Metric,
{
    cost_fn: C,
    estimate_fn: E,
    costs: Vec<M>,
    predecessors: Vec<Option<NodeIdx>>,
    queue: BinaryHeap<CostNode<M>>, // max-heap, but CostNode's natural order is reversed
}

impl<C, E, M> GenericAstar<C, E, M>
where
    C: Fn(&HalfEdge) -> M,
    E: Fn(&Node, &Node) -> M,
    M: Metric + Ord + Add<M, Output = M>,
{
    pub fn from(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E, M> {
        GenericAstar {
            cost_fn,
            estimate_fn,
            costs: vec![M::inf(); 0],
            predecessors: vec![None; 0],
            queue: BinaryHeap::new(),
        }
    }

    /// Resizes existing datastructures storing routing-data like costs saving re-allocations.
    fn resize(&mut self, new_len: usize) {
        let old_len = self.costs.len();
        let min_len = std::cmp::min(old_len, new_len);
        for i in 0..min_len {
            self.costs[i] = M::inf();
            self.predecessors[i] = None;
        }
        self.costs.resize(new_len, M::inf());
        self.predecessors.resize(new_len, None);

        self.queue.clear();
    }
}

impl<C, E, M> Astar<M> for GenericAstar<C, E, M>
where
    C: Fn(&HalfEdge) -> M,
    E: Fn(&Node, &Node) -> M,
    M: Metric + Ord + Add<M, Output = M>,
{
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path<M>> {
        //----------------------------------------------------------------------------------------//
        // initialization-stuff

        let nodes = graph.nodes();
        let fwd_edges = graph.fwd_edges();
        let bwd_edges = graph.bwd_edges();
        self.resize(nodes.count());

        //----------------------------------------------------------------------------------------//
        // prepare first iteration(s)

        // push src-node
        self.queue.push(CostNode {
            idx: src.idx(),
            cost: M::zero(),
            estimation: M::zero(),
            pred_idx: None,
        });
        self.costs[src.idx().to_usize()] = M::zero();
        // // push dst-node
        // self.queue.push(CostNode {
        //     idx: dst.idx(),
        //     cost: M::zero(),
        //     estimation: M::zero(),
        //     pred_idx: None,
        // });
        // self.costs[dst.idx().to_usize()] = M::zero();

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
                while let Some(pred_idx) = self.predecessors[cur_idx.to_usize()] {
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

            if current.cost > self.costs[current.idx.to_usize()] {
                continue;
            }

            //------------------------------------------------------------------------------------//
            // update costs and add predecessors
            // of nodes, which are dst of current's leaving edges

            let leaving_edges = match fwd_edges.starting_from(current.idx) {
                Some(e) => e,
                None => continue,
            };
            for leaving_edge in leaving_edges {
                let new_cost = current.cost + (self.cost_fn)(&leaving_edge);
                if new_cost < self.costs[leaving_edge.dst_idx().to_usize()] {
                    self.predecessors[leaving_edge.dst_idx().to_usize()] = Some(current.idx);
                    self.costs[leaving_edge.dst_idx().to_usize()] = new_cost;

                    let leaving_edge_of_dst = nodes.create(leaving_edge.dst_idx());
                    let estimation = (self.estimate_fn)(&leaving_edge_of_dst, dst);
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
