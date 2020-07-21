use super::paths::Path;
use crate::{
    configs::routing::Config,
    defaults::routing::IS_USING_CH_LEVEL_SPEEDUP,
    helpers,
    network::{EdgeIdx, Graph, NodeIdx},
};
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Copy, Clone)]
pub struct Query<'a> {
    pub src_idx: NodeIdx,
    pub dst_idx: NodeIdx,
    pub graph: &'a Graph,
    pub routing_cfg: &'a Config,
}

/// A bidirectional implementation of Dijkstra's algorithm.
/// This implementation reuses the underlying datastructures to speedup multiple computations.
///
/// This implementation is correct for contracted and non-contracted graphs.
/// However, the performance highly depends on a flag in the config, which has to be provided when computing the best path.
pub struct Dijkstra {
    // general
    is_ch_dijkstra: bool,
    // data-structures for a query
    queue: BinaryHeap<Reverse<CostNode>>,
    costs: [Vec<f64>; 2],
    predecessors: [Vec<Option<EdgeIdx>>; 2],
    is_visited: [Vec<bool>; 2],
    has_found_best_meeting_node: [bool; 2],
}

impl Dijkstra {
    pub fn new() -> Dijkstra {
        Dijkstra {
            is_ch_dijkstra: false,
            queue: BinaryHeap::new(),
            costs: [Vec::new(), Vec::new()],
            predecessors: [Vec::new(), Vec::new()],
            is_visited: [Vec::new(), Vec::new()],
            has_found_best_meeting_node: [false, false],
        }
    }

    fn fwd_idx(&self) -> usize {
        0
    }

    fn bwd_idx(&self) -> usize {
        1
    }

    fn dir_idx(&self, direction: Direction) -> usize {
        match direction {
            Direction::FWD => self.fwd_idx(),
            Direction::BWD => self.bwd_idx(),
        }
    }

    fn opp_dir_idx(&self, direction: Direction) -> usize {
        match direction {
            Direction::FWD => self.bwd_idx(),
            Direction::BWD => self.fwd_idx(),
        }
    }

    /// Resizes existing datastructures storing routing-data, like costs, saving re-allocations.
    fn init_query(&mut self, new_len: usize) {
        // fwd and bwd
        for dir in vec![Direction::FWD, Direction::BWD] {
            let dir = self.dir_idx(dir);
            self.costs[dir].resize(new_len, std::f64::INFINITY);
            self.costs[dir]
                .iter_mut()
                .for_each(|c| *c = std::f64::INFINITY);

            self.predecessors[dir].resize(new_len, None);
            self.predecessors[dir].iter_mut().for_each(|p| *p = None);

            if !self.is_ch_dijkstra {
                self.is_visited[dir].resize(new_len, false);
                self.is_visited[dir].iter_mut().for_each(|v| *v = false);
            }

            self.has_found_best_meeting_node[dir] = false;
        }

        self.queue.clear();
    }

    fn visit(&mut self, costnode: &CostNode) {
        if !self.is_ch_dijkstra {
            self.is_visited[self.dir_idx(costnode.direction)][*costnode.idx] = true
        }
    }

    /// This method is optimized by assuming that the provided CostNode has already been visited.
    fn is_meeting_costnode(&self, costnode: &CostNode) -> bool {
        // Costs are updated when costnodes are enqueued, but costnodes have to be dequeued
        // before they can be considered as visited (for bidir Dijkstra).
        if self.is_ch_dijkstra {
            self.costs[self.opp_dir_idx(costnode.direction)][*costnode.idx] != std::f64::INFINITY
        } else {
            // The CostNode has already been dequeued, which is the reason for this assertion.
            debug_assert!(
                self.is_visited[self.dir_idx(costnode.direction)][*costnode.idx],
                "CostNode should already be visited."
            );
            self.is_visited[self.opp_dir_idx(costnode.direction)][*costnode.idx]
        }
    }

    /// This method returns true, if both queries can't be better.
    fn has_found_best_meeting_node(&self) -> bool {
        self.has_found_best_meeting_node[self.fwd_idx()]
            && self.has_found_best_meeting_node[self.bwd_idx()]
    }

