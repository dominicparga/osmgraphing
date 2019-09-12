use super::parse;
mod dijkstra;

//--------------------------------------------------------------------------------------------//

use osmgraphing::network::Graph;
use osmgraphing::routing;

//--------------------------------------------------------------------------------------------//
// helpers

fn idx(node: (usize, i64)) -> usize {
    node.0
}

fn id(node: (usize, i64)) -> i64 {
    node.1
}

//--------------------------------------------------------------------------------------------//
// test-path

struct TestPath {
    src: (usize, i64),
    dst: (usize, i64),
    cost: u32,
    alternative_nodes: Vec<Vec<(usize, i64)>>,
}
impl TestPath {
    fn from(src: (usize, i64), dst: (usize, i64), cost: u32, nodes: Vec<(usize, i64)>) -> TestPath {
        TestPath {
            src,
            dst,
            cost,
            alternative_nodes: vec![nodes],
        }
    }

    fn from_alternatives(
        src: (usize, i64),
        dst: (usize, i64),
        cost: u32,
        alternative_nodes: Vec<Vec<(usize, i64)>>,
    ) -> TestPath {
        TestPath {
            src,
            dst,
            cost,
            alternative_nodes,
        }
    }

    fn assert(&self, path: &routing::dijkstra::Path, graph: &Graph) {
        let node = |idx: usize| -> (usize, i64) { (idx, graph.node(idx).id()) };

        //----------------------------------------------------------------------------------------//
        // check meta-info

        let path_src = node(path.src_idx());
        assert_eq!(
            path_src, self.src,
            "Path has wrong src (idx, id)={:?} (should be {:?})",
            path_src, self.src,
        );
        let path_dst = node(path.dst_idx());
        assert_eq!(
            path_dst, self.dst,
            "Path has wrong dst (idx, id)={:?} (should be {:?})",
            path_dst, self.dst,
        );
        assert_eq!(
            path.cost(),
            self.cost,
            "Path from src-id {} to dst-id {} should have cost={}",
            id(self.src),
            id(self.dst),
            self.cost,
        );

        //----------------------------------------------------------------------------------------//
        // check predecessors/successors

        // src has no predecessor
        assert_eq!(
            path.predecessor(idx(self.src)),
            None,
            "Predecessor of src-idx {} should be None",
            idx(self.src)
        );
        // dst has no successor
        assert_eq!(
            path.successor(idx(self.dst)),
            None,
            "Predecessor of dst-idx {} should be None",
            idx(self.dst)
        );

        let mut is_pred_eq = false;
        let mut is_succ_eq = false;
        for nodes in &self.alternative_nodes {
            if nodes.len() > 0 {
                // predecessor-path
                let mut current = path_dst;
                let mut pred_path = vec![current];
                while let Some(pred) = path.predecessor(idx(current)) {
                    let pred = node(pred);
                    pred_path.push(pred);
                    current = pred;
                }
                pred_path.reverse();
                is_pred_eq |= &pred_path == nodes;

                // successor-path
                let mut current = path_src;
                let mut succ_path = vec![current];
                while let Some(succ) = path.successor(idx(current)) {
                    let succ = node(succ);
                    succ_path.push(succ);
                    current = succ;
                }
                is_succ_eq |= &succ_path == nodes;
            } else {
                is_pred_eq = true;
                is_succ_eq = true;
            }
        }
        assert!(
            is_pred_eq,
            "Predecessor-path from src={:?} to dst={:?} is wrong.",
            self.src, self.dst
        );
        assert!(
            is_succ_eq,
            "Successor-path from src={:?} to dst={:?} is wrong.",
            self.src, self.dst
        );
    }
}
