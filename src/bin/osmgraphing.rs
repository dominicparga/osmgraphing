use log::{error, info};
use osmgraphing::{configs::Config, helpers, network::NodeIdx, routing, Parser};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{path::PathBuf, time::Instant};

fn main() {
    // process user-input
    let matches = parse_cmdline();
    match helpers::init_logging(matches.value_of("log").unwrap(), vec![]) {
        Ok(_) => (),
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

    // execute
    info!("Executing {}", env!("CARGO_PKG_NAME"));

    //--------------------------------------------------------------------------------------------//
    // parsing config

    // get config by provided map-file
    let cfg = {
        let cfg_file = PathBuf::from(matches.value_of("cfg").unwrap());
        match Config::from_yaml(&cfg_file) {
            Ok(cfg) => cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        }
    };
    if cfg.routing.dim() > 0 {
        info!(
            "Parsing map-file {:?}, then routing with {} metric(s)",
            cfg.graph.map_file,
            cfg.routing.dim()
        );
    } else {
        info!(
            "Parsing map-file {:?} without routing (no metrics specified)",
            cfg.graph.map_file
        );
    }

    //--------------------------------------------------------------------------------------------//
    // parsing graph

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

    // if no routing specified -> exit
    if cfg.routing.dim() <= 0 {
        return;
    }

    //--------------------------------------------------------------------------------------------//
    // executing dijkstra-queries

    let nodes = graph.nodes();
    let mut dijkstra = routing::Dijkstra::new();

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
        let option_path = dijkstra.compute_best_path(&src, &dst, &graph, &cfg.routing);
        info!(
            "Ran Dijkstra-query in {} ms",
            now.elapsed().as_micros() as f32 / 1_000.0,
        );
        if let Some(path) = option_path {
            info!("Cost {:?} from ({}) to ({}).", path.cost(), src, dst);
        } else {
            info!("No path from ({}) to ({}).", src, dst);
        }
    }
}

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    // arg: quiet
    let tmp = &[
        "Sets the logging-level.",
        "The env-variable 'RUST_LOG' has precedence.",
    ]
    .join("\n");
    let arg_log_level = clap::Arg::with_name("log")
        .long("log")
        .short("l")
        .value_name("FILTER-LEVEL")
        .help(tmp)
        .takes_value(true)
        .required(false)
        .default_value("INFO")
        .possible_values(&vec!["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]);

    let arg_cfg_file = clap::Arg::with_name("cfg")
        .long("config")
        .short("c")
        .value_name("PATH")
        .help("Sets the parser and routing according to this config.")
        .takes_value(true)
        .required(true);

    // all
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "LOGGING",
                "",
                "You can set up the logger by setting RUST_LOG, e.g. to",
                "    export RUST_LOG='warn,osmgraphing=info,parser=info,dijkstra=info'",
                "for getting 'warn's per default, but 'info' about the others (e.g. 'parser').",
                "RUST_LOG is set up automatically, setting RUST_LOG to 'info'",
                "for relevant parts of the software, but consider the flag '--logging'.",
                "",
                "",
                "EXAMPLES",
                "",
                "In case you're using cargo, please use",
                "    cargo run --example",
                "for all supported example files",
                "",
                "",
                "BENCHMARKS",
                "",
                "In addition, you can execute benchmarks, e.g.",
                "    cargo bench --bench routing -- --warm-up-time 10 --measurement-time 120",
                "and view the results in ./target/criterion/<bench>/report/index.html",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .arg(arg_cfg_file)
        .get_matches()
}
