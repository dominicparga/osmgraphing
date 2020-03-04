pub use super::astar::Astar;
use super::paths::Path;

//------------------------------------------------------------------------------------------------//

pub mod unidirectional {
    use super::{Astar, Path};
    use crate::network::{Graph, HalfEdge, Node, NodeIdx};
    use std::collections::BinaryHeap;

    /// Cost-function, Estimation-function and Metric
    pub struct GenericDijkstra<C>
    where
        C: Fn(&HalfEdge) -> f32,
    {
        cost_fn: C,
        costs: Vec<f32>,
        predecessors: Vec<Option<NodeIdx>>,
        queue: BinaryHeap<CostNode>, // max-heap, but CostNode's natural order is reversed
    }

    impl<C> GenericDijkstra<C>
    where
        C: Fn(&HalfEdge) -> f32,
    {
        pub fn new(cost_fn: C) -> GenericDijkstra<C> {
            GenericDijkstra {
                cost_fn,
                costs: vec![std::f32::INFINITY; 0],
                predecessors: vec![None; 0],
                queue: BinaryHeap::new(),
            }
        }

        /// Resizes existing datastructures storing routing-data like costs saving re-allocations.
        fn resize(&mut self, new_len: usize) {
            self.costs.splice(.., vec![std::f32::INFINITY; new_len]);
            self.predecessors.splice(.., vec![None; new_len]);

            self.queue.clear();
        }
    }

    impl<C> Astar for GenericDijkstra<C>
    where
        C: Fn(&HalfEdge) -> f32,
    {
        fn compute_best_path(
            &mut self,
            src: &Node,
            dst: &Node,
            graph: &Graph,
        ) -> Option<Path<f32>> {
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
                cost: 0.0,
            });
            self.costs[*src.idx()] = 0.0;

            //----------------------------------------------------------------------------------------//
            // search for shortest path

            while let Some(current) = self.queue.pop() {
                //------------------------------------------------------------------------------------//
                // if shortest path found
                // -> create path

                if current.idx == dst.idx() {
                    let mut cur_idx = current.idx;

                    let mut path = Path::with_capacity(
                        src.idx(),
                        dst.idx(),
                        std::f32::INFINITY,
                        nodes.count(),
                    );
                    *(path.cost_mut()) = current.cost;
                    while let Some(pred_idx) = self.predecessors[*cur_idx] {
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

                if current.cost > self.costs[*current.idx] {
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
                    if new_cost < self.costs[*leaving_edge.dst_idx()] {
                        self.predecessors[*leaving_edge.dst_idx()] = Some(current.idx);
                        self.costs[*leaving_edge.dst_idx()] = new_cost;

                        self.queue.push(CostNode {
                            idx: leaving_edge.dst_idx(),
                            cost: new_cost,
                        });
                    }
                }
            }

            None
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Copy, Clone)]
    struct CostNode {
        idx: NodeIdx,
        cost: f32,
    }

    mod costnode {
        use super::CostNode;
        use std::cmp::Ordering;

        impl Ord for CostNode {
            fn cmp(&self, other: &CostNode) -> Ordering {
                // inverse order since BinaryHeap is max-heap, but min-heap is needed
                other
                    .cost
                    .partial_cmp(&(self.cost))
                    .expect("Didn't expect NaN when comparing cost-nodes!")
                    .then_with(|| other.idx.cmp(&self.idx))
            }
        }

        impl PartialOrd for CostNode {
            fn partial_cmp(&self, other: &CostNode) -> Option<Ordering> {
                Some(
                    other
                        .cost
                        .partial_cmp(&self.cost)?
                        .then_with(|| other.idx.cmp(&self.idx)),
                )
            }
        }

        impl Eq for CostNode {}

        impl PartialEq for CostNode {
            fn eq(&self, other: &CostNode) -> bool {
                self.idx == other.idx && self.cost == other.cost
            }
        }
    }
}

//------------------------------------------------------------------------------------------------//

pub mod bidirectional {
    use super::{Astar, Path};
    use crate::network::{Graph, HalfEdge, Node, NodeIdx};
    use std::collections::BinaryHeap;

    /// Cost-function, Estimation-function and Metric
    pub struct GenericDijkstra<C>
    where
        C: Fn(&HalfEdge) -> f32,
    {
        cost_fn: C,
        queue: BinaryHeap<CostNode>, // max-heap, but CostNode's natural order is reversed
        // fwd
        fwd_costs: Vec<f32>,
        predecessors: Vec<Option<NodeIdx>>,
        is_visited_by_src: Vec<bool>,
        // bwd
        bwd_costs: Vec<f32>,
        successors: Vec<Option<NodeIdx>>,
        is_visited_by_dst: Vec<bool>,
    }

