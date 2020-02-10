use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::error;
use osmgraphing::{
    network::{Graph, NodeIdx},
    routing, Parser,
};
use std::ffi::OsString;

fn init_logging(quietly: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if quiet logging: doesn't log `info` for the server and this repo
    if !quietly {
        builder.filter(Some(env!("CARGO_PKG_NAME")), log::LevelFilter::Info);
    }
    // overwrite default with environment-variables
    if let Ok(filters) = std::env::var("RUST_LOG") {
        builder.parse_filters(&filters);
    }
    if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&write_style);
    }
    // init
    builder.init();
}

fn criterion_benchmark(c: &mut Criterion) {
    init_logging(true);

    // parsing
    let path = OsString::from("resources/maps/isle-of-man_2019-09-05.osm.pbf");
    let graph = match Parser::parse_and_finalize(&path) {
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
            "---------- ", " with short routes (~3 km) ----------",
            vec![(
                nodes.idx_from(283500532).unwrap(),
                nodes.idx_from(283501263).unwrap(),
            )],
        ),
        // medium route (~30 km)
        (
            "---------- ", " with medium routes (~30 km) ----------",
            vec![(
                nodes.idx_from(283483998).unwrap(),
                nodes.idx_from(1746745421).unwrap(),
            )],
        ),
        // long route (~56 km)
        (
            "---------- ", " with long routes (~56 km) ----------",
            vec![(
                nodes.idx_from(1151603193).unwrap(),
                nodes.idx_from(456478793).unwrap(),
            )],
        ),
    ];

    for (prefix, suffix, routes) in labelled_routes.iter() {
        // benchmarking routing
        c.bench_function(&format!("{}unidir-shortest-dijkstra{}",prefix, suffix), |b| {
            b.iter(|| unidir_shortest_dijkstra(black_box(&graph), black_box(&routes)))
        });
        c.bench_function(&format!("{}bidir-shortest-dijkstra{}", prefix,suffix), |b| {
            b.iter(|| bidir_shortest_dijkstra(black_box(&graph), black_box(&routes)))
        });
        c.bench_function(&format!("{}unidir-shortest-astar{}", prefix,suffix), |b| {
            b.iter(|| unidir_shortest_astar(black_box(&graph), black_box(&routes)))
        });
        c.bench_function(&format!("{}bidir-shortest-astar{}", prefix,suffix), |b| {
            b.iter(|| bidir_shortest_astar(black_box(&graph), black_box(&routes)))
        });
    }

    for (prefix, suffix, routes) in labelled_routes.iter() {
        // benchmarking routing
        c.bench_function(&format!("{}unidir-fastest-dijkstra{}", prefix, suffix), |b| {
            b.iter(|| unidir_fastest_dijkstra(black_box(&graph), black_box(&routes)))
        });
        c.bench_function(&format!("{}bidir-fastest-dijkstra{}", prefix, suffix), |b| {
            b.iter(|| bidir_fastest_dijkstra(black_box(&graph), black_box(&routes)))
        });
        c.bench_function(&format!("{}unidir-fastest-astar{}", prefix, suffix), |b| {
            b.iter(|| unidir_fastest_astar(black_box(&graph), black_box(&routes)))
        });
        c.bench_function(&format!("{}bidir-fastest-astar{}", prefix, suffix), |b| {
            b.iter(|| bidir_fastest_astar(black_box(&graph), black_box(&routes)))
        });
    }
}

//------------------------------------------------------------------------------------------------//

fn unidir_fastest_dijkstra(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::dijkstra::unidirectional::fastest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_fastest_dijkstra(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::dijkstra::bidirectional::fastest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn unidir_fastest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::astar::unidirectional::fastest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_fastest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::astar::bidirectional::fastest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn unidir_shortest_dijkstra(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::dijkstra::unidirectional::shortest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_shortest_dijkstra(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::dijkstra::bidirectional::shortest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn unidir_shortest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::astar::unidirectional::shortest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

fn bidir_shortest_astar(graph: &Graph, routes: &Vec<(NodeIdx, NodeIdx)>) {
    let mut astar = routing::factory::astar::bidirectional::shortest();

    let nodes = graph.nodes();
    for &(src_idx, dst_idx) in routes.iter() {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);
        let _option_path = astar.compute_best_path(&src, &dst, graph);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
