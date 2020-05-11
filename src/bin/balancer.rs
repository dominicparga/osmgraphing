use log::{error, info};
use osmgraphing::{configs, helpers, io, network::RoutePair, routing};
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

    let graph = {
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

    let routing_cfg = {
        // take parsing-cfg if no other config is given

        // parse config

        match configs::routing::Config::try_from_yaml(&args.routing_cfg, graph.cfg()) {
            Ok(cfg) => cfg,
            Err(msg) => return Err(format!("{}", msg)),
        }
    };

    let mut dijkstra = routing::Dijkstra::new();
    let mut explorator = routing::ConvexHullExplorator::new();

    info!(
        "Explorate several routes for metrics {:?} of dimension {}",
        graph.cfg().edges.metrics.units,
        graph.metrics().dim()
    );

    // calculate best paths

    for RoutePair { src, dst } in io::routing::Parser::parse(&routing_cfg)?
        .iter()
        .map(|(route_pair, _)| route_pair.into_node(&graph))
    {
        let now = Instant::now();
        info!("Explore new query");
        info!("src {}", src);
        info!("dst {}", dst);
        let found_paths =
            explorator.fully_explorate(src.idx(), dst.idx(), &mut dijkstra, &graph, &routing_cfg);
        info!(
            "Ran Explorator-query in {} ms. Found {} path(s).",
            now.elapsed().as_micros() as f64 / 1_000.0,
            found_paths.len()
        );
        found_paths.iter().for_each(|p| info!("    {}", p));
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
    }
}

struct CmdlineArgs {
    max_log_level: String,
    cfg: String,
    routing_cfg: String,
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

        CmdlineArgs {
            max_log_level: String::from(max_log_level),
            cfg: String::from(cfg),
            routing_cfg: String::from(routing_cfg),
        }
    }
}
