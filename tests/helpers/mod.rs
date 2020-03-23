// Dead code allowed, because it is actually used in test-modules, but compiler doesn't recognize.
// March 6th, 2020

use osmgraphing::{
    configs::{self, Config},
    defaults::capacity::DimVec,
    helpers::{self, ApproxEq},
    io::Parser,
    network::{EdgeIdx, Graph, MetricIdx, Node, NodeAccessor, NodeIdx},
    routing::{self},
    units::geo::Coordinate,
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use smallvec::SmallVec;
use std::fmt::{self, Display};

#[allow(dead_code)]
pub mod defaults {
    pub const LENGTH_ID: &str = "Meters";
    pub const DURATION_ID: &str = "Seconds";

    pub mod paths {
        pub mod resources {
            pub mod configs {
                pub const SIMPLE_STUTTGART_FMI: &str =
                    "resources/configs/simple-stuttgart.fmi.yaml";
                pub const SMALL_FMI: &str = "resources/configs/small.fmi.yaml";
                pub const SMALL_CH_FMI: &str = "resources/configs/small.ch.fmi.yaml";
                pub const BIDIRECTIONAL_BAIT_FMI: &str =
                    "resources/configs/bidirectional-bait.fmi.yaml";
                pub const ISLE_OF_MAN_FMI: &str = "resources/configs/isle-of-man.fmi.yaml";
                pub const ISLE_OF_MAN_CH_FMI: &str = "resources/configs/isle-of-man.ch.fmi.yaml";
                pub const ISLE_OF_MAN_PBF: &str = "resources/configs/isle-of-man.pbf.yaml";
            }
        }
    }
}

pub fn parse(cfg: configs::parser::Config) -> Graph {
    let map_file = cfg.map_file.clone();
    match Parser::parse_and_finalize(cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!("Could not parse {}. ERROR: {}", map_file.display(), msg);
        }
    }
}

#[allow(dead_code)]
pub fn test_dijkstra(
    config_file: &str,
    metric_id: &str,
    is_ch_dijkstra: bool,
    expected_paths: Box<
        dyn Fn(
            &configs::parser::Config,
        ) -> Vec<(
            TestNode,
            TestNode,
            DimVec<MetricIdx>,
            Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
        )>,
    >,
) {
    let mut cfg = Config::from_yaml(config_file).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}], is-ch-dijkstra: {} }}",
            metric_id,
            if is_ch_dijkstra { "true" } else { "false" }
        ),
        &cfg.parser,
    )
    .ok();

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths(&cfg.parser);

    assert_path(&mut dijkstra, expected_paths, cfg);
}

#[allow(dead_code)]
pub fn compare_dijkstras(ch_fmi_config_file: &str, metric_id: &str) {
    // build configs
    let mut cfg = Config::from_yaml(ch_fmi_config_file).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!("routing: {{ metrics: [{{ id: '{}' }}] }}", metric_id),
        &cfg.parser,
    )
    .ok();
    let mut cfg_routing = cfg.routing.unwrap();
    cfg_routing.set_ch_dijkstra(false);
    let mut cfg_routing_ch = cfg_routing.clone();
    cfg_routing_ch.set_ch_dijkstra(true);

    // parse graph and init dijkstra
    let graph = Parser::parse_and_finalize(cfg.parser).unwrap();

    let nodes = graph.nodes();
    let mut dijkstra = routing::Dijkstra::new();

    // generate random route-pairs
    let route_count = 100;
    let seed = 42;

    // if all possible routes are less than the preferred route-count
    // -> just print all possible routes
    // else: print random routes
    let mut gen_route = {
        let mut rng = rand_pcg::Pcg32::seed_from_u64(seed);
        let die = Uniform::from(0..nodes.count());
        let mut i = 0;
        move || {
            if i < route_count {
                let src_idx = NodeIdx(die.sample(&mut rng));
                let dst_idx = NodeIdx(die.sample(&mut rng));
                i += 1;
                Some((src_idx, dst_idx))
            } else {
                None
            }
        }
    };

    while let Some((src_idx, dst_idx)) = gen_route() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);

        let option_ch_path = dijkstra.compute_best_path(&src, &dst, &graph, &cfg_routing_ch);
        let option_path = dijkstra.compute_best_path(&src, &dst, &graph, &cfg_routing);

        // check if both are none/not-none
        if option_ch_path.is_none() != option_path.is_none() {
            let (ch_err, err) = {
                if option_ch_path.is_none() {
                    ("None", "Some")
                } else {
                    ("Some", "None")
                }
            };
            panic!(
                "CH-Dijkstra's result is {}, while Dijkstra's result is {}. \
                 Route is from ({}) to ({}).",
                ch_err, err, src, dst
            );
        }

        // check basic info
        if let (Some(ch_path), Some(path)) = (option_ch_path, option_path) {
            let flattened_ch_path = ch_path.flatten(&graph);
            let flattened_path = path.flatten(&graph);

            // cmp cost
            let ch_cost = flattened_ch_path.calc_cost(cfg_routing.metric_indices(), &graph);
            let cost = flattened_path.calc_cost(cfg_routing.metric_indices(), &graph);
            assert!(ch_cost.approx_eq(&cost),
                "CH-Dijkstra's path's cost ({:?}) is different ({:?}) from Dijkstra's path's cost ({:?}). --------------------- CH-Dijkstra's path {} --------------------- Dijkstra's path {}",
                ch_cost, helpers::sub(&ch_cost, &cost), cost,
                flattened_ch_path,
                flattened_path
            );

            // cmp edges
            assert!(flattened_ch_path == flattened_path, "CH-Dijkstra's path  is different from Dijkstra's path. --------------------- CH-Dijkstra's path {} --------------------- Dijkstra's path {}", flattened_ch_path, flattened_path);
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
    expected_paths: Vec<(
        TestNode,
        TestNode,
        DimVec<MetricIdx>,
        Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
    )>,
    cfg: Config,
) {
    let graph = parse(cfg.parser);
    for (src, dst, metric_indices, option_specs) in expected_paths {
        let nodes = graph.nodes();
        let graph_src = nodes.create(src.idx);
        let graph_dst = nodes.create(dst.idx);
        let option_path = dijkstra.compute_best_path(
            &graph_src,
            &graph_dst,
            &graph,
            &cfg.routing
                .as_ref()
                .expect("Routing-config should be existedefaults::LENGTH_IDnt"),
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

        if let (Some((cost, nodes)), Some(actual_path)) = (option_specs, option_path) {
            TestPath::from_alternatives(src, dst, cost, metric_indices, nodes)
                .assert_correct(&actual_path, &graph);
        }
    }
}

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
    pub fn new(name: &str, id: i64, lat: f64, lon: f64, level: usize, graph: &Graph) -> TestNode {
        let idx = graph
            .nodes()
            .idx_from(id)
            .expect(&format!("The node-id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            coord: Coordinate { lat, lon },
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
    pub fn new_fwd(
        name: Option<&str>,
        edge_idx: usize,
        src: &TestNode,
        dst: &TestNode,
        length: f64,
        maxspeed: f64,
        duration: f64,
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
        length: f64,
        maxspeed: f64,
        duration: f64,
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
