use log::{debug, error, info};
use osmgraphing::{
    configs, helpers, io,
    network::{EdgeIdx, RoutePair},
    routing,
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{path::PathBuf, time::Instant};

fn main() {
    let result = run();
    if let Err(msg) = result {
        error!("{}\n", msg);
        panic!("{}", msg);
    }
}

fn run() -> Result<(), String> {
    // process user-input

    let args = parse_cmdline();
    match helpers::init_logging(&args.max_log_level, vec!["balancer"]) {
        Ok(_) => (),
        Err(msg) => return Err(format!("{}", msg)),
    };

    info!("EXECUTE {}", env!("CARGO_PKG_NAME"));

    // parse graph

    let mut graph = {
        // get config by provided user-input

        let parsing_cfg = {
            let raw_parsing_cfg = PathBuf::from(args.cfg);
            match configs::parsing::Config::try_from_yaml(&raw_parsing_cfg) {
                Ok(cfg) => cfg,
                Err(msg) => return Err(format!("{}", msg)),
            }
        };

        // parse and create graph

        // measure parsing-time
        let now = Instant::now();

        let graph = match io::network::Parser::parse_and_finalize(parsing_cfg) {
            Ok(graph) => graph,
            Err(msg) => return Err(format!("{}", msg)),
        };
        info!(
            "Finished parsing in {} seconds ({} µs).",
            now.elapsed().as_secs(),
            now.elapsed().as_micros(),
        );
        info!("");
        info!("{}", graph);
        info!("");

        graph
    };

    // routing-example

    let mut routing_cfg =
        match configs::routing::Config::try_from_yaml(&args.routing_cfg, graph.cfg()) {
            Ok(cfg) => cfg,
            Err(msg) => return Err(format!("{}", msg)),
        };

    let balancing_cfg = {
        // parse config

        let balancing_cfg = match configs::balancing::Config::try_from_yaml(&args.balancing_cfg) {
            Ok(cfg) => cfg,
            Err(msg) => return Err(format!("{}", msg)),
        };

        // check if new file does already exist

        if balancing_cfg.results_file.exists() {
            return Err(format!(
                "New results-dir {} does already exist. Please remove it.",
                balancing_cfg.results_file.display()
            ));
        }

        balancing_cfg
    };

    let mut dijkstra = routing::Dijkstra::new();
    let mut explorator = routing::ConvexHullExplorator::new();

    info!(
        "Explorate several routes for metrics {:?} of dimension {}",
        graph.cfg().edges.metrics.units,
        graph.metrics().dim()
    );

    // calculate best paths

    // collect all metric-info to edit them

    let metric_idx = match graph.cfg().edges.metrics.idx_of(&balancing_cfg.metric_id) {
        Some(idx) => idx,
        None => {
            return Err(format!(
                "Metric-id {} should be existent in graph, but isn't.",
                balancing_cfg.metric_id
            ))
        }
    };

    let route_pairs = io::routing::Parser::parse(&routing_cfg)?;
    let mut rng = rand_pcg::Pcg32::seed_from_u64(42); // TODO
    for iteration in 0..balancing_cfg.num_iterations {
        let mut next_workload: Vec<usize> = vec![0; graph.fwd_edges().count()];

        // look for best paths wrt

        if iteration <= 0 {
            routing_cfg.alphas[*metric_idx] = 0.0;
        } else {
            routing_cfg.alphas[*metric_idx] = 1.0;
        }

        // find all routes and count density on graph

        for &(route_pair, route_count) in route_pairs.iter() {
            let RoutePair { src, dst } = route_pair.into_node(&graph);

            // find explorated routes

            let now = Instant::now();
            let found_paths = explorator.fully_explorate(
                src.idx(),
                dst.idx(),
                &mut dijkstra,
                &graph,
                &routing_cfg,
            );
            debug!(
                "Ran Explorator-query from src-id {} to dst-id {} in {} ms. Found {} path(s).",
                src.id(),
                dst.id(),
                now.elapsed().as_micros() as f64 / 1_000.0,
                found_paths.len()
            );

            // Update next workload by looping over all found routes
            // -> Routes have to be flattened,
            // -> or shortcuts will lead to wrong best-paths, because counts won't be cumulated.

            if found_paths.len() > 0 {
                let die = Uniform::from(0..found_paths.len());
                for _ in 0..route_count {
                    // TODO flatten after loops and cumulate all workloads for sc-edges
                    let p = found_paths[die.sample(&mut rng)].clone().flatten(&graph);

                    debug!("    {}", p);

                    for edge_idx in p {
                        next_workload[*edge_idx] += 1;
                    }
                }
            }
        }

        // update graph with new values
        for (edge_idx, workload) in next_workload.into_iter().enumerate() {
            graph.metrics_mut()[EdgeIdx(edge_idx)][*metric_idx] = workload as f64;

            // TODO update shortcuts-metrics
        }

        // export density

        // measure writing-time
        let now = Instant::now();

        match io::balancing::Writer::write(iteration, &graph, &balancing_cfg) {
            Ok(()) => (),
            Err(msg) => return Err(format!("{}", msg)),
        };
        info!(
            "Finished writing in {} seconds ({} µs).",
            now.elapsed().as_secs(),
            now.elapsed().as_micros(),
        );
        info!("");
    }

    Ok(())
}

fn parse_cmdline<'a>() -> CmdlineArgs {
    // arg: quiet
    let tmp = &[
        "Sets the logging-level by setting environment-variable 'RUST_LOG'.",
        "The env-variable 'RUST_LOG' has precedence.",
        "It takes values of modules, e.g.",
        "export RUST_LOG='warn,osmgraphing=info'",
        "for getting warn's by default, but 'info' about the others",
    ]
    .join("\n");
    let arg_log_level = clap::Arg::with_name(constants::ids::MAX_LOG_LEVEL)
        .long("log")
        .short("l")
        .value_name("FILTER-LEVEL")
        .help(tmp)
        .takes_value(true)
        .required(false)
        .case_insensitive(true)
        .default_value("INFO")
        .possible_values(&vec!["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]);

    let arg_parser_cfg = clap::Arg::with_name(constants::ids::CFG)
        .long("config")
        .short("c")
        .alias("parsing")
        .value_name("PATH")
        .help("Sets the parser and other configurations according to this config.")
        .takes_value(true)
        .required(true);

    // all
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                "This tool takes a config-file, parses the chosen graph with specified",
                "settings, and optimizes found routes.",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .arg(arg_parser_cfg)
        .get_matches()
        .into()
}

mod constants {
    pub mod ids {
        pub const MAX_LOG_LEVEL: &str = "max-log-level";
        pub const CFG: &str = "cfg";
        pub const ROUTING_CFG: &str = "routing-cfg";
        pub const BALANCING_CFG: &str = "balancing-cfg";
    }
}

struct CmdlineArgs {
    max_log_level: String,
    cfg: String,
    routing_cfg: String,
    balancing_cfg: String,
}

impl<'a> From<clap::ArgMatches<'a>> for CmdlineArgs {
    fn from(matches: clap::ArgMatches<'a>) -> CmdlineArgs {
        let max_log_level = matches
            .value_of(constants::ids::MAX_LOG_LEVEL)
            .expect(&format!("cmdline-arg: {}", constants::ids::MAX_LOG_LEVEL));
        let cfg = matches
            .value_of(constants::ids::CFG)
            .expect(&format!("cmdline-arg: {}", constants::ids::CFG));
        let routing_cfg = match matches.value_of(constants::ids::ROUTING_CFG) {
            Some(path) => path,
            None => &cfg,
        };
        let balancing_cfg = match matches.value_of(constants::ids::BALANCING_CFG) {
            Some(path) => path,
            None => &cfg,
        };

        CmdlineArgs {
            max_log_level: String::from(max_log_level),
            cfg: String::from(cfg),
            routing_cfg: String::from(routing_cfg),
            balancing_cfg: String::from(balancing_cfg),
        }
    }
}
