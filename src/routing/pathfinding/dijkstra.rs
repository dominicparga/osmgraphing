use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::routing;
use routing::Edge;
use routing::Graph;

//--------------------------------------------------------------------------------------------------
// nodes

#[derive(Copy, Clone)]
struct CostNode {
    pub id: usize,
    pub cost: u64,
}

impl Ord for CostNode {
    fn cmp(&self, other: &CostNode) -> Ordering {
        // (1) cost in float, but cmp uses only m, which is ok
        // (2) inverse order since BinaryHeap is max-heap, but min-heap is needed
        let delta = (other.cost - self.cost) as i64;
        delta.cmp(&0).then_with(|| other.id.cmp(&self.id))
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

//--------------------------------------------------------------------------------------------------
// Dijkstra's type of path

#[derive(Clone)]
pub struct Path<'a> {
    pub cost: Vec<u64>,
    pub predecessors: Vec<Option<&'a Edge>>,
}

//--------------------------------------------------------------------------------------------------
// Dijkstra

pub struct Dijkstra<'a> {
    pub graph: &'a Graph,
    pub path: Path<'a>,
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Dijkstra {
        Dijkstra {
            graph,
            path: Path {
                cost: vec![std::u64::MAX; graph.node_count()],
                predecessors: vec![None; graph.node_count()],
            },
        }
    }
}

impl<'a> Dijkstra<'a> {
    pub fn compute_shortest_path(&mut self, src: usize, dst: usize) -> Cow<Path> {
        //------------------------------------------------------------------------------------------
        // initialize, but check path-"cache" before

        for i in 0..self.graph.node_count() {
            self.path.cost[i] = std::u64::MAX;
            self.path.predecessors[i] = None;
        }
        let mut queue = BinaryHeap::new(); // max-heap, but CostNode's natural order is reversed

        // prepare first iteration
        queue.push(CostNode { id: src, cost: 0 });
        self.path.cost[src] = 0;

        //------------------------------------------------------------------------------------------
        // compute

        while let Some(CostNode { id, cost }) = queue.pop() {
            // shortest path found
            if id == dst {
                break;
            }

            // if node has already been visited
            if cost > self.path.cost[id] {
                continue;
            }

            // if not -> update "official" cost
            // and add successors
            for edge in self.graph.leaving_edges(id) {
                let new_cost = cost + edge.weight();

                if new_cost < self.path.cost[edge.dst()] {
                    self.path.predecessors[edge.dst()] = Some(&edge);
                    self.path.cost[edge.dst()] = new_cost;
                    queue.push(CostNode {
                        id: edge.dst(),
                        cost: new_cost,
                    });
                }
            }
        }

        Cow::Borrowed(&self.path)
    }

    // TODO
    // pub fn get_path(&mut self, src: usize, dst: usize) -> std::vec::Vec<usize> {
    //     if src >= self.graph.node_count() || dst >= self.graph.node_count() {
    //         let result = vec![];
    //         result
    //     } else {
    //         let mut shortest_path = Vec::new();
    //         let mut current_predec = dst;
    //         while current_predec != src {
    //             let edge = self.graph.edge(self.path.predecessors[current_predec]);
    //             shortest_path.push(edge.id);
    //             current_predec = edge.src;
    //         }
    //         shortest_path
    //     }
    // }
}
