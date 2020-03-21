use crate::helpers::{assert_path, defaults, TestNode};
use osmgraphing::{
    configs::{self, Config},
    helpers::ApproxEq,
    io::Parser,
    network::NodeIdx,
    routing,
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};

#[test]
#[ignore]
fn compare_dijkstras() {
    let mut cfg =
        Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_CH_FMI).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}] }}",
            defaults::LENGTH_ID
        ),
        &cfg.parser,
    )
    .ok();

    let graph = Parser::parse_and_finalize(cfg.parser).unwrap();

    let nodes = graph.nodes();
    let mut dijkstra = routing::Dijkstra::new();

    let mut cfg_routing = cfg.routing.unwrap();
    cfg_routing.set_ch_dijkstra(false);
    let mut cfg_routing_ch = cfg_routing.clone();
    cfg_routing_ch.set_ch_dijkstra(true);

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
            assert!(ch_path.approx_eq(path),
                "CH-Dijkstra's path is different from Dijkstra's path. --------------------- CH-Dijkstra's {:?} --------------------- Dijkstra's {:?}",
                ch_path,
                path
        );
        }
    }
}

#[test]
#[ignore]
fn chdijkstra() {
    let mut cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}] }}",
            defaults::LENGTH_ID
        ),
        &cfg.parser,
    )
    .ok();

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths();

    assert_path(&mut dijkstra, expected_paths, cfg);
}

#[test]
#[ignore]
fn dijkstra() {
    let mut cfg = Config::from_yaml(defaults::paths::resources::configs::ISLE_OF_MAN_PBF).unwrap();
    cfg.routing = configs::routing::Config::from_str(
        &format!(
            "routing: {{ metrics: [{{ id: '{}' }}] }}",
            defaults::LENGTH_ID
        ),
        &cfg.parser,
    )
    .ok();

    let mut dijkstra = routing::Dijkstra::new();
    let expected_paths = expected_paths();

    assert_path(&mut dijkstra, expected_paths, cfg);
}

fn expected_paths() -> Vec<(TestNode, TestNode, Option<(f64, Vec<Vec<TestNode>>)>)> {
    unimplemented!("Testing routing on isle-of-man is not supported yet.")
}
