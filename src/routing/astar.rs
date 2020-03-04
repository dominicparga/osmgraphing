use super::paths::Path;
use crate::network::{Graph, Node, NodeIdx};

/// Metric-based trait for computing shortest paths with Astar.
///
/// Get default-implementations from the factory-module in routing.
///
/// Short note about the implementation: The Astar-creation abstracts over the cost-fn and estimation-fn in a handy way, such that they can be created dynamically (via closures) and with factory-methods outside of the Astar, but are hold by the Astar directly, so no extra heap-call from Astar is needed.
/// This is achieved by using a Box for the whole Astar.
///
/// Besides that, implementations of this trait are implemented to keep allocated data for repeaded calls.
/// That's why `&mut self` is required.
pub trait Astar {
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path<f32>>;
}

pub mod unidirectional {
    use super::{Astar, CostNode, Path};
    use crate::network::{Graph, HalfEdge, Node, NodeIdx};
    use std::{cmp::Reverse, collections::BinaryHeap};

    /// A generic Astar-implementation using a cost- and estimation-function.
    pub struct GenericAstar<C, E>
    where
        C: Fn(&HalfEdge) -> f32,
        E: Fn(&Node, &Node) -> f32,
    {
        cost_fn: C,
        estimate_fn: E,
        costs: Vec<f32>,
        predecessors: Vec<Option<NodeIdx>>,
        queue: BinaryHeap<Reverse<CostNode>>, // max-heap, but CostNode's natural order is reversed
    }

    impl<C, E> GenericAstar<C, E>
    where
        C: Fn(&HalfEdge) -> f32,
        E: Fn(&Node, &Node) -> f32,
    {
        pub fn new(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E> {
            GenericAstar {
                cost_fn,
                estimate_fn,
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

    impl<C, E> Astar for GenericAstar<C, E>
    where
        C: Fn(&HalfEdge) -> f32,
        E: Fn(&Node, &Node) -> f32,
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
            self.queue.push(Reverse(CostNode {
                idx: src.idx(),
                cost: 0.0,
                estimation: 0.0,
            }));
            self.costs[*src.idx()] = 0.0;

            //----------------------------------------------------------------------------------------//
            // search for shortest path

            while let Some(Reverse(current)) = self.queue.pop() {
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

                        let leaving_edge_of_dst = nodes.create(leaving_edge.dst_idx());
                        let estimation = (self.estimate_fn)(&leaving_edge_of_dst, dst);
                        self.queue.push(Reverse(CostNode {
                            idx: leaving_edge.dst_idx(),
                            cost: new_cost,
                            estimation: estimation,
                        }));
                    }
                }
            }

            None
        }
    }
}

pub mod bidirectional {
    use super::{Astar, BiCostNode, CostNode, Direction, Path};
    use crate::network::{Graph, HalfEdge, Node, NodeIdx};
    use std::{cmp::Reverse, collections::BinaryHeap};

    /// Cost-function, Estimation-function and Metric
    pub struct GenericAstar<C, E>
    where
        C: Fn(&HalfEdge) -> f32,
        E: Fn(&Node, &Node) -> f32,
    {
        cost_fn: C,
        estimate_fn: E,
        queue: BinaryHeap<Reverse<BiCostNode>>, // max-heap, but CostNode's natural order is reversed
        // fwd
        fwd_costs: Vec<f32>,
        predecessors: Vec<Option<NodeIdx>>,
        is_visited_by_src: Vec<bool>,
        // bwd
        bwd_costs: Vec<f32>,
        successors: Vec<Option<NodeIdx>>,
        is_visited_by_dst: Vec<bool>,
    }

