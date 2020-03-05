use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::error;
use osmgraphing::{
    configs::Config,
    helpers,
    network::{Graph, MetricIdx, NodeIdx},
    routing, Parser,
};

fn criterion_benchmark(c: &mut Criterion) {
    helpers::init_logging("WARN", vec![]).expect("No user-input, so this should be fine.");

    // parsing
    let cfg = Config::from_yaml("resources/configs/isle-of-man.pbf.yaml").unwrap();

    // indices for routing
    let length_idx = cfg.graph.edges.metrics.idx(&"Meters".into());
    let _maxspeed_idx = cfg.graph.edges.metrics.idx(&"KilometersPerHour".into());
    let duration_idx = cfg.graph.edges.metrics.idx(&"Seconds".into());
    // create graph
    let graph = match Parser::parse_and_finalize(cfg.graph) {
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
    for (prefix, suffix, routes) in labelled_routes.iter() {
        c.bench_function(
            &format!("{}Shortest Dijkstra (unidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    unidir_shortest_dijkstra(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(length_idx),
                    )
                })
            },
        );
        c.bench_function(
            &format!("{}Shortest Dijkstra (bidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    bidir_shortest_dijkstra(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(length_idx),
                    )
                })
            },
        );
        c.bench_function(
            &format!("{}Shortest Astar (unidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    unidir_shortest_astar(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(length_idx),
                    )
                })
            },
        );
        c.bench_function(
            &format!("{}Shortest Astar (bidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    bidir_shortest_astar(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(length_idx),
                    )
                })
            },
        );
    }

    // benchmarking fastest routing
    for (prefix, suffix, routes) in labelled_routes.iter() {
        c.bench_function(
            &format!("{}Fastest Dijkstra (unidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    unidir_fastest_dijkstra(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(duration_idx),
                    )
                })
            },
        );
        c.bench_function(
            &format!("{}Fastest Dijkstra (bidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    bidir_fastest_dijkstra(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(duration_idx),
                    )
                })
            },
        );
        c.bench_function(
            &format!("{}Fastest Astar (unidir){}", prefix, suffix),
            |b| {
                b.iter(|| {
                    unidir_fastest_astar(
                        black_box(&graph),
                        black_box(&routes),
                        black_box(duration_idx),
                    )
                })
            },
        );
        c.bench_function(&format!("{}Fastest Astar (bidir){}", prefix, suffix), |b| {
            b.iter(|| {
                bidir_fastest_astar(
                    black_box(&graph),
                    black_box(&routes),
                    black_box(duration_idx),
                )
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//------------------------------------------------------------------------------------------------//

fn unidir_shortest_dijkstra(
    graph: &Graph,
    routes: &Vec<(NodeIdx, NodeIdx)>,
    length_idx: MetricIdx,
) {
    let mut dijkstra = routing::factory::dijkstra::unidirectional::shortest(length_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = dijkstra.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_shortest_dijkstra(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>, length_idx: MetricIdx) {
    let mut dijkstra = routing::factory::dijkstra::bidirectional::shortest(length_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = dijkstra.compute_best_path(&src, &dst, graph);
    }
}

fn unidir_shortest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>, length_idx: MetricIdx) {
    let mut astar = routing::factory::astar::unidirectional::shortest(length_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_shortest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>, length_idx: MetricIdx) {
    let mut astar = routing::factory::astar::bidirectional::shortest(length_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn unidir_fastest_dijkstra(
    graph: &Graph,
    routes: &Vec<(NodeIdx, NodeIdx)>,
    duration_idx: MetricIdx,
) {
    let mut dijkstra = routing::factory::dijkstra::unidirectional::fastest(duration_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = dijkstra.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_fastest_dijkstra(
    graph: &Graph,
    routes: &Vec<(NodeIdx, NodeIdx)>,
    duration_idx: MetricIdx,
) {
    let mut dijkstra = routing::factory::dijkstra::bidirectional::fastest(duration_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = dijkstra.compute_best_path(&src, &dst, graph);
    }
}

fn unidir_fastest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>, duration_idx: MetricIdx) {
    let mut astar = routing::factory::astar::unidirectional::fastest(duration_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_fastest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>, duration_idx: MetricIdx) {
    let mut astar = routing::factory::astar::bidirectional::fastest(duration_idx);

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}
