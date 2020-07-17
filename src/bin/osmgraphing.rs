use log::{error, info};
use osmgraphing::{
    configs,
    helpers::{err, init_logging},
    io,
    network::RoutePair,
    routing,
};
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

fn main() {
    let args = parse_cmdline();
    let result = init_logging(&args.max_log_level, &[]);
    if let Err(msg) = result {
        error!("{}{}", msg, "\n");
        panic!("{}", msg);
    }
    let result = run(args);
    if let Err(msg) = result {
        error!("{}{}", msg, "\n");
        panic!("{}", msg);
    }
}

fn run(args: CmdlineArgs) -> err::Feedback {
    info!("EXECUTE {}", env!("CARGO_PKG_NAME"));

    // parse graph

    let graph = {
        // get config by provided user-input

        let parsing_cfg = {
            let raw_parsing_cfg = PathBuf::from(args.cfg.clone());
            configs::parsing::Config::try_from_yaml(&raw_parsing_cfg)?
        };

        // parse and create graph

        // measure parsing-time
        let now = Instant::now();

        let graph = io::network::graph::Parser::parse_and_finalize(parsing_cfg)?;
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

    // writing built graph

    if args.is_writing_graph {
        // get config by provided user-input

        let writing_cfg = configs::writing::network::graph::Config::try_from_yaml(&args.cfg)?;

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

        io::network::graph::Writer::write(&graph, &writing_cfg)?;
        info!(
            "Finished writing in {} seconds ({} µs).",
            now.elapsed().as_secs(),
            now.elapsed().as_micros(),
        );
        info!("");
    }

    // writing edges to file

    if args.is_writing_edges {
        // get config by provided user-input

        let writing_cfg = configs::writing::network::edges::Config::try_from_yaml(&args.cfg)?;

        // check if new file does already exist

        if writing_cfg.file.exists() {
            return Err(err::Msg::from(format!(
                "New file {} does already exist. Please remove it.",
                writing_cfg.file.display()
            )));
        }

        // writing to file

        // measure writing-time
        let now = Instant::now();

        io::network::edges::Writer::write(&graph, &writing_cfg)?;
        info!(
            "Finished writing in {} seconds ({} µs).",
            now.elapsed().as_secs(),
            now.elapsed().as_micros(),
        );
        info!("");
    }

    // writing routes to file

    if args.is_writing_routes {
        // get config by provided user-input

        let routing_cfg = configs::routing::Config::try_from_yaml(&args.cfg, graph.cfg())?;
        let writing_cfg = configs::writing::routing::Config::try_from_yaml(&args.cfg)?;

        // check if new file does already exist

        if writing_cfg.file.exists() {
            return Err(err::Msg::from(format!(
                "New routes-file {} does already exist. Please remove it.",
                writing_cfg.file.display()
            )));
        }

        // writing to file

        // measure writing-time
        let now = Instant::now();

        match io::routing::Writer::write(&graph, &routing_cfg, &writing_cfg) {
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

    // routing-example

    if args.is_routing {
        // get config by provided user-input

        let routing_cfg = configs::routing::Config::try_from_yaml(&args.cfg, graph.cfg())?;

        info!("EXECUTE Do routing with alphas: {:?}", routing_cfg.alphas);

        let mut dijkstra = routing::Dijkstra::new();

        // calculate best paths

        for RoutePair { src, dst } in io::routing::Parser::parse(&routing_cfg)?
            .iter()
            .map(|(route_pair, _)| route_pair.into_node(&graph))
        {
            info!("");

            let now = Instant::now();
            let best_path = dijkstra.compute_best_path(routing::Query {
                src_idx: src.idx(),
                dst_idx: dst.idx(),
                graph: &graph,
                routing_cfg: &routing_cfg,
            });
            info!(
                "Ran Dijkstra-query in {} ms",
                now.elapsed().as_micros() as f64 / 1_000.0,
            );
            if let Some(best_path) = best_path {
                let best_path = best_path.flatten(&graph);
                info!("Found path {}.", best_path);
            } else {
                info!("No path from ({}) to ({}).", src, dst);
            }
        }
    }
    Ok(())
}

fn parse_cmdline<'a>() -> CmdlineArgs {
    let tmp = &[
        "Sets the logging-level according to the env-variable 'RUST_LOG'.",
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

    let arg_is_routing = clap::Arg::with_name(constants::ids::IS_ROUTING)
        .long("routing")
        .help("Does routing as specified in the provided config.")
        .takes_value(false)
        .requires(constants::ids::CFG);

    let arg_is_writing_graph = clap::Arg::with_name(constants::ids::IS_WRITING_GRAPH)
        .long("writing-graph")
        .help(
            "The generated graph will be exported \
               as described in the provided config.",
        )
        .takes_value(false)
        .requires(constants::ids::CFG);

    let arg_is_writing_edges = clap::Arg::with_name(constants::ids::IS_WRITING_EDGES)
        .long("writing-edges")
        .help(
            "The generated graph's edges will be exported \
               as described in the provided config.",
        )
        .takes_value(false)
        .requires(constants::ids::CFG);

    let arg_is_writing_routes = clap::Arg::with_name(constants::ids::IS_WRITING_ROUTES)
        .long("writing-routes")
        .help(
            "The generated graph will be used to \
               generate and export valid routes \
               as described in the provided config.",
        )
        .takes_value(false)
        .requires(constants::ids::CFG);

    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                "This tool takes a config-file, parses the chosen graph with specified",
                "settings, and can execute specified tasks.",
                "Such tasks may be exporting the graph as fmi-map-file or doing some ",
                "routing-queries (if provided in config-file).",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .arg(arg_parser_cfg)
        .arg(arg_is_routing)
        .arg(arg_is_writing_graph)
        .arg(arg_is_writing_edges)
        .arg(arg_is_writing_routes)
        .get_matches()
        .into()
}

mod constants {
    pub mod ids {
        pub const MAX_LOG_LEVEL: &str = "max-log-level";
        pub const CFG: &str = "cfg";
        pub const IS_WRITING_GRAPH: &str = "is_writing_graph";
        pub const IS_WRITING_EDGES: &str = "is_writing_edges";
        pub const IS_WRITING_ROUTES: &str = "is_writing_routes";
        pub const IS_ROUTING: &str = "is_routing";
    }
}

struct CmdlineArgs {
    max_log_level: String,
    cfg: String,
    is_writing_graph: bool,
    is_writing_edges: bool,
    is_writing_routes: bool,
    is_routing: bool,
}

impl<'a> From<clap::ArgMatches<'a>> for CmdlineArgs {
    fn from(matches: clap::ArgMatches<'a>) -> CmdlineArgs {
        let max_log_level = matches
            .value_of(constants::ids::MAX_LOG_LEVEL)
            .expect(&format!("cmdline-arg: {}", constants::ids::MAX_LOG_LEVEL));
        let cfg = matches
            .value_of(constants::ids::CFG)
            .expect(&format!("cmdline-arg: {}", constants::ids::CFG));
        let is_writing_graph = matches.is_present(constants::ids::IS_WRITING_GRAPH);
        let is_writing_edges = matches.is_present(constants::ids::IS_WRITING_EDGES);
        let is_writing_routes = matches.is_present(constants::ids::IS_WRITING_ROUTES);
        let is_routing = matches.is_present(constants::ids::IS_ROUTING);

        CmdlineArgs {
            max_log_level: String::from(max_log_level),
            cfg: String::from(cfg),
            is_writing_graph,
            is_writing_edges,
            is_writing_routes,
            is_routing,
        }
    }
}
