use log::{error, info};
use osmgraphing::{configs::Config, network::NodeIdx, routing, Parser};
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use std::{path::PathBuf, time::Instant};

//------------------------------------------------------------------------------------------------//
// points in Germany

// somewhere in Stuttgart (Schwabstrasse)
// id 20_443_604 osm-id 2_933_335_353 lat 48.77017570000000291 lon 9.15657690000000102

// "near" Esslingen
// id:4_647 osm-id:163_354 lat:48.66743380000000485 lon:9.24459110000000095

// somewhere in Ulm
// id 9_058_109 osm-id 580_012_224 lat 48.39352330000000535 lon 9.9816315000000006

// near Aalen
// id 54_288 osm-id 2_237_652 lat 48.88542720000000230 lon 10.13642900000000147

// somewhere in Berlin
// id 296_679 osm-id 26_765_334 lat 52.50536590000000103 lon 13.38662390000000002

//------------------------------------------------------------------------------------------------//

fn init_logging(quietly: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if quiet logging: doesn't log `info` for the server and this repo
    if !quietly {
        builder.filter(Some(env!("CARGO_PKG_NAME")), log::LevelFilter::Info);
        builder.filter(Some("astar"), log::LevelFilter::Info);
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

fn main() {
    init_logging(false);
    info!("Executing example: A*");

    // get config by provided map-file
    let cfg = {
        let map_file = match std::env::args_os().nth(1) {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("resources/maps/simple_stuttgart.fmi"),
        };
        match Config::from_map_file(&map_file) {
            Ok(cfg) => cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        }
    };

    // measure parsing-time
    let now = Instant::now();
    // parse and create graph
    let graph = match Parser::parse_and_finalize(cfg.graph) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    //--------------------------------------------------------------------------------------------//
    // astar

    let nodes = graph.nodes();
    let mut astar = routing::factory::astar::unidirectional::shortest(
        graph.cfg().edges.metrics.idx(&"Length".into()),
    );

    // generate random route-pairs
    let route_count = 100;
    let seed = 42;
    let routes = {
        let mut routes = vec![];
        // if all possible routes are less than the preferred route-count
        // -> just print all possible routes
        // else: print random routes
        if nodes.count() * nodes.count() <= route_count {
            for src_idx in (0..nodes.count()).map(NodeIdx) {
                for dst_idx in (0..nodes.count()).map(NodeIdx) {
                    routes.push((src_idx, dst_idx));
                }
            }
        } else {
            let mut rng = rand_pcg::Pcg32::seed_from_u64(seed);
            let die = Uniform::from(0..nodes.count());
            for _ in 0..route_count {
                let src_idx = NodeIdx(die.sample(&mut rng));
                let dst_idx = NodeIdx(die.sample(&mut rng));
                routes.push((src_idx, dst_idx));
            }
        }
        routes
    };

    // calculate best paths
    for (src_idx, dst_idx) in routes {
        let src = nodes.create(src_idx);
        let dst = nodes.create(dst_idx);

        info!("");

        let now = Instant::now();
        let option_path = astar.compute_best_path(&src, &dst, &graph);
        info!(
            "Ran A* in {} µs a.k.a {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        if let Some(path) = option_path {
            info!("Distance {} from ({}) to ({}).", path.cost(), src, dst);
        } else {
            info!("No path from ({}) to ({}).", src, dst);
        }
    }
}
