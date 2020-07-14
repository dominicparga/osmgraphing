use criterion::{black_box, Criterion};
use log::error;
use osmgraphing::{
    configs, helpers,
    io::network::graph::Parser,
    network::{Graph, NodeIdx},
    routing,
};
use std::time::Duration;

fn main() {
    let mut criterion = Criterion::default()
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(120))
        .configure_from_args();
    do_benchmark(&mut criterion);
    criterion.final_summary();
}

fn do_benchmark(criterion: &mut Criterion) {
    helpers::init_logging("WARN", &[]).expect("No user-input, so this should be fine.");

    // parsing
    let parsing_cfg =
        configs::parsing::Config::from_yaml("resources/isle_of_man_2020-03-14/osm.pbf.yaml");
    let routing_strs = vec![
        "routing: { metrics: [{ id: 'kilometers' }] }",
        "routing: { metrics: [{ id: 'kilometers' }, { id: 'hours' }] }",
    ];

    // create graph
    let graph = match Parser::parse_and_finalize(parsing_cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    let nodes = graph.nodes();

    // routing
    let labelled_routes = vec![
        // short route (~3 km)
        (
            "",
            " with short routes (~3 km)",
            vec![(
                nodes.idx_from(283_500_532).expect("A"),
                nodes.idx_from(283_501_263).expect("B"),
            )],
        ),
        // medium route (~30 km)
        (
            "",
            " with medium routes (~30 km)",
            vec![(
                nodes.idx_from(283_483_998).expect("C"),
                nodes.idx_from(1_746_745_421).expect("D"),
            )],
        ),
        // long route (~56 km)
        (
            "",
            " with long routes (~56 km)",
            vec![(
                nodes.idx_from(1_151_603_193).expect("E"),
                nodes.idx_from(456_478_793).expect("F"),
            )],
        ),
    ];

    // benchmarking shortest routing
    for routing_str in routing_strs {
        let routing_cfg = configs::routing::Config::from_str(routing_str, graph.cfg());

        for (prefix, suffix, routes) in labelled_routes.iter() {
            criterion.bench_function(
                &format!("{}Shortest Dijkstra (bidir){}", prefix, suffix),
                |b| {
                    b.iter(|| {
                        bidir_shortest_dijkstra(
                            black_box(&graph),
                            black_box(&routes),
                            black_box(&routing_cfg),
                        )
                    })
                },
            );
        }

        // benchmarking fastest routing
        for (prefix, suffix, routes) in labelled_routes.iter() {
            criterion.bench_function(
                &format!("{}Fastest Dijkstra (bidir){}", prefix, suffix),
                |b| {
                    b.iter(|| {
                        bidir_fastest_dijkstra(
                            black_box(&graph),
                            black_box(&routes),
                            black_box(&routing_cfg),
                        )
                    })
                },
            );
        }
    }
}

fn bidir_shortest_dijkstra(
    graph: &Graph,
    routes: &Vec<(NodeIdx, NodeIdx)>,
    cfg: &configs::routing::Config,
) {
    let mut dijkstra = routing::Dijkstra::new();

    for &(src_idx, dst_idx) in routes.iter() {
        let _option_path = dijkstra.compute_best_path(routing::Query {
            src_idx,
            dst_idx,
            graph,
            routing_cfg: cfg,
        });
    }
}

fn bidir_fastest_dijkstra(
    graph: &Graph,
    routes: &Vec<(NodeIdx, NodeIdx)>,
    cfg: &configs::routing::Config,
) {
    let mut dijkstra = routing::Dijkstra::new();

    for &(src_idx, dst_idx) in routes.iter() {
        let _option_path = dijkstra.compute_best_path(routing::Query {
            src_idx,
            dst_idx,
            graph,
            routing_cfg: cfg,
        });
    }
}
