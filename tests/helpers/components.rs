use osmgraphing::{
    defaults::{self, capacity::DimVec},
    helpers::ApproxEq,
    network::{EdgeIdx, Graph, MetricIdx, Node, NodeIdx},
    routing::{self},
    units::{geo::Coordinate, speed::KilometersPerHour},
};
use smallvec::SmallVec;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq)]
pub struct TestNode {
    pub name: String,
    pub id: i64,
    pub idx: NodeIdx,
    pub coord: Coordinate,
    pub level: usize,
}

impl From<Node> for TestNode {
    fn from(node: Node) -> TestNode {
        TestNode {
            name: "node-from-graph".to_owned(),
            id: node.id(),
            idx: node.idx(),
            coord: node.coord(),
            level: node.level(),
        }
    }
}

impl Display for TestNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (idx={}, id={})", self.name, self.idx, self.id)
    }
}

impl PartialEq for TestNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.idx == other.idx
            && self.coord == other.coord
            && self.level == other.level
    }
}

impl TestNode {
    #[allow(dead_code)]
    pub fn new(name: &str, id: i64, coord: Coordinate, level: usize, graph: &Graph) -> TestNode {
        let idx = graph
            .nodes()
            .idx_from(id)
            .expect(&format!("The node-id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            coord,
            level,
        }
    }
}

#[allow(dead_code)]
pub struct TestEdge {
    name: String,
    edge_idx: EdgeIdx,
    is_fwd: bool,
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    metrics: Vec<f64>,
}

impl TestEdge {
    #[allow(dead_code)]
    pub fn new_fwd<L, T>(
        name: Option<&str>,
        edge_idx: EdgeIdx,
        src: &TestNode,
        dst: &TestNode,
        length: L,
        maxspeed: KilometersPerHour,
        duration: T,
    ) -> TestEdge
    where
        L: Into<defaults::length::TYPE>,
        T: Into<defaults::time::TYPE>,
    {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            edge_idx: edge_idx,
            is_fwd: true,
            src_idx: src.idx.into(),
            dst_idx: dst.idx.into(),
            metrics: vec![*length.into(), *maxspeed, *duration.into()],
        }
    }

    #[allow(dead_code)]
    pub fn new_bwd<L, T>(
        name: Option<&str>,
        edge_idx: EdgeIdx,
        src: &TestNode,
        dst: &TestNode,
        length: L,
        maxspeed: KilometersPerHour,
        duration: T,
    ) -> TestEdge
    where
        L: Into<defaults::length::TYPE>,
        T: Into<defaults::time::TYPE>,
    {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            edge_idx: edge_idx,
            is_fwd: false,
            src_idx: src.idx.into(),
            dst_idx: dst.idx.into(),
            metrics: vec![*length.into(), *maxspeed, *duration.into()],
        }
    }

    #[allow(dead_code)]
    pub fn assert_correct(&self, graph: &Graph) {
        // get graph-components dependent on own direction
        let fwd_edges = graph.fwd_edges();
        let bwd_edges = graph.bwd_edges();
        let (edge, edge_idx) = {
            if self.is_fwd {
                fwd_edges
                    .between(self.src_idx, self.dst_idx)
                    .expect(&format!(
                        "Fwd-edge (src_idx, dst_idx): ({}, {}) does not exist.",
                        self.src_idx, self.dst_idx
                    ))
            } else {
                bwd_edges
                    .between(self.src_idx, self.dst_idx)
                    .expect(&format!(
                        "Bwd-edge (src_idx, dst_idx): ({}, {}) does not exist.",
                        self.src_idx, self.dst_idx
                    ))
            }
        };
        let prefix = {
            if self.is_fwd {
                "fwd-"
            } else {
                "bwd-"
            }
        };

        assert_eq!(
            edge_idx, self.edge_idx,
            "Wrong {}edge-idx={} for {}",
            prefix, edge_idx, self.name
        );
        assert_eq!(
            edge.dst_idx(),
            self.dst_idx,
            "Wrong dst_idx={} for {}edge {}",
            edge.dst_idx(),
            prefix,
            self.name
        );

        let metric_indices = &[MetricIdx(0), MetricIdx(1), MetricIdx(2)];
        let value = edge.metrics(metric_indices);
        let expected = SmallVec::from_slice(&self.metrics);
        assert!(
            value.approx_eq(&expected),
            "Wrong metrics {:?} for {}edge {}. Expected: {:?}",
            value,
            prefix,
            self.name,
            expected
        );
    }
}

