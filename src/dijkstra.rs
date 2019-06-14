use super::graph;
use std:: {
    cmp::Ordering,
    collections::BinaryHeap,
};

pub struct Dijkstra<'a> {
    pub graph: &'a graph::Graph,
    pub cost: Vec<usize>,
    pub path: Vec<usize>
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CostNode {
    pub cost: usize,
    pub id: usize,
}

impl Ord for CostNode {
    fn cmp(&self, other: &CostNode) -> Ordering {
        //flips the ordering
        other.cost.cmp(&self.cost)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for CostNode {
    fn partial_cmp(&self, other: &CostNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait ShortestPath {
    fn compute_shortestPath(&mut self, source: usize, dest: usize);
    fn get_distance(&mut self, node_id: usize) -> usize;
    fn get_Path(&mut self, source: usize, dest: usize) -> Vec<usize>;
}

impl<'a>  ShortestPath for Dijkstra<'a>  {
    fn compute_shortestPath(&mut self, source: usize, dest: usize) {
        self.cost[source] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(CostNode {cost: 0, id: source});
        while let Some(CostNode {cost, id} ) = queue.pop() {
            if id == dest {
                break;
            } 
            if cost > self.cost[id] {
                continue;
            }
            let graph_node = &self.graph.nodes[id];
            for i in graph_node.edge_start .. graph_node.edge_end + 1 {
                let current_edge = &self.graph.edges[i];
                let current_cost = cost + current_edge.weight;
                if current_cost < self.cost[current_edge.dest] {
                    self.path[current_edge.dest] = i;
                    self.cost[current_edge.dest] = current_cost;
                    queue.push(CostNode {cost: current_cost, id: current_edge.dest});
                }
            }
        }
    }

    fn get_distance(&mut self, node_id: usize) -> usize{
        if node_id >= self.graph.node_count {
            let result = std::usize::MAX;
            result
        } else {
            self.cost[node_id]
        }
    }

    fn get_Path(&mut self, source: usize, dest: usize) -> std::vec::Vec<usize> {
        if source >= self.graph.node_count || dest >= self.graph.node_count {
            let result = vec![];
            result
        } else {
            let mut shortest_path = Vec::new();
            let mut current_predec = dest;
            while current_predec != source {
                let current_edge = &self.graph.edges[self.path[current_predec]];
                shortest_path.push(current_edge.id);
                current_predec = current_edge.src;
            }
        shortest_path
        }
    }
}

pub fn init_dijkstra(graph: &graph::Graph) -> Dijkstra{
    let cost = vec![std::usize::MAX; graph.node_count];
    let path = vec![std::usize::MAX; graph.node_count];
    let dijkstra = Dijkstra {
        graph : graph,
        cost : cost,
        path : path
    };
    dijkstra
}

