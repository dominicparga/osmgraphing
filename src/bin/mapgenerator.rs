use log::{error, info};
use osmgraphing::{
    configs::Config,
    helpers,
    io::{Parser, SupportingFileExts, Writer},
};
use std::{path::PathBuf, time::Instant};

fn main() {
    // process user-input
    let matches = parse_cmdline();
    match helpers::init_logging(matches.value_of("log").unwrap(), vec!["mapgenerator"]) {
        Ok(_) => (),
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

    // execute
    info!("EXECUTE mapgenerator");

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

    // check if config is correct
    let writing_cfg = match cfg.writing {
        Some(writing_cfg) => writing_cfg,
        None => {
            error!("No generator specified.");
            return;
        }
    };
    // Parser checks when parsing, but here is nicer for user
    match Parser::find_supported_ext(&cfg.parsing.map_file) {
        Ok(_) => (),
        Err(msg) => {
            error!("Wrong parser-map-file: {}", msg);
            return;
        }
    };
    match Writer::find_supported_ext(&writing_cfg.map_file) {
        Ok(_) => (),
        Err(msg) => {
            error!("Wrong generator-map-file: {}", msg);
            return;
        }
    };
    // check if new file does already exist
    if writing_cfg.map_file.exists() {
        error!(
            "New map-file {} does already exist. Please remove it.",
            writing_cfg.map_file.display()
        );
        return;
    }

    //--------------------------------------------------------------------------------------------//
    // parsing graph

    // measure parsing-time
    let now = Instant::now();
    // parse and create graph
    let graph = match Parser::parse_and_finalize(cfg.parsing) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("Wrong parser-map-file: {}", msg);
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
    // write to file

    // measure writing-time
    let now = Instant::now();
    match Writer::write(&graph, &writing_cfg) {
        Ok(()) => (),
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    info!(
        "Finished writing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
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
        .help("Sets the parser and generator according to this config.")
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
                "settings, and exports it as fmi-map-file as specified.",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .arg(arg_cfg_file)
        .get_matches()
}
