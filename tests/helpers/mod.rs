// Dead code allowed, because it is actually used in test-modules, but compiler doesn't recognize.
// March 6th, 2020

use osmgraphing::{
    configs,
    defaults::capacity::DimVec,
    helpers,
    io::network::Parser,
    network::{Graph, MetricIdx, NodeIdx},
    routing,
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};

#[allow(dead_code)]
pub mod defaults {
    pub const DISTANCE_ID: &str = "kilometers";
    pub const DURATION_ID: &str = "minutes";
    pub const SPEED_ID: &str = "kmph";

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

mod components;
pub use components::{TestEdge, TestNode, TestPath};

pub fn parse(cfg: configs::parsing::Config) -> Graph {
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
            &configs::parsing::Config,
        ) -> Vec<(
            TestNode,
            TestNode,
            DimVec<MetricIdx>,
            Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
        )>,
    >,
) {
    // parse

    let parsing_cfg = configs::parsing::Config::from_yaml(config_file);
    let graph = parse(parsing_cfg);

    // set up routing

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths(graph.cfg());

    let routing_cfg = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}], is-ch-dijkstra: {} }}",
            metric_id,
            if is_ch_dijkstra { "true" } else { "false" }
        ),
        graph.cfg(),
    );

    // test

    for (src, dst, metric_indices, option_specs) in expected_paths {
        let option_path = dijkstra.compute_best_path(src.idx, dst.idx, &graph, &routing_cfg);
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

#[allow(dead_code)]
pub fn compare_dijkstras(ch_fmi_config_file: &str, metric_id: &str) {
    // parse graph

    let parsing_cfg = configs::parsing::Config::from_yaml(ch_fmi_config_file);
    let graph = Parser::parse_and_finalize(parsing_cfg).unwrap();

    // init dijkstra for routing

    let nodes = graph.nodes();
    let mut dijkstra = routing::Dijkstra::new();

    let mut routing_cfg = configs::routing::Config::from_str(
        &format!("routing: {{ metrics: [{{ id: '{}' }}] }}", metric_id),
        graph.cfg(),
    );
    routing_cfg.is_ch_dijkstra = false;
    let mut ch_routing_cfg = routing_cfg.clone();
    ch_routing_cfg.is_ch_dijkstra = true;

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

    // testing

    while let Some((src_idx, dst_idx)) = gen_route() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);

        let option_ch_path =
            dijkstra.compute_best_path(src.idx(), dst.idx(), &graph, &ch_routing_cfg);
        let option_path = dijkstra.compute_best_path(src.idx(), dst.idx(), &graph, &routing_cfg);

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
            let ch_cost = flattened_ch_path.costs();
            let cost = flattened_path.costs();
            // not approx because both Dijkstras are running on the same graph
            // -> same best path-cost should be found
            assert!(ch_cost == cost,
                "CH-Dijkstra's path's cost ({:?}) is different ({:?}) from Dijkstra's path's cost ({:?}). --------------------- CH-Dijkstra's path {} --------------------- Dijkstra's path {}",
                ch_cost, helpers::sub(&ch_cost, &cost), cost,
                flattened_ch_path,
                flattened_path
            );

            // cmp edges
            // unfortunately incorrect for alternative paths of same cost
            // assert!(flattened_ch_path == flattened_path, "CH-Dijkstra's path  is different from Dijkstra's path. --------------------- CH-Dijkstra's path {} --------------------- Dijkstra's path {}", flattened_ch_path, flattened_path);
        }
    }
}

#[allow(dead_code)]
pub fn assert_graph(
    test_nodes: Vec<TestNode>,
    fwd_test_edges: Vec<TestEdge>,
    bwd_test_edges: Vec<TestEdge>,
    graph: &Graph,
) {
    let _nodes = graph.nodes();
    let nodes = graph.nodes(); // calling twice should be fine
    let _fwd_edges = graph.fwd_edges();
    let fwd_edges = graph.fwd_edges(); // calling twice should be fine
    let _bwd_edges = graph.bwd_edges();
    let bwd_edges = graph.bwd_edges(); // calling twice should be fine

    assert_eq!(
        nodes.count(),
        test_nodes.len(),
        "Number of nodes in graph should be {} but is {}.",
        test_nodes.len(),
        nodes.count()
    );
    assert_eq!(
        fwd_edges.count(),
        fwd_test_edges.len(),
        "Number of fwd-edges in graph should be {} but is {}.",
        fwd_test_edges.len(),
        fwd_edges.count()
    );
    assert_eq!(
        bwd_edges.count(),
        bwd_test_edges.len(),
        "Number of bwd-edges in graph should be {} but is {}.",
        bwd_test_edges.len(),
        bwd_edges.count()
    );

    // for i in nodes.count()..(2 * nodes.count()) {
    //     for j in nodes.count()..(2 * nodes.count()) {
    //         assert!(
    //             fwd_edges.starting_from(NodeIdx(i)).is_none(),
    //             "NodeIdx {} >= n={} shouldn't have leaving-edges in fwd-edges",
    //             i,
    //             nodes.count()
    //         );
    //         assert!(
    //             bwd_edges.starting_from(NodeIdx(j)).is_none(),
    //             "NodeIdx {} >= n={} shouldn't have leaving-edges in bwd-edges",
    //             j,
    //             nodes.count()
    //         );
    //         assert!(
    //             fwd_edges.between(NodeIdx(i), NodeIdx(j)).is_none(),
    //             "There should be no fwd-edge from NodeIdx {} to NodeIdx {}.",
    //             i,
    //             j
    //         );
    //         assert!(
    //             bwd_edges.between(NodeIdx(j), NodeIdx(i)).is_none(),
    //             "There should be no bwd-edge from NodeIdx {} to NodeIdx {}.",
    //             j,
    //             i
    //         );
    //     }
    // }

    //--------------------------------------------------------------------------------------------//
    // testing nodes

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

    //--------------------------------------------------------------------------------------------//
    // testing forward- and backward-edges

    for test_edge in fwd_test_edges.iter().chain(bwd_test_edges.iter()) {
        test_edge.assert_correct(&graph);
    }
}
