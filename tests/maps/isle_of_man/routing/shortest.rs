use crate::helpers::{defaults, test_dijkstra_on_map, TestNode};
use osmgraphing::{
    configs::{self, Config},
    defaults::capacity::DimVec,
    io::Parser,
    network::{MetricIdx, NodeIdx},
    routing,
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};

const METRIC_ID: &str = defaults::LENGTH_ID;
const PBF_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_PBF;
const CH_FMI_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI;
const FMI_CONFIG: &str = defaults::paths::resources::configs::ISLE_OF_MAN_FMI;
const IS_CH_DIJKSTRA: bool = true;

#[test]
#[ignore]
fn compare_dijkstras_on_ch_fmi_map() {
    // build configs
    let mut cfg = Config::from_yaml(CH_FMI_CONFIG).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!("routing: {{ metrics: [{{ id: '{}' }}] }}", METRIC_ID),
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
    let route_count = 1_000;
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
        if let (Some(ch_path), Some(path)) = (&option_ch_path, &option_path) {
            assert!(ch_path == path,
                "CH-Dijkstra's path is different from Dijkstra's path. --------------------- CH-Dijkstra's {:?} --------------------- Dijkstra's {:?}",
                ch_path,
                path
        );
        }
    }
}

#[test]
#[ignore]
fn chdijkstra_on_ch_fmi_map() {
    test_dijkstra_on_map(
        CH_FMI_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_ch_fmi_map() {
    test_dijkstra_on_map(
        CH_FMI_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn chdijkstra_on_fmi_map() {
    test_dijkstra_on_map(
        FMI_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_fmi_map() {
    test_dijkstra_on_map(
        FMI_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn chdijkstra_on_pbf_map() {
    test_dijkstra_on_map(
        PBF_CONFIG,
        METRIC_ID,
        IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

#[test]
#[ignore]
fn dijkstra_on_pbf_map() {
    test_dijkstra_on_map(
        PBF_CONFIG,
        METRIC_ID,
        !IS_CH_DIJKSTRA,
        Box::new(expected_paths),
    )
}

fn expected_paths(
    _cfg_parser: &configs::parser::Config,
) -> Vec<(
    TestNode,
    TestNode,
    DimVec<MetricIdx>,
    Option<(DimVec<f64>, Vec<Vec<TestNode>>)>,
)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
