use log::info;
use osmgraphing::{configs, helpers, io, network::RoutePair, routing};
use std::{path::PathBuf, time::Instant};

fn main() -> Result<(), String> {
    // process user-input

    let matches = parse_cmdline();
    match helpers::init_logging(matches.value_of("log").unwrap(), vec!["balancer"]) {
        Ok(_) => (),
        Err(msg) => return Err(format!("{}", msg)),
    };

    info!("EXECUTE {}", env!("CARGO_PKG_NAME"));

    //--------------------------------------------------------------------------------------------//
    // parsing config

    // get config by provided user-input

    let raw_cfg = PathBuf::from(matches.value_of("cfg").unwrap());
    let parsing_cfg = {
        match configs::parsing::Config::try_from_yaml(&raw_cfg) {
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

    //--------------------------------------------------------------------------------------------//
    // routing-example

    let routing_cfg = {
        match configs::routing::Config::try_from_yaml(&raw_cfg, graph.cfg()) {
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

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    // arg: quiet
    let tmp = &[
        "Sets the logging-level by setting environment-variable 'RUST_LOG'.",
        "The env-variable 'RUST_LOG' has precedence.",
        "It takes values of modules, e.g.",
        "export RUST_LOG='warn,osmgraphing=info'",
        "for getting warn's by default, but 'info' about the others",
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
                "",
                "This tool takes a config-file, parses the chosen graph with specified",
                "settings, and optimizes found routes.",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .arg(arg_cfg_file)
        .get_matches()
}
