// Dead code allowed, because it is actually used in test-modules, but compiler doesn't recognize.
// March 6th, 2020

use osmgraphing::{
    configs::{self as configs, Config},
    defaults::DimVec,
    helpers::{ApproxEq, MapFileExt},
    network::{EdgeIdx, Graph, MetricIdx, Node, NodeAccessor, NodeIdx},
    routing::{self},
    units::geo::Coordinate,
    Parser,
};
use smallvec::{smallvec, SmallVec};
use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[allow(dead_code)]
pub mod defaults {
    pub const LENGTH_ID: &str = "Meters";
    pub const DURATION_ID: &str = "Seconds";
}

#[allow(dead_code)]
pub enum TestType {
    BidirectionalBait,
    IsleOfMan,
    SimpleStuttgart,
    Small,
}

pub fn create_config(test_type: TestType, routing_cfg: Option<&str>) -> Config {
    // cfg.graph
    let map_file = match test_type {
        TestType::BidirectionalBait => "resources/maps/bidirectional-bait.fmi",
        TestType::IsleOfMan => "resources/maps/isle-of-man_2019-09-05.osm.pbf",
        TestType::SimpleStuttgart => "resources/maps/simple-stuttgart.fmi",
        TestType::Small => "resources/maps/small.fmi",
    };
    let mut cfg = match MapFileExt::from_path(map_file).expect("Map-file should exist.") {
        MapFileExt::PBF => Config::from_yaml("resources/configs/isle-of-man.pbf.yaml"),
        MapFileExt::FMI => Config::from_yaml("resources/configs/simple-stuttgart.fmi.yaml"),
    }
    .expect("Config is tested separatedly.");
    cfg.graph.map_file = PathBuf::from(map_file);

    // cfg.routing
    if let Some(yaml_str) = routing_cfg {
        cfg.routing = Some(
            configs::routing::Config::from_str(yaml_str, &cfg.graph)
                .expect("Config is tested separatedly"),
        );
    }

    // return
    cfg
}

#[allow(dead_code)]
pub fn parse(cfg: configs::graph::Config) -> Graph {
    let map_file = cfg.map_file.clone();
    match Parser::parse_and_finalize(cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!("Could not parse {}. ERROR: {}", map_file.display(), msg);
        }
    }
}

#[allow(dead_code)]
pub fn assert_nodes(test_nodes: &Vec<TestNode>, nodes: &NodeAccessor) {
    for (expected, original) in test_nodes
        .iter()
        .map(|expected| (expected, TestNode::from(nodes.create(expected.idx))))
    {
        assert_eq!(
            expected, &original,
            "Expected node {} but graph-node is {}.",
            expected, original
        );
    }
}

#[allow(dead_code)]
pub fn assert_path(
    dijkstra: &mut routing::Dijkstra,
    expected_paths: Vec<(TestNode, TestNode, Option<(f32, Vec<Vec<TestNode>>)>)>,
    cfg: Config,
) {
    let graph = parse(cfg.graph);
    for (src, dst, option_specs) in expected_paths {
        let nodes = graph.nodes();
        let graph_src = nodes.create(src.idx);
        let graph_dst = nodes.create(dst.idx);
        let option_path = dijkstra.compute_best_path(
            &graph_src,
            &graph_dst,
            &graph,
            &cfg.routing
                .as_ref()
                .expect("Routing-config should be existent"),
        );
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
            TestPath::from_alternatives(src, dst, smallvec![cost], nodes)
                .assert_correct(&path, &graph);
        }
    }
}

#[derive(Clone, Debug, Eq)]
pub struct TestNode {
    pub name: String,
    pub id: i64,
    pub idx: NodeIdx,
    pub coord: Coordinate,
}

impl From<Node> for TestNode {
    fn from(node: Node) -> TestNode {
        TestNode {
            name: "node-from-graph".to_owned(),
            id: node.id(),
            idx: node.idx(),
            coord: node.coord(),
        }
    }
}

impl Display for TestNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, idx: {}, coord: {:?} }}",
            self.id, self.idx, self.coord,
        )
    }
}

impl PartialEq for TestNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.idx == other.idx && self.coord == other.coord
    }
}

impl TestNode {
    #[allow(dead_code)]
    pub fn new(name: &str, id: i64, lat: f32, lon: f32, graph: &Graph) -> TestNode {
        let idx = graph
            .nodes()
            .idx_from(id)
            .expect(&format!("The node-id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            coord: Coordinate { lat, lon },
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
    metrics: Vec<f32>,
}

impl TestEdge {
    #[allow(dead_code)]
    pub fn new_fwd(
        name: Option<&str>,
        edge_idx: usize,
        src: &TestNode,
        dst: &TestNode,
        length: f32,
        maxspeed: f32,
        duration: f32,
    ) -> TestEdge {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            edge_idx: EdgeIdx(edge_idx),
            is_fwd: true,
            src_idx: src.idx.into(),
            dst_idx: dst.idx.into(),
            metrics: vec![length, maxspeed, duration],
        }
    }

    #[allow(dead_code)]
    pub fn new_bwd(
        name: Option<&str>,
        edge_idx: usize,
        src: &TestNode,
        dst: &TestNode,
        length: f32,
        maxspeed: f32,
        duration: f32,
    ) -> TestEdge {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            edge_idx: EdgeIdx(edge_idx),
            is_fwd: false,
            src_idx: src.idx.into(),
            dst_idx: dst.idx.into(),
            metrics: vec![length, maxspeed, duration],
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

        let metric_indices = smallvec![MetricIdx(0), MetricIdx(1), MetricIdx(2)];
        let value = edge.metric(&metric_indices);
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
    cost: DimVec<f32>,
    alternative_nodes: Vec<Vec<TestNode>>,
}

impl TestPath {
    pub fn from_alternatives(
        src: TestNode,
        dst: TestNode,
        cost: DimVec<f32>,
        alternative_nodes: Vec<Vec<TestNode>>,
    ) -> TestPath {
        TestPath {
            src,
            dst,
            cost,
            alternative_nodes,
        }
    }

    pub fn assert_correct(&self, path: &routing::paths::Path<DimVec<f32>>, graph: &Graph) {
        let node = |idx: NodeIdx| -> TestNode { TestNode::from(graph.nodes().create(idx)) };

        let path_src = node(path.src_idx());
        assert_eq!(
            &path_src, &self.src,
            "Path has wrong src {} (should be {})",
            &path_src, &self.src,
        );
        let path_dst = node(path.dst_idx());
        assert_eq!(
            &path_dst, &self.dst,
            "Path has wrong dst {} (should be {})",
            &path_dst, &self.dst,
        );
        assert!(
            path.cost().approx_eq(&self.cost),
            "Path from src {} to dst {} should have cost {:?}, but has {:?}.",
            self.src,
            self.dst,
            self.cost,
            path.cost() // path.cost().approx_eq(&self.cost),
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
                let mut current = path_dst.clone();
                let mut pred_path = vec![current.clone()];
                while let Some(pred) = path.pred_node_idx(current.idx) {
                    let pred = node(pred);
                    pred_path.push(pred.clone());
                    current = pred;
                }
                pred_path.reverse();
                is_pred_eq |= &pred_path == nodes;

                // build successor-path
                let mut current = path_src.clone();
                let mut succ_path = vec![current.clone()];
                while let Some(succ) = path.succ_node_idx(current.idx) {
                    let succ = node(succ);
                    succ_path.push(succ.clone());
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
