use log::{debug, error, info};
use osmgraphing::{
    configs, defaults,
    helpers::{err, init_logging},
    io,
    network::RoutePair,
    routing,
};
use progressing::{Bar, MappingBar};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{fs, path::PathBuf, time::Instant};

fn main() {
    let result = run();
    if let Err(msg) = result {
        error!("{}\n", msg);
        std::process::exit(1);
    }
}

fn run() -> err::Feedback {
    // process user-input

    let args = parse_cmdline();
    match init_logging(&args.max_log_level, &["balancer"]) {
        Ok(_) => (),
        Err(msg) => return Err(format!("{}", msg).into()),
    };

    info!("EXECUTE balancer");

    // parse graph

    let mut graph = {
        // get config by provided user-input

        let parsing_cfg = {
            let raw_parsing_cfg = PathBuf::from(args.cfg);
            match configs::parsing::Config::try_from_yaml(&raw_parsing_cfg) {
                Ok(cfg) => cfg,
                Err(msg) => return Err(format!("{}", msg).into()),
            }
        };

        // parse and create graph

        // measure parsing-time
        let now = Instant::now();

        let graph = match io::network::Parser::parse_and_finalize(parsing_cfg) {
            Ok(graph) => graph,
            Err(msg) => return Err(format!("{}", msg).into()),
        };
        info!(
            "FINISHED Parsed graph in {} seconds ({} µs).",
            now.elapsed().as_secs(),
            now.elapsed().as_micros(),
        );
        info!("");
        info!("{}", graph);
        info!("");

        graph
    };

    // execute routing-example and count workload

    {
        let routing_cfg =
            match configs::routing::Config::try_from_yaml(&args.routing_cfg, graph.cfg()) {
                Ok(cfg) => cfg,
                Err(msg) => return Err(format!("{}", msg).into()),
            };

        let balancing_cfg = {
            // parse config

            let balancing_cfg =
                match configs::balancing::Config::try_from_yaml(&args.balancing_cfg, graph.cfg()) {
                    Ok(cfg) => cfg,
                    Err(msg) => return Err(format!("{}", msg).into()),
                };

            // check if new file does already exist

            if balancing_cfg.results_dir.exists() {
                return Err(format!(
                    "Directory {} for results does already exist. Please remove it.",
                    balancing_cfg.results_dir.display()
                )
                .into());
            } else {
                fs::create_dir_all(&balancing_cfg.results_dir).map_err(err::Msg::from)?
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

        // calculate best paths and analyze workload

        let route_pairs = io::routing::Parser::parse(&routing_cfg)?;
        let mut rng = rand_pcg::Pcg32::seed_from_u64(defaults::SEED);

        // simple init-logging

        info!("START Executing routes and analyzing workload",);
        let mut progress_bar = MappingBar::new(0..=route_pairs.len());
        info!("{}", progress_bar);

        // find all routes and count density on graph

        let mut workloads: Vec<usize> = vec![0; graph.fwd_edges().count()];

        for &(route_pair, num_routes) in &route_pairs {
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
                for _ in 0..num_routes {
                    let p = found_paths[die.sample(&mut rng)].clone().flatten(&graph);

                    debug!("    {}", p);

                    for edge_idx in p {
                        workloads[*edge_idx] += 1;
                    }
                }
            }

            progress_bar.add(true);
            if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }
        }

        // update graph with new values
        defaults::vehicles::update_new_metric(&workloads, &mut graph, &balancing_cfg);

        // export density

        // measure writing-time
        let now = Instant::now();

        match io::balancing::Writer::write(&workloads, &graph, &balancing_cfg) {
            Ok(()) => (),
            Err(msg) => return Err(format!("{}", msg).into()),
        };
        info!(
            "FINISHED Written in {} seconds ({} µs).",
            now.elapsed().as_secs(),
            now.elapsed().as_micros(),
        );
        info!("");

        info!("FINISHED",);
    }

    // write fmi-graph

    {
        // get config by provided user-input
        let writing_cfg =
            configs::writing::network::Config::try_from_yaml(&args.writing_graph_cfg)?;

        // check if new file does already exist

        if writing_cfg.map_file.exists() {
            return Err(err::Msg::from(format!(
                "New map-file {} does already exist. Please remove it.",
                writing_cfg.map_file.display()
            )));
        }

        // writing to file

        // measure writing-time
        let now = Instant::now();

        match io::network::Writer::write(&graph, &writing_cfg) {
            Ok(()) => (),
            Err(msg) => return Err(err::Msg::from(format!("{}", msg))),
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
        pub const WRITING_GRAPH_CFG: &str = "writing-graph-cfg";
    }
}

struct CmdlineArgs {
    max_log_level: String,
    cfg: String,
    routing_cfg: String,
    balancing_cfg: String,
    writing_graph_cfg: String,
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
        let writing_graph_cfg = match matches.value_of(constants::ids::WRITING_GRAPH_CFG) {
            Some(path) => path,
            None => &cfg,
        };

        CmdlineArgs {
            max_log_level: String::from(max_log_level),
            cfg: String::from(cfg),
            routing_cfg: String::from(routing_cfg),
            balancing_cfg: String::from(balancing_cfg),
            writing_graph_cfg: String::from(writing_graph_cfg),
        }
    }
}
