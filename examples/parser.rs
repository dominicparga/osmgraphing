use log::{error, info};
use osmgraphing::{
    configs,
    helpers::{err, init_logging},
    io::network::graph::Parser,
};
use std::time::Instant;

fn main() {
    init_logging("INFO", &["parser"]).expect("LogLevel 'INFO' does exist.");
    let result = run();
    if let Err(msg) = result {
        error!("{}", msg);
        panic!("{}", msg);
    }
}

fn run() -> err::Feedback {
    info!("Executing example: parser");

    let parsing_cfg =
        configs::parsing::Config::try_from_yaml("resources/isle_of_man_2020-03-14/osm.pbf.yaml")?;

    let now = Instant::now();
    let graph = Parser::parse_and_finalize(parsing_cfg)?;
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    Ok(())
}
