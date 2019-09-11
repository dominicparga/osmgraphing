use clap;

//------------------------------------------------------------------------------------------------//

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                "You can set up the logger by setting RUST_LOG, e.g. to",
                "    export RUST_LOG='warn,osmgraphing=info,parser=info,dijkstra=info'",
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
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help(
                    &[
                        "Logs 'info' in addition to 'warn' and 'error'.",
                        "The env-variable 'RUST_LOG' has precedence."
                    ]
                    .join("\n"),
                ),
        )
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

fn main() {
    let matches = parse_cmdline();
    setup_logging(matches.is_present("verbose"));

    if matches.args.len() == 0 {
        println!("Execute '.../osmgraphing -h' (or 'cargo run -- -h') for more info.");
    }
}
