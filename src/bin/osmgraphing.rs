use log::error;
use osmgraphing::helpers;

fn main() {
    let matches = parse_cmdline();
    match helpers::init_logging(matches.value_of("log").unwrap(), vec![]) {
        Ok(_) => (),
        Err(msg) => {
            error!("{}", msg);
            std::process::exit(1);
        }
    };

    println!("Execute 'path/to/built/binary --help' (or 'cargo run -- --help') for more info.");
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
                "RUST_LOG is set up automatically, setting RUST_LOG to 'info'",
                "for relevant parts of the software, but consider the flag '--logging'.",
                "",
                "In case you're using cargo, please use",
                "    cargo run --example",
                "for all supported example files",
                "",
                "In addition, you can execute benchmarks, e.g.",
                "    cargo bench --bench routing -- --warm-up-time 10 --measurement-time 120",
                "and view the results in ./target/criterion/<bench>/report/index.html",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .get_matches()
}