    /// Returns true, if the provided costnode's cost are better than the registered cost for this
    /// node-idx (and for this query-direction).
    fn has_costnode_improved(&self, costnode: &CostNode) -> bool {
        costnode.cost <= self.costs[self.dir_idx(costnode.direction)][*costnode.idx]
    }

    /// Returns the cost of a path, so cost(src->v) + cost(v->dst)
    fn total_cost(&self, costnode: &CostNode) -> f64 {
        self.costs[self.fwd_idx()][*costnode.idx] + self.costs[self.bwd_idx()][*costnode.idx]
    }

    /// None means no path exists, whereas an empty path is a path from a node to itself.
    ///
    /// ATTENTION!
    /// If any alpha-value in the routing-config is negative, or any metric in the graph is negative, this method won't terminate.
    pub fn compute_best_path(&mut self, query: Query) -> Option<Path> {
        debug_assert!(
            query.routing_cfg.alphas.len() > 0,
            "Best path should be computed, but no alphas are specified."
        );

        for alpha in query.routing_cfg.alphas.iter() {
            // Dijkstra would not terminate with negative weights
            // -> no path found
            if alpha < &0.0 {
                return None;
            }
        }

        self.is_ch_dijkstra = query.routing_cfg.is_ch_dijkstra;

        //----------------------------------------------------------------------------------------//
        // initialization-stuff

        let nodes = query.graph.nodes();
        let xwd_edges = {
            debug_assert_eq!(
                0,
                self.dir_idx(Direction::FWD),
                "Direction-Idx of FWD is expected to be 0."
            );
            debug_assert_eq!(
                1,
                self.dir_idx(Direction::BWD),
                "Direction-Idx of BWD is expected to be 1."
            );
            [query.graph.fwd_edges(), query.graph.bwd_edges()]
        };
        self.init_query(nodes.count());
        let mut best_meeting: Option<(NodeIdx, f64)> = None;

        //----------------------------------------------------------------------------------------//
        // prepare first iteration(s)

        // push src-node
        self.queue.push(Reverse(CostNode {
            idx: query.src_idx,
            cost: 0.0,
            direction: Direction::FWD,
        }));
        // push dst-node
        self.queue.push(Reverse(CostNode {
            idx: query.dst_idx,
            cost: 0.0,
            direction: Direction::BWD,
        }));
        // update fwd-stats
        self.costs[self.fwd_idx()][*query.src_idx] = 0.0;
        // update bwd-stats
        self.costs[self.bwd_idx()][*query.dst_idx] = 0.0;

        //----------------------------------------------------------------------------------------//
        // search for shortest path

        while let Some(Reverse(current)) = self.queue.pop() {
            // For non-contracted graphs, this could be an slight improvement.
            // For contracted graphs, this is the only stop-criterion.
            // This is needed, because the bidirectional Dijkstra processes sub-graphs,
            // which are not equal.
            // This leads to the possibility, that shortest-paths of a sub-graph could be
            // non-optimal for the total graph, even if both sub-queries (forward and backward) have
            // already found a common meeting-node.
            //
            // Paths in sub-graphs have only one direction wrt node-level, namely up for fwd-graph
            // and down for bwd-graph.
            // This leads to weight-inbalanced queries, leading to solutions, which are optimal only
            // for the sub-graphs, not for the whole graph.
            if self.has_found_best_meeting_node() {
                break;
            }

            // distinguish between fwd and bwd
            let dir = self.dir_idx(current.direction);

            // First occurrence has improved, because init-value is infinity.
            // -> Replaces check if current CostNode has already been visited.
            if !self.has_costnode_improved(&current) {
                continue;
            }
            // otherwise, mark CostNode as visitted
            self.visit(&current);

            // if meeting-node is already found
            // -> check if new meeting-node is better
            if let Some((_meeting_node, best_total_cost)) = best_meeting {
                // if cost of single-queue is more expensive than best meeting-node
                // -> This can't be improved anymore
                if current.cost > best_total_cost {
                    self.has_found_best_meeting_node[dir] = true;
                    continue;
                }

                let new_total_cost = self.total_cost(&current);
                if new_total_cost < best_total_cost {
                    best_meeting = Some((current.idx, new_total_cost));
                }
            } else
            // if meeting-node is found for the first time, remember it
            if self.is_meeting_costnode(&current) {
                let new_total_cost = self.total_cost(&current);
                best_meeting = Some((current.idx, new_total_cost));
            }

            // update costs and add predecessors of nodes, which are dst of current's leaving edges
            for leaving_edge in xwd_edges[dir].starting_from(current.idx) {
                if self.is_ch_dijkstra
                    && nodes.level(current.idx) > nodes.level(leaving_edge.dst_idx())
                {
                    if !IS_USING_CH_LEVEL_SPEEDUP {
                        continue;
                    } else {
                        // break because leaving-edges are sorted by level
                        break;
                    }
                }

                let new_cost = current.cost
                    + helpers::dot_product(&query.routing_cfg.alphas, &leaving_edge.metrics());
                if new_cost < self.costs[dir][*leaving_edge.dst_idx()] {
                    self.predecessors[dir][*leaving_edge.dst_idx()] = Some(leaving_edge.idx());
                    self.costs[dir][*leaving_edge.dst_idx()] = new_cost;

                    // if path is found
                    // -> Run until queue is empty
                    //    since the shortest path could have longer hop-distance
                    //    with shorter weight-distance than currently found node.
                    // -> Only for bidirectional Dijkstra, but not incorrect for CH-Dijkstra.
                    //    The CH-Dijkstra has to continue until the global best meeting-node has
                    //    been found (see above).
                    if self.is_ch_dijkstra || best_meeting.is_none() {
                        self.queue.push(Reverse(CostNode {
                            idx: leaving_edge.dst_idx(),
                            cost: new_cost,
                            direction: current.direction,
                        }));
                    }
                }
            }
        }

        //----------------------------------------------------------------------------------------//
        // create path if found

        if let Some((meeting_node_idx, _best_total_cost)) = best_meeting {
            let mut proto_path = Vec::new();

            // iterate backwards over fwd-path
            let mut cur_idx = meeting_node_idx;
            let dir = self.fwd_idx();
            let opp_dir = self.bwd_idx();
            while let Some(incoming_idx) = self.predecessors[dir][*cur_idx] {
                proto_path.push(incoming_idx);

                // get incoming edge, but reversed to get the forward's src-node
                cur_idx = xwd_edges[opp_dir].dst_idx(incoming_idx);
            }

            // take fwd-part in the right order
            proto_path.reverse();

            // iterate backwards over bwd-path
            let mut cur_idx = meeting_node_idx;
            let dir = self.bwd_idx();
            let opp_dir = self.fwd_idx();
            while let Some(leaving_idx) = self.predecessors[dir][*cur_idx] {
                proto_path.push(leaving_idx);

                // get leaving edge, but reversed to get the backward's src-node
                cur_idx = xwd_edges[opp_dir].dst_idx(leaving_idx);
            }

            Some(Path::new(
                query.src_idx,
                nodes.id(query.src_idx),
                query.dst_idx,
                nodes.id(query.dst_idx),
                proto_path,
            ))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    FWD,
    BWD,
}

#[derive(Clone)]
struct CostNode {
    idx: NodeIdx,
    cost: f64,
    direction: Direction,
}

mod costnode {
    use super::{CostNode, Direction};
    use crate::approximating::Approx;
    use std::{
        cmp::Ordering,
        fmt::{self, Display},
    };

    impl Display for CostNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{{ idx: {}, cost: {}, {} }}",
                self.idx, self.cost, self.direction
            )
        }
    }

    impl Ord for CostNode {
        fn cmp(&self, other: &CostNode) -> Ordering {
            Approx(self.cost)
                .cmp(&Approx(other.cost))
                .then_with(|| self.idx.cmp(&other.idx))
                .then_with(|| self.direction.cmp(&other.direction))
        }
    }

    impl PartialOrd for CostNode {
        fn partial_cmp(&self, other: &CostNode) -> Option<Ordering> {
            Some(
                Approx(self.cost)
                    .partial_cmp(&Approx(other.cost))?
                    .then_with(|| self.idx.cmp(&other.idx))
                    .then_with(|| self.direction.cmp(&other.direction)),
            )
        }
    }

    impl Eq for CostNode {}

    impl PartialEq for CostNode {
        fn eq(&self, other: &CostNode) -> bool {
            self.idx == other.idx
                && self.direction == other.direction
                && Approx(self.cost) == Approx(other.cost)
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