    impl<C, E> GenericAstar<C, E>
    where
        C: Fn(&HalfEdge) -> f32,
        E: Fn(&Node, &Node) -> f32,
    {
        pub fn new(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E> {
            GenericAstar {
                cost_fn,
                estimate_fn,
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
        fn is_meeting_costnode(&self, costnode: &BiCostNode) -> bool {
            self.is_visited_by_src[*costnode.core.idx] && self.is_visited_by_dst[*costnode.core.idx]
        }

        fn visit(&mut self, costnode: &BiCostNode) {
            match costnode.direction {
                Direction::FWD => self.is_visited_by_src[*costnode.core.idx] = true,
                Direction::BWD => self.is_visited_by_dst[*costnode.core.idx] = true,
            }
        }

        fn total_cost(&self, costnode: &BiCostNode) -> f32 {
            self.fwd_costs[*costnode.core.idx] + self.bwd_costs[*costnode.core.idx]
        }
    }

    impl<C, E> Astar for GenericAstar<C, E>
    where
        C: Fn(&HalfEdge) -> f32,
        E: Fn(&Node, &Node) -> f32,
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
            let mut best_meeting: Option<(BiCostNode, f32)> = None;

            //------------------------------------------------------------------------------------//
            // prepare first iteration(s)

            // push src-node
            self.queue.push(Reverse(BiCostNode {
                core: CostNode {
                    idx: src.idx(),
                    cost: 0.0,
                    estimation: 0.0,
                },
                direction: Direction::FWD,
            }));
            // push dst-node
            self.queue.push(Reverse(BiCostNode {
                core: CostNode {
                    idx: dst.idx(),
                    cost: 0.0,
                    estimation: 0.0,
                },
                direction: Direction::BWD,
            }));
            // update fwd-stats
            self.fwd_costs[*src.idx()] = 0.0;
            // update bwd-stats
            self.bwd_costs[*dst.idx()] = 0.0;

            //------------------------------------------------------------------------------------//
            // search for shortest path

            while let Some(Reverse(current)) = self.queue.pop() {
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
                if current.core.cost > xwd_costs[*current.core.idx] {
                    continue;
                }

                // update costs and add predecessors
                // of nodes, which are dst of current's leaving edges
                let leaving_edges = match xwd_edges.starting_from(current.core.idx) {
                    Some(e) => e,
                    None => continue,
                };
                for leaving_edge in leaving_edges {
                    let new_cost = current.core.cost + (self.cost_fn)(&leaving_edge);
                    if new_cost < xwd_costs[*leaving_edge.dst_idx()] {
                        xwd_predecessors[*leaving_edge.dst_idx()] = Some(current.core.idx);
                        xwd_costs[*leaving_edge.dst_idx()] = new_cost;

                        // if path is found
                        // -> Run until queue is empty
                        //    since the shortest path could have longer hop-distance
                        //    with shorter weight-distance than currently found node.
                        if best_meeting.is_none() {
                            let leaving_edge_dst = nodes.create(leaving_edge.dst_idx());
                            let estimation = (self.estimate_fn)(&leaving_edge_dst, xwd_dst);
                            self.queue.push(Reverse(BiCostNode {
                                core: CostNode {
                                    idx: leaving_edge.dst_idx(),
                                    cost: new_cost,
                                    estimation: estimation,
                                },
                                direction: current.direction,
                            }));
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
                let mut cur_idx = meeting_node.core.idx;
                while let Some(pred_idx) = self.predecessors[*cur_idx] {
                    path.add_pred_succ(pred_idx, cur_idx);
                    cur_idx = pred_idx;
                }

                // iterate backwards over bwd-path
                let mut cur_idx = meeting_node.core.idx;
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
}

#[derive(Copy, Clone)]
struct CostNode {
    idx: NodeIdx,
    cost: f32,
    estimation: f32,
}

mod costnode {
    use super::CostNode;
    use crate::helpers::Approx;
    use std::{
        cmp::Ordering,
        fmt::{self, Display},
    };

    impl Display for CostNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{{ idx: {}, cost: {}, esti: {} }}",
                self.idx, self.cost, self.estimation,
            )
        }
    }

    impl Ord for CostNode {
        fn cmp(&self, other: &CostNode) -> Ordering {
            (self.cost + self.estimation)
                .approx_cmp(&(other.cost + other.estimation))
                .then_with(|| self.idx.cmp(&other.idx))
        }
    }

    impl PartialOrd for CostNode {
        fn partial_cmp(&self, other: &CostNode) -> Option<Ordering> {
            Some(
                (self.cost + self.estimation)
                    .approx_partial_cmp(&(other.cost + other.estimation))?
                    .then_with(|| self.idx.cmp(&other.idx)),
            )
        }
    }

    impl Eq for CostNode {}

    impl PartialEq for CostNode {
        fn eq(&self, other: &CostNode) -> bool {
            self.idx == other.idx
                && (self.cost + self.estimation).approx_eq(&(other.cost + other.estimation))
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    FWD,
    BWD,
}

#[derive(Copy, Clone)]
struct BiCostNode {
    core: CostNode,
    direction: Direction,
}

mod bicostnode {
    use super::{BiCostNode, Direction};
    use std::{cmp::Ordering, fmt, fmt::Display};

    impl Display for BiCostNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{{ idx: {}, cost: {}, esti: {}, {} }}",
                self.core.idx, self.core.cost, self.core.estimation, self.direction
            )
        }
    }

    impl BiCostNode {}

    impl Ord for BiCostNode {
        fn cmp(&self, other: &BiCostNode) -> Ordering {
            self.core
                .cmp(&other.core)
                .then_with(|| self.direction.cmp(&other.direction))
        }
    }

    impl PartialOrd for BiCostNode {
        fn partial_cmp(&self, other: &BiCostNode) -> Option<Ordering> {
            Some(
                self.core
                    .partial_cmp(&other.core)?
                    .then_with(|| self.direction.cmp(&other.direction)),
            )
        }
    }

    impl Eq for BiCostNode {}

    impl PartialEq for BiCostNode {
        fn eq(&self, other: &BiCostNode) -> bool {
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