pub struct TestPath {
    src: TestNode,
    dst: TestNode,
    cost: DimVec<f64>,
    metric_indices: DimVec<MetricIdx>,
    alternative_nodes: Vec<Vec<TestNode>>,
}

impl TestPath {
    pub fn from_alternatives(
        src: TestNode,
        dst: TestNode,
        cost: DimVec<f64>,
        metric_indices: DimVec<MetricIdx>,
        alternative_nodes: Vec<Vec<TestNode>>,
    ) -> TestPath {
        TestPath {
            src,
            dst,
            cost,
            metric_indices,
            alternative_nodes,
        }
    }

    pub fn assert_correct(&self, actual_path: &routing::paths::Path, graph: &Graph) {
        let node = |idx: NodeIdx| -> TestNode { TestNode::from(graph.nodes().create(idx)) };

        let path_src = node(actual_path.src_idx());
        assert_eq!(
            &path_src.idx, &self.src.idx,
            "Path has wrong src-idx {} (should be {})",
            &path_src.idx, &self.src.idx,
        );
        let path_dst = node(actual_path.dst_idx());
        assert_eq!(
            &path_dst.idx, &self.dst.idx,
            "Path has wrong dst-idx {} (should be {})",
            &path_dst.idx, &self.dst.idx,
        );

        // flatten shortcuts
        let flattened_actual_path = actual_path.clone().flatten(graph);

        let mut is_path_eq = false;
        let mut wrong_path_result = None;
        let mut wrong_cost_result = None;
        let mut is_cost_eq = false;
        for nodes in &self.alternative_nodes {
            // build path from own path
            let mut own_proto_path = Vec::new();

            // build own path

            let fwd_edges = graph.fwd_edges();
            let mut iter = nodes.windows(2);
            while let Some([test_src, test_dst]) = iter.next() {
                own_proto_path.push(
                    fwd_edges
                        .between(test_src.idx, test_dst.idx)
                        .expect(&format!(
                            "Edge expected between idx={} and idx={}. Path is from idx={} to idx={}",
                            test_src.idx, test_dst.idx, path_src.idx, path_dst.idx
                        ))
                        .1,
                );
            }

            // check path

            let expected_path =
                routing::paths::Path::new(self.src.idx, self.dst.idx, own_proto_path);
            if expected_path != flattened_actual_path {
                wrong_path_result = Some((expected_path, &flattened_actual_path));
                continue;
            } else {
                is_path_eq = true;
            }

            // check path-cost

            let (expected_cost, actual_cost) = (
                &self.cost,
                flattened_actual_path.calc_cost(&self.metric_indices, graph),
            );
            if !expected_cost.approx_eq(&actual_cost) {
                wrong_cost_result = Some((expected_cost, actual_cost));
                continue;
            } else {
                is_cost_eq = true;
            }
        }

        // check if one correct alternative has been equal
        // if not, print error
        // ATTENTION: order is important since path is set above before cost

        if !is_path_eq {
            let (expected_path, flattened_actual_path) =
                wrong_path_result.expect("Fix testing path: Bool is set wrongly.");
            panic!(
                "Graph: {}; Path from src {} to dst {} is not equal. (expected: {}, actual: {})",
                graph, self.src, self.dst, expected_path, flattened_actual_path
            );
        }

        if !is_cost_eq {
            let (expected_cost, actual_cost) =
                wrong_cost_result.expect("Fix testing path-cost: Bool is set wrongly.");
            panic!(
                "Path-cost {:?} from src {} to dst {} is not correct (expected: {:?}).",
                actual_cost, self.src, self.dst, expected_cost
            );
        }
    }
}
