//------------------------------------------------------------------------------------------------//
// other modules

use std::fmt;

use osmgraphing::network::Graph;
use osmgraphing::routing;

//------------------------------------------------------------------------------------------------//
// own modules

mod astar;

//------------------------------------------------------------------------------------------------//
// helpers

fn assert_correct(
    astar: &mut Box<dyn routing::Astar>,
    expected_paths: Vec<(TestNode, TestNode, Option<(u32, Vec<Vec<TestNode>>)>)>,
    filepath: &str,
) {
    let graph = super::parse(filepath);

    for (src, dst, option_specs) in expected_paths {
        let graph_src = graph
            .node(src.idx)
            .expect(&format!("src-node of idx={} should be in graph.", src.idx));
        let graph_dst = graph
            .node(dst.idx)
            .expect(&format!("dst-node of idx={} should be in graph.", dst.idx));
        let option_path = astar.compute_best_path(graph_src, graph_dst, &graph);
        assert_eq!(
            option_path.is_some(),
            option_specs.is_some(),
            "Path from {} to {} should be {}",
            src,
            dst,
            if option_specs.is_some() {
                "Some"
            } else {
                "None"
            }
        );

        if let (Some((cost, nodes)), Some(path)) = (option_specs, option_path) {
            TestPath::from_alternatives(src, dst, cost, nodes).assert_correct(&path, &graph);
        }
    }
}

//------------------------------------------------------------------------------------------------//
// test-node

#[derive(Debug, Copy, Clone)]
struct TestNode {
    idx: usize,
    id: i64,
}
impl TestNode {
    pub fn from(idx: usize, id: i64) -> TestNode {
        TestNode { idx, id }
    }
}
impl Eq for TestNode {}
impl PartialEq for TestNode {
    fn eq(&self, other: &TestNode) -> bool {
        self.idx.eq(&other.idx) && self.id.eq(&other.id)
    }
}
impl fmt::Display for TestNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(idx: {}, id: {})", self.idx, self.id)
    }
}

//------------------------------------------------------------------------------------------------//
// test-path

struct TestPath {
    src: TestNode,
    dst: TestNode,
    cost: u32,
    alternative_nodes: Vec<Vec<TestNode>>,
}
impl TestPath {
    fn from_alternatives(
        src: TestNode,
        dst: TestNode,
        cost: u32,
        alternative_nodes: Vec<Vec<TestNode>>,
    ) -> TestPath {
        TestPath {
            src,
            dst,
            cost,
            alternative_nodes,
        }
    }

    fn assert_correct(&self, path: &routing::astar::Path, graph: &Graph) {
        let node = |idx: usize| -> TestNode {
            TestNode::from(
                idx,
                graph.node(idx).expect("Node should be in graph here.").id(),
            )
        };

        //----------------------------------------------------------------------------------------//
        // check meta-info

        let path_src = node(path.src_idx());
        assert_eq!(
            path_src, self.src,
            "Path has wrong src {} (should be {})",
            path_src, self.src,
        );
        let path_dst = node(path.dst_idx());
        assert_eq!(
            path_dst, self.dst,
            "Path has wrong dst {} (should be {})",
            path_dst, self.dst,
        );
        assert_eq!(
            path.cost(),
            self.cost,
            "Path from src {} to dst {} should have cost {}",
            self.src,
            self.dst,
            self.cost,
        );

        //----------------------------------------------------------------------------------------//
        // check predecessors/successors

        // src has no predecessor
        assert_eq!(
            path.pred_node_idx(self.src.idx),
            None,
            "Predecessor of src {} should be None",
            self.src
        );
        // dst has no successor
        assert_eq!(
            path.succ_node_idx(self.dst.idx),
            None,
            "Predecessor of dst {} should be None",
            self.dst
        );

        let mut is_pred_eq = false;
        let mut is_succ_eq = false;
        for nodes in &self.alternative_nodes {
            if nodes.len() > 0 {
                // build predecessor-path
                let mut current = path_dst;
                let mut pred_path = vec![current];
                while let Some(pred) = path.pred_node_idx(current.idx) {
                    let pred = node(pred);
                    pred_path.push(pred);
                    current = pred;
                }
                pred_path.reverse();
                is_pred_eq |= &pred_path == nodes;

                // build successor-path
                let mut current = path_src;
                let mut succ_path = vec![current];
                while let Some(succ) = path.succ_node_idx(current.idx) {
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
            "Predecessor-path from src {} to dst {} is wrong.",
            self.src, self.dst
        );
        assert!(
            is_succ_eq,
            "Successor-path from src {} to dst {} is wrong.",
            self.src, self.dst
        );
    }
}