    impl<C> GenericDijkstra<C>
    where
        C: Fn(&HalfEdge) -> f32,
    {
        pub fn new(cost_fn: C) -> GenericDijkstra<C> {
            GenericDijkstra {
                cost_fn,
                queue: BinaryHeap::new(),
                // fwd
                fwd_costs: vec![std::f32::INFINITY; 0],
                predecessors: vec![None; 0],
                is_visited_by_src: vec![false; 0],
                // bwd
                bwd_costs: vec![std::f32::INFINITY; 0],
                successors: vec![None; 0],
                is_visited_by_dst: vec![false; 0],
            }
        }

        /// Resizes existing datastructures storing routing-data like costs saving re-allocations.
        fn resize(&mut self, new_len: usize) {
            // fwd
            self.fwd_costs.splice(.., vec![std::f32::INFINITY; new_len]);
            self.predecessors.splice(.., vec![None; new_len]);
            self.is_visited_by_src.splice(.., vec![false; new_len]);
            // bwd
            self.bwd_costs.splice(.., vec![std::f32::INFINITY; new_len]);
            self.successors.splice(.., vec![None; new_len]);
            self.is_visited_by_dst.splice(.., vec![false; new_len]);

            self.queue.clear();
        }

        /// The given costnode is a meeting-costnode, if it is visited by both, the search starting in src and the search starting in dst.
        fn is_meeting_costnode(&self, costnode: &CostNode) -> bool {
            self.is_visited_by_src[*costnode.idx] && self.is_visited_by_dst[*costnode.idx]
        }

        fn visit(&mut self, costnode: &CostNode) {
            match costnode.direction {
                Direction::FWD => self.is_visited_by_src[*costnode.idx] = true,
                Direction::BWD => self.is_visited_by_dst[*costnode.idx] = true,
            }
        }

        fn total_cost(&self, costnode: &CostNode) -> f32 {
            self.fwd_costs[*costnode.idx] + self.bwd_costs[*costnode.idx]
        }
    }

    impl<C> Astar for GenericDijkstra<C>
    where
        C: Fn(&HalfEdge) -> f32,
    {
        fn compute_best_path(
            &mut self,
            src: &Node,
            dst: &Node,
            graph: &Graph,
        ) -> Option<Path<f32>> {
            //------------------------------------------------------------------------------------//
            // initialization-stuff

            let nodes = graph.nodes();
            let fwd_edges = graph.fwd_edges();
            let bwd_edges = graph.bwd_edges();
            self.resize(nodes.count());
            let mut best_meeting: Option<(CostNode, f32)> = None;

            //------------------------------------------------------------------------------------//
            // prepare first iteration(s)

            // push src-node
            self.queue.push(CostNode {
                idx: src.idx(),
                cost: 0.0,
                pred_idx: None,
                direction: Direction::FWD,
            });
            // push dst-node
            self.queue.push(CostNode {
                idx: dst.idx(),
                cost: 0.0,
                pred_idx: None,
                direction: Direction::BWD,
            });
            // update fwd-stats
            self.fwd_costs[*src.idx()] = 0.0;
            // update bwd-stats
            self.bwd_costs[*dst.idx()] = 0.0;

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
                let (xwd_costs, xwd_edges, xwd_predecessors) = match current.direction {
                    Direction::FWD => (&mut self.fwd_costs, &fwd_edges, &mut self.predecessors),
                    Direction::BWD => (&mut self.bwd_costs, &bwd_edges, &mut self.successors),
                };

                // first occurrence has lowest cost
                // -> check if current has already been expanded
                if current.cost > xwd_costs[*current.idx] {
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
                    if new_cost < xwd_costs[*leaving_edge.dst_idx()] {
                        xwd_predecessors[*leaving_edge.dst_idx()] = Some(current.idx);
                        xwd_costs[*leaving_edge.dst_idx()] = new_cost;

                        // if path is found
                        // -> Run until queue is empty
                        //    since the shortest path could have longer hop-distance
                        //    with shorter weight-distance than currently found node.
                        if best_meeting.is_none() {
                            self.queue.push(CostNode {
                                idx: leaving_edge.dst_idx(),
                                cost: new_cost,
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
                let mut path =
                    Path::with_capacity(src.idx(), dst.idx(), std::f32::INFINITY, nodes.count());
                *(path.cost_mut()) = total_cost;

                // iterate backwards over fwd-path
                let mut cur_idx = meeting_node.idx;
                while let Some(pred_idx) = self.predecessors[*cur_idx] {
                    path.add_pred_succ(pred_idx, cur_idx);
                    cur_idx = pred_idx;
                }

                // iterate backwards over bwd-path
                let mut cur_idx = meeting_node.idx;
                while let Some(succ_idx) = self.successors[*cur_idx] {
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
    struct CostNode {
        idx: NodeIdx,
        cost: f32,
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
        use std::{cmp::Ordering, fmt, fmt::Display};

        impl Display for CostNode {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{{ idx: {}, cost: {}, pred-idx: {}, {} }}",
                    self.idx,
                    self.cost,
                    match self.pred_idx {
                        Some(idx) => format!("{}", idx),
                        None => String::from("None"),
                    },
                    self.direction
                )
            }
        }

        impl Ord for CostNode {
            fn cmp(&self, other: &CostNode) -> Ordering {
                // (1) cost in float, but cmp uses only m, which is ok
                // (2) inverse order since BinaryHeap is max-heap, but min-heap is needed
                other
                    .cost
                    .partial_cmp(&(self.cost))
                    .expect("Didn't expect NaN when comparing cost-nodes!")
                    .then_with(|| other.idx.cmp(&self.idx))
                    .then_with(|| other.direction.cmp(&self.direction))
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
