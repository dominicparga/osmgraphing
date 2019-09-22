//------------------------------------------------------------------------------------------------//
// other modules

use clap;
use log::error;

//------------------------------------------------------------------------------------------------//
// own modules

mod braess;

//------------------------------------------------------------------------------------------------//

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    //--------------------------------------------------------------------------------------------//
    // subcmd: braess

    // arg: map
    let arg_map_file_path = clap::Arg::with_name("map")
        .long("map")
        .help("The path to the map-file being parsed.")
        .takes_value(true)
        .default_value("resources/maps/simple_stuttgart.fmi");

    // arg: proto_routes
    let arg_proto_routes = clap::Arg::with_name("proto_routes")
        .long("proto-routes")
        .help("The path to the file of proto-routes (csv of (src, dst)-pairs).")
        .takes_value(true)
        .default_value("resources/braess/proto_routes.csv");

    // arg: results
    let arg_results_dir = clap::Arg::with_name("results_dir")
        .short("o")
        .long("out")
        .help("The path to the directory where the results should be stored.")
        .takes_value(true)
        .required(true);

    // arg: threads
    let arg_threads = clap::Arg::with_name("threads")
        .long("threads")
        .help("How many threads should be applied.")
        .takes_value(true)
        .default_value("8");

    // arg: routes
    let arg_route_count = clap::Arg::with_name("route_count")
        .long("route-count")
        .help("How many routes should be used from provided file.")
        .takes_value(true);

    // subcmd
    let subcmd_braess = clap::SubCommand::with_name("braess")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Executes shortest-path-algorithms and try to improve the resulting routes")
        .long_about("Executes shortest-path-algorithms and try to improve the resulting routes")
        .arg(arg_proto_routes)
        .arg(arg_map_file_path)
        .arg(arg_results_dir)
        .arg(arg_threads)
        .arg(arg_route_count);

    //--------------------------------------------------------------------------------------------//
    // subcmd: proto-routes

    // arg: seed
    let arg_seed = clap::Arg::with_name("seed")
        .long("seed")
        .help("The seed to find random src-/dst-nodes")
        .takes_value(true)
        .required(true)
        .default_value("42");

    // arg: route_count
    let arg_route_count = clap::Arg::with_name("route_count")
        .short("c")
        .long("route-count")
        .help("The amount of routes, that should be generated.")
        .takes_value(true)
        .required(true);

    // arg: map
    let arg_map_file_path = clap::Arg::with_name("map")
        .long("map")
        .help("The path to the map-file being parsed.")
        .takes_value(true)
        .required(true);

    // arg: proto_routes
    let arg_proto_routes = clap::Arg::with_name("out")
        .short("o")
        .long("out")
        .help("The path to the file of proto-routes (csv of (src, dst)-pairs).")
        .takes_value(true)
        .required(true);

    // subcmd
    let tmp = [
        "",
        "Creates a csv-file containing random src-dst-pairs related to a given map,",
        "guaranteed to have a route in this particular map.",
    ]
    .join("\n");
    let subcmd_gen_proto_routes = clap::SubCommand::with_name("gen-proto-routes")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Creates a csv-file containing random src-dst-pairs")
        .long_about(tmp.as_ref())
        .arg(arg_seed)
        .arg(arg_route_count)
        .arg(arg_map_file_path)
        .arg(arg_proto_routes);

    //--------------------------------------------------------------------------------------------//
    // return composition

    // arg: verbose
    let tmp = &[
        "Logs 'info' in addition to 'warn' and 'error'.",
        "The env-variable 'RUST_LOG' has precedence.",
    ]
    .join("\n");
    let arg_verbose = clap::Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .help(tmp);

    // all
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                "You can set up the logger by setting RUST_LOG, e.g. to",
                "    export RUST_LOG='warn,osmgraphing=info,parser=info,astar=info'",
                "for getting 'warn's per default, but 'info' about the others (e.g. 'parser').",
                "Consider the flag '--verbose', so you don't have to mess around with RUST_LOG,",
                "setting RUST_LOG to 'info' for relevant parts of the software.",
                "",
                "In case you're using cargo, please use",
                "    cargo run --example",
                "for all supported example files",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_verbose)
        .subcommand(subcmd_braess)
        .subcommand(subcmd_gen_proto_routes)
        .get_matches()
}

fn setup_logging(verbosely: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if verbose logging: log `info` for the server and this repo
    if verbosely {
        builder.filter(Some("osmgraphing"), log::LevelFilter::Info);
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

fn main() -> Result<(), ()> {
    let matches = parse_cmdline();
    setup_logging(matches.is_present("verbose"));

    if let Some(matches) = matches.subcommand_matches("gen-proto-routes") {
        let seed = match matches.value_of("seed").unwrap().parse() {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return Err(());
            }
        };
        let route_count = match matches.value_of("route_count").unwrap().parse() {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e);
                return Err(());
            }
        };

        use braess::routes::cfg;
        let cfg = cfg::Config {
            seed,
            route_count,
            paths: cfg::Paths {
                input: cfg::InputPaths {
                    files: cfg::InputFiles {
                        map: matches.value_of("map").unwrap(),
                    },
                },
                output: cfg::OutputPaths {
                    files: cfg::OutputFiles {
                        proto_routes: matches.value_of("out").unwrap(),
                    },
                },
            },
        };
        if let Err(msg) = braess::routes::search_and_export(cfg) {
            error!("{}", msg);
            return Err(());
        }
    } else if let Some(matches) = matches.subcommand_matches("braess") {
        use braess::cfg;
        let cfg = cfg::Config {
            thread_count: match matches.value_of("threads").unwrap().parse::<u8>() {
                Ok(value) => value,
                Err(e) => {
                    error!("{}", e);
                    return Err(());
                }
            },
            route_count: match matches.value_of("route_count") {
                Some(s) => Some(match s.parse::<usize>() {
                    Ok(c) => c,
                    Err(e) => {
                        error!("{}", e);
                        return Err(());
                    }
                }),
                None => None,
            },
            paths: cfg::Paths {
                input: cfg::InputPaths {
                    files: cfg::InputFiles {
                        map: matches.value_of("map").unwrap(),
                        proto_routes: matches.value_of("proto_routes").unwrap(),
                    },
                },
                output: cfg::OutputPaths {
                    dirs: cfg::OutputDirs {
                        results: matches.value_of("results_dir").unwrap(),
                    },
                },
            },
        };
        if let Err(msg) = braess::run(cfg) {
            error!("{}", msg);
            return Err(());
        }
    } else if matches.args.len() == 0 {
        println!("Execute '.../osmgraphing -h' (or 'cargo run -- -h') for more info.");
    }

    Ok(())
}
