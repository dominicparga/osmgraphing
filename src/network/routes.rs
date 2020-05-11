use crate::network::{Graph, Node, NodeIdx};

#[derive(Copy, Clone)]
pub struct RoutePair<T> {
    pub src: T,
    pub dst: T,
}

// impl<T> Into<(T, T)> for RoutePair<T> {
//     fn into(self) -> (T, T) {
//         (self.src, self.dst)
//     }
// }

impl RoutePair<i64> {
    pub fn into_idx(self, graph: &Graph) -> RoutePair<NodeIdx> {
        let nodes = graph.nodes();
        RoutePair {
            src: nodes.idx_from(self.src).expect(&format!(
                "RoutePair<i64> contains src-id {}, which is not part of the graph.",
                self.src
            )),
            dst: nodes.idx_from(self.dst).expect(&format!(
                "RoutePair<i64> contains dst-id {}, which is not part of the graph.",
                self.dst
            )),
        }
    }

    pub fn into_node(self, graph: &Graph) -> RoutePair<Node> {
        let nodes = graph.nodes();
        RoutePair {
            src: nodes.create_from(self.src).expect(&format!(
                "RoutePair<i64> contains src-id {}, which is not part of the graph.",
                self.src
            )),
            dst: nodes.create_from(self.dst).expect(&format!(
                "RoutePair<i64> contains dst-id {}, which is not part of the graph.",
                self.dst
            )),
        }
    }
}

impl RoutePair<NodeIdx> {
    pub fn into_node(self, graph: &Graph) -> RoutePair<Node> {
        let nodes = graph.nodes();
        RoutePair {
            src: nodes.create(self.src),
            dst: nodes.create(self.dst),
        }
    }
}
