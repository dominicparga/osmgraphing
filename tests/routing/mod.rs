use super::{create_config, TestType};
use osmgraphing::{configs::graph, network::Graph, network::NodeIdx, routing, units::Metric};
use std::{fmt, fmt::Display};

mod fastest;
mod shortest;

fn assert_correct<M>(
    astar: &mut Box<dyn routing::astar::Astar<M>>,
    expected_paths: Vec<(TestNode, TestNode, Option<(M, Vec<Vec<TestNode>>)>)>,
    cfg: &graph::Config,
) where
    M: Metric + PartialEq + Display,
{
    let graph = super::parse(cfg);

    for (src, dst, option_specs) in expected_paths {
        let nodes = graph.nodes();
        let graph_src = nodes.create(src.idx);
        let graph_dst = nodes.create(dst.idx);
        let option_path = astar.compute_best_path(&graph_src, &graph_dst, &graph);
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
            TestPath::<M>::from_alternatives(src, dst, cost, nodes).assert_correct(&path, &graph);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TestNode {
    pub idx: NodeIdx,
    pub id: i64,
}

impl Display for TestNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(idx: {}, id: {})", self.idx, self.id)
    }
}

impl Eq for TestNode {}

impl PartialEq for TestNode {
    fn eq(&self, other: &TestNode) -> bool {
        self.idx.eq(&other.idx) && self.id.eq(&other.id)
    }
}

struct TestPath<M>
where
    M: Metric,
{
    src: TestNode,
    dst: TestNode,
    cost: M,
    alternative_nodes: Vec<Vec<TestNode>>,
}

impl<M> TestPath<M>
where
    M: Metric + PartialEq + Display,
{
    fn from_alternatives(
        src: TestNode,
        dst: TestNode,
        cost: M,
        alternative_nodes: Vec<Vec<TestNode>>,
    ) -> TestPath<M> {
        TestPath {
            src,
            dst,
            cost,
            alternative_nodes,
        }
    }

    fn assert_correct(&self, path: &routing::paths::Path<M>, graph: &Graph) {
        let node = |idx: NodeIdx| -> TestNode {
            TestNode {
                idx,
                id: graph.nodes().id(idx),
            }
        };

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
