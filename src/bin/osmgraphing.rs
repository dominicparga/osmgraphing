use log::info;
use osmgraphing::{
    configs, helpers,
    io::{Parser, Writer},
    network::NodeIdx,
    routing,
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
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

fn main() -> Result<(), String> {
    // process user-input

    let matches = parse_cmdline();
    match helpers::init_logging(matches.value_of("log").unwrap(), vec![]) {
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

    let graph = match Parser::parse_and_finalize(parsing_cfg) {
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
    // writing built graph

    if matches.is_present("is-writing") {
        // get config by provided user-input

        let writing_cfg = {
            match configs::writing::Config::try_from_yaml(&raw_cfg) {
                Ok(cfg) => cfg,
                Err(msg) => return Err(format!("{}", msg)),
            }
        };

        // check if new file does already exist

        if writing_cfg.map_file.exists() {
            return Err(format!(
                "New map-file {} does already exist. Please remove it.",
                writing_cfg.map_file.display()
            ));
        }

        // writing to file

        // measure writing-time
        let now = Instant::now();

        match Writer::write(&graph, &writing_cfg) {
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

    //--------------------------------------------------------------------------------------------//
    // routing-example

    if matches.is_present("is-routing") {
        let routing_cfg = {
            match configs::routing::Config::try_from_yaml(&raw_cfg, graph.cfg()) {
                Ok(cfg) => cfg,
                Err(msg) => return Err(format!("{}", msg)),
            }
        };

        info!("EXECUTE Do routing with alphas: {:?}", routing_cfg.alphas);

        let nodes = graph.nodes();
        let mut dijkstra = routing::Dijkstra::new();

        // generate random route-pairs
        let route_count = 100;
        let seed = 42;

        // if all possible routes are less than the preferred route-count
        // -> just print all possible routes
        // else: print random routes
        let mut gen_route: Box<dyn FnMut() -> Option<(NodeIdx, NodeIdx)>> = {
            if nodes.count() * nodes.count() <= route_count {
                let mut i = 0;
                let nodes = graph.nodes();
                Box::new(move || {
                    if i < nodes.count() * nodes.count() {
                        let src_idx = NodeIdx(i / nodes.count());
                        let dst_idx = NodeIdx(i % nodes.count());
                        i += 1;
                        Some((src_idx, dst_idx))
                    } else {
                        None
                    }
                })
            } else {
                let mut rng = rand_pcg::Pcg32::seed_from_u64(seed);
                let die = Uniform::from(0..nodes.count());
                let mut i = 0;
                Box::new(move || {
                    if i < route_count {
                        let src_idx = NodeIdx(die.sample(&mut rng));
                        let dst_idx = NodeIdx(die.sample(&mut rng));
                        i += 1;
                        Some((src_idx, dst_idx))
                    } else {
                        None
                    }
                })
            }
        };

        // calculate best paths
        while let Some((src, dst)) =
            gen_route().map(|(src_idx, dst_idx)| (nodes.create(src_idx), nodes.create(dst_idx)))
        {
            info!("");

            let now = Instant::now();
            let best_path = dijkstra.compute_best_path(src.idx(), dst.idx(), &graph, &routing_cfg);
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

    let arg_is_writing = clap::Arg::with_name("is-writing")
        .long("writing")
        .help(
            "If provided, the generated graph will be exported \
             as described in the provided config.",
        )
        .takes_value(false)
        .required(false);

    let arg_is_routing = clap::Arg::with_name("is-routing")
        .long("routing")
        .help(
            "If provided, the generated graph will be used \
             for routing-queries as described in the provided config.",
        )
        .takes_value(false)
        .required(false);

    // all
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
        .arg(arg_cfg_file)
        .arg(arg_is_writing)
        .arg(arg_is_routing)
        .get_matches()
}
