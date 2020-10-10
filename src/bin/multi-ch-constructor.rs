use log::{error, info};
use osmgraphing::{
    helpers::{err, init_logging},
    multi_ch_constructor::{self, Config},
};

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
    info!("EXECUTE multi-ch-constructor");

    let mchc_cfg = Config::try_from_yaml(&args.cfg)?;
    multi_ch_constructor::build(&mchc_cfg)?;
    multi_ch_constructor::construct_ch_graph(&mchc_cfg)?;

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

    let arg_cfg = clap::Arg::with_name(constants::ids::CFG)
        .long("config")
        .short("c")
        .value_name("PATH")
        .help("Sets the constructor's configuration according to this config.")
        .takes_value(true)
        .required(true);

    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                &format!(
                    "{}{}",
                    "This tool takes a config-file and constructs a contracted graph",
                    " from the provided graph-file."
                ),
                "settings, and can execute specified tasks.",
                "Such tasks may be exporting the graph as fmi-map-file or doing some ",
                "routing-queries (if provided in config-file).",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_log_level)
        .arg(arg_cfg)
        .get_matches()
        .into()
}

mod constants {
    pub mod ids {
        pub const MAX_LOG_LEVEL: &str = "max-log-level";
        pub const CFG: &str = "cfg";
    }
}

struct CmdlineArgs {
    max_log_level: String,
    cfg: String,
}

impl<'a> From<clap::ArgMatches<'a>> for CmdlineArgs {
    fn from(matches: clap::ArgMatches<'a>) -> CmdlineArgs {
        let max_log_level = matches
            .value_of(constants::ids::MAX_LOG_LEVEL)
            .expect(&format!("cmdline-arg: {}", constants::ids::MAX_LOG_LEVEL));
        let cfg = matches
            .value_of(constants::ids::CFG)
            .expect(&format!("cmdline-arg: {}", constants::ids::CFG));

        CmdlineArgs {
            max_log_level: String::from(max_log_level),
            cfg: String::from(cfg),
        }
    }
}
