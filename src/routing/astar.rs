use crate::{
    network::{Graph, Node, NodeIdx},
    units::Metric,
};

pub trait Astar<M>
where
    M: Metric,
{
    fn compute_best_path(&mut self, src: &Node, dst: &Node, graph: &Graph) -> Option<Path<M>>
    where
        M: Metric;
}

//------------------------------------------------------------------------------------------------//
// path

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
}

mod paths {
    use super::Path;
    use crate::{network::NodeIdx, units::Metric};
    use std::collections::HashMap;

    impl<M> Path<M>
    where
        M: Metric,
    {
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
}

//------------------------------------------------------------------------------------------------//

pub mod unidirectional {
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
            let _bwd_edges = graph.bwd_edges();
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
        pub fn from(cost_fn: C, estimate_fn: E) -> GenericAstar<C, E, M> {
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

            // create path if found
            if let Some((meeting_node, total_cost)) = best_meeting {
                let mut path = Path::from(src.idx(), dst.idx(), &graph);
                path.core.cost = total_cost;

                // iterate backwards over fwd-path
                let mut cur_idx = meeting_node.idx;
                while let Some(pred_idx) = self.predecessors[cur_idx.to_usize()] {
                    path.core.add_pred_succ(pred_idx, cur_idx);
                    cur_idx = pred_idx;
                }

                // iterate backwards over bwd-path
                let mut cur_idx = meeting_node.idx;
                while let Some(succ_idx) = self.successors[cur_idx.to_usize()] {
                    path.core.add_pred_succ(cur_idx, succ_idx);
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
