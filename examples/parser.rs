use log::{error, info};
use osmgraphing::{
    configs,
    helpers::{err, init_logging},
    io::network::Parser,
};
use std::{path::PathBuf, time::Instant};

fn main() {
    init_logging("INFO", &["parser"]).expect("LogLevel 'INFO' does exist.");
    if let Err(msg) = run() {
        error!("{}", msg);
        panic!("{}", msg);
    }
}

fn run() -> err::Feedback {
    info!("Executing example: parser");

    // get config by provided map-file
    let parsing_cfg = {
        let raw_cfg = PathBuf::from("resources/isle_of_man_2020-03-14/osm.pbf.yaml");
        match configs::parsing::Config::try_from_yaml(&raw_cfg) {
            Ok(parsing_cfg) => parsing_cfg,
            Err(msg) => return Err(err::Msg::from(format!("{}", msg))),
        }
    };

    // measure parsing-time
    let now = Instant::now();
    // parse and create graph
    let graph = match Parser::parse_and_finalize(parsing_cfg) {
        Ok(graph) => graph,
        Err(msg) => return Err(err::Msg::from(format!("{}", msg))),
    };
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    Ok(())
}
