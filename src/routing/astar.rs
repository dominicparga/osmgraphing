use super::paths::Path;
use crate::{
    network::{Graph, Node},
    units::Metric,
};

/// Metric-based trait for computing shortest paths with Astar.
///
/// Get default-implementations from the factory-module in routing.
///
/// Short note about the implementation: The Astar-creation abstracts over the cost-fn and estimation-fn in a handy way, such that they can be created dynamically (via closures) and with factory-methods outside of the Astar, but are hold by the Astar directly, so no extra heap-call from Astar is needed.
/// This is achieved by using a Box for the whole Astar.
///
/// Besides that, implementations of this trait are implemented to keep allocated data for repeaded calls.
/// That's why `&mut self` is required.
pub trait Astar<M>
where
    M: Metric,
{
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path<M>>
    where
        M: Metric;
}

//------------------------------------------------------------------------------------------------//

pub mod unidirectional {
    use super::{Astar, Path};
    use crate::{
        network::{Graph, HalfEdge, Node, NodeIdx},
        units::Metric,
    };
    use std::{collections::BinaryHeap, ops::Add};

    /// A generic Astar-implementation using a cost- and estimation-function.
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
        pub fn new(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E, M> {
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
            self.costs.splice(.., vec![M::inf(); new_len]);
            self.predecessors.splice(.., vec![None; new_len]);

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

            //----------------------------------------------------------------------------------------//
            // search for shortest path

            while let Some(current) = self.queue.pop() {
                //------------------------------------------------------------------------------------//
                // if shortest path found
                // -> create path

                if current.idx == dst.idx() {
                    let mut cur_idx = current.idx;

                    let mut path = Path::with_capacity(src.idx(), dst.idx(), nodes.count());
                    *(path.cost_mut()) = current.cost;
                    while let Some(pred_idx) = self.predecessors[cur_idx.to_usize()] {
                        path.add_pred_succ(pred_idx, cur_idx);
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

    //--------------------------------------------------------------------------------------------//

    #[derive(Copy, Clone)]
    struct CostNode<M>
    where
        M: Metric,
    {
        idx: NodeIdx,
        cost: M,
        estimation: M,
        pred_idx: Option<NodeIdx>,
    }

    mod costnode {
        use super::CostNode;
        use crate::units::Metric;
        use std::{cmp::Ordering, ops::Add};

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
            M: Metric + PartialOrd + Add<M, Output = M>,
        {
            fn partial_cmp(&self, other: &CostNode<M>) -> Option<Ordering> {
                let order =
                    (other.cost + other.estimation).partial_cmp(&(self.cost + self.estimation))?;
                if order == Ordering::Equal {
                    other.idx.partial_cmp(&self.idx)
                } else {
                    Some(order)
                }
            }
        }

        impl<M> Eq for CostNode<M> where M: Metric + Eq + Add<M, Output = M> {}

        impl<M> PartialEq for CostNode<M>
        where
            M: Metric + PartialEq + Add<M, Output = M>,
        {
            fn eq(&self, other: &CostNode<M>) -> bool {
                self.idx == other.idx && (self.cost + self.estimation) == (other.cost + other.estimation)
            }
        }
    }
}

//------------------------------------------------------------------------------------------------//

pub mod bidirectional {
    use super::{Astar, Path};
    use crate::{
        network::{Graph, HalfEdge, Node, NodeIdx},
        units::Metric,
    };
    use std::{collections::BinaryHeap, ops::Add};

    /// Cost-function, Estimation-function and Metric
    pub struct GenericAstar<C, E, M>
    where
        C: Fn(&HalfEdge) -> M,
        E: Fn(&Node, &Node) -> M,
        M: Metric,
    {
        cost_fn: C,
        estimate_fn: E,
        queue: BinaryHeap<CostNode<M>>, // max-heap, but CostNode's natural order is reversed
        // fwd
        fwd_costs: Vec<M>,
        predecessors: Vec<Option<NodeIdx>>,
        is_visited_by_src: Vec<bool>,
        // bwd
        bwd_costs: Vec<M>,
        successors: Vec<Option<NodeIdx>>,
        is_visited_by_dst: Vec<bool>,
    }

    impl<C, E, M> GenericAstar<C, E, M>
    where
        C: Fn(&HalfEdge) -> M,
        E: Fn(&Node, &Node) -> M,
        M: Metric + Ord + Add<M, Output = M>,
    {
        pub fn new(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E, M> {
            GenericAstar {
                cost_fn,
                estimate_fn,
                queue: BinaryHeap::new(),
                // fwd
                fwd_costs: vec![M::inf(); 0],
                predecessors: vec![None; 0],
                is_visited_by_src: vec![false; 0],
                // bwd
                bwd_costs: vec![M::inf(); 0],
                successors: vec![None; 0],
                is_visited_by_dst: vec![false; 0],
            }
        }

        /// Resizes existing datastructures storing routing-data like costs saving re-allocations.
        fn resize(&mut self, new_len: usize) {
            // fwd
            self.fwd_costs.splice(.., vec![M::inf(); new_len]);
            self.predecessors.splice(.., vec![None; new_len]);
            self.is_visited_by_src.splice(.., vec![false; new_len]);
            // bwd
            self.bwd_costs.splice(.., vec![M::inf(); new_len]);
            self.successors.splice(.., vec![None; new_len]);
            self.is_visited_by_dst.splice(.., vec![false; new_len]);

            self.queue.clear();
        }

        /// The given costnode is a meeting-costnode, if it is visited by both, the search starting in src and the search starting in dst.
        fn is_meeting_costnode(&self, costnode: &CostNode<M>) -> bool {
            self.is_visited_by_src[costnode.idx.to_usize()]
                && self.is_visited_by_dst[costnode.idx.to_usize()]
        }

        fn visit(&mut self, costnode: &CostNode<M>) {
            match costnode.direction {
                Direction::FWD => self.is_visited_by_src[costnode.idx.to_usize()] = true,
                Direction::BWD => self.is_visited_by_dst[costnode.idx.to_usize()] = true,
            }
        }

        fn total_cost(&self, costnode: &CostNode<M>) -> M {
            self.fwd_costs[costnode.idx.to_usize()] + self.bwd_costs[costnode.idx.to_usize()]
        }
    }

    impl<C, E, M> Astar<M> for GenericAstar<C, E, M>
    where
        C: Fn(&HalfEdge) -> M,
        E: Fn(&Node, &Node) -> M,
        M: Metric + Ord + Add<M, Output = M>,
    {
        fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path<M>> {
            //------------------------------------------------------------------------------------//
            // initialization-stuff

            let nodes = graph.nodes();
            let fwd_edges = graph.fwd_edges();
            let bwd_edges = graph.bwd_edges();
            self.resize(nodes.count());
            let mut best_meeting: Option<(CostNode<M>, M)> = None;

            //------------------------------------------------------------------------------------//
            // prepare first iteration(s)

            // push src-node
            self.queue.push(CostNode {
                idx: src.idx(),
                cost: M::zero(),
                estimation: M::zero(),
                pred_idx: None,
                direction: Direction::FWD,
            });
            // push dst-node
            self.queue.push(CostNode {
                idx: dst.idx(),
                cost: M::zero(),
                estimation: M::zero(),
                pred_idx: None,
                direction: Direction::BWD,
            });
            // update fwd-stats
            self.fwd_costs[src.idx().to_usize()] = M::zero();
            // update bwd-stats
            self.bwd_costs[dst.idx().to_usize()] = M::zero();

            //------------------------------------------------------------------------------------//
            // search for shortest path

            while let Some(current) = self.queue.pop() {
                // if path is found
                // -> remember best meeting-node
                self.visit(&current);
                if self.is_meeting_costnode(&current) {
                    if let Some((_meeting_node, total_cost)) = best_meeting {
                        // if meeting-node is already found
                        // check if new meeting-node is better
                        let new_total_cost = self.total_cost(&current);
                        if new_total_cost < total_cost {
                            best_meeting = Some((current, new_total_cost));
                        }
                    } else {
                        best_meeting = Some((current, self.total_cost(&current)));
                    }
                }

                // distinguish between fwd and bwd
                let (xwd_costs, xwd_edges, xwd_predecessors, xwd_dst) = match current.direction {
                    Direction::FWD => (
                        &mut self.fwd_costs,
                        &fwd_edges,
                        &mut self.predecessors,
                        &dst,
                    ),
                    Direction::BWD => (&mut self.bwd_costs, &bwd_edges, &mut self.successors, &src),
                };

                // first occurrence has lowest cost
                // -> check if current has already been expanded
                if current.cost > xwd_costs[current.idx.to_usize()] {
                    continue;
                }

                // update costs and add predecessors
                // of nodes, which are dst of current's leaving edges
                let leaving_edges = match xwd_edges.starting_from(current.idx) {
                    Some(e) => e,
                    None => continue,
                };
                for leaving_edge in leaving_edges {
                    let new_cost = current.cost + (self.cost_fn)(&leaving_edge);
                    if new_cost < xwd_costs[leaving_edge.dst_idx().to_usize()] {
                        xwd_predecessors[leaving_edge.dst_idx().to_usize()] = Some(current.idx);
                        xwd_costs[leaving_edge.dst_idx().to_usize()] = new_cost;

                        // if path is found
                        // -> Run until queue is empty
                        //    since the shortest path could have longer hop-distance
                        //    with shorter weight-distance than currently found node.
                        if best_meeting.is_none() {
                            let leaving_edge_dst = nodes.create(leaving_edge.dst_idx());
                            let estimation = (self.estimate_fn)(&leaving_edge_dst, xwd_dst);
                            self.queue.push(CostNode {
                                idx: leaving_edge.dst_idx(),
                                cost: new_cost,
                                estimation: estimation,
                                pred_idx: Some(current.idx),
                                direction: current.direction,
                            });
                        }
                    }
                }
            }

            //------------------------------------------------------------------------------------//
            // create path if found

            if let Some((meeting_node, total_cost)) = best_meeting {
                let mut path = Path::with_capacity(src.idx(), dst.idx(), nodes.count());
                *(path.cost_mut()) = total_cost;

                // iterate backwards over fwd-path
                let mut cur_idx = meeting_node.idx;
                while let Some(pred_idx) = self.predecessors[cur_idx.to_usize()] {
                    path.add_pred_succ(pred_idx, cur_idx);
                    cur_idx = pred_idx;
                }

                // iterate backwards over bwd-path
                let mut cur_idx = meeting_node.idx;
                while let Some(succ_idx) = self.successors[cur_idx.to_usize()] {
                    path.add_pred_succ(cur_idx, succ_idx);
                    cur_idx = succ_idx;
                }

                // predecessor of src is not set
                // successor of dst is not set
                Some(path)
            } else {
                None
            }
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Copy, Clone)]
    struct CostNode<M>
    where
        M: Metric,
    {
        idx: NodeIdx,
        cost: M,
        estimation: M,
        pred_idx: Option<NodeIdx>,
        direction: Direction,
    }

    #[derive(Copy, Clone, Debug)]
    enum Direction {
        FWD,
        BWD,
    }

    mod costnode {
        use super::{CostNode, Direction};
        use crate::units::Metric;
        use std::{cmp::Ordering, fmt, fmt::Display, ops::Add};

        impl<M> Display for CostNode<M>
        where
            M: Metric,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{{ idx: {}, cost: {}, esti: {}, pred-idx: {}, {} }}",
                    self.idx,
                    self.cost,
                    self.estimation,
                    match self.pred_idx {
                        Some(idx) => format!("{}", idx),
                        None => String::from("None"),
                    },
                    self.direction
                )
            }
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
                    .then_with(|| other.direction.cmp(&self.direction))
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

        impl Display for Direction {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        Direction::FWD => "forward",
                        Direction::BWD => "backward",
                    }
                )
            }
        }

        impl Ord for Direction {
            fn cmp(&self, other: &Direction) -> Ordering {
                let self_value = match self {
                    Direction::FWD => 1,
                    Direction::BWD => -1,
                };
                let other_value = match other {
                    Direction::FWD => 1,
                    Direction::BWD => -1,
                };
                self_value.cmp(&other_value)
            }
        }

        impl PartialOrd for Direction {
            fn partial_cmp(&self, other: &Direction) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Eq for Direction {}

        impl PartialEq for Direction {
            fn eq(&self, other: &Direction) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }
    }
}
