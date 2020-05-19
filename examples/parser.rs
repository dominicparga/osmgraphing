use log::{error, info};
use osmgraphing::{configs, helpers::init_logging, io::network::Parser};
use std::{path::PathBuf, time::Instant};

fn main() {
    init_logging("INFO", &["parser"]).expect("LogLevel 'INFO' does exist.");
    info!("Executing example: parser");

    // get config by provided map-file
    let parsing_cfg = {
        let raw_cfg = PathBuf::from("resources/configs/isle-of-man_2020-03-14.pbf.yaml");
        match configs::parsing::Config::try_from_yaml(&raw_cfg) {
            Ok(parsing_cfg) => parsing_cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        }
    };

    // measure parsing-time
    let now = Instant::now();
    // parse and create graph
    let graph = match Parser::parse_and_finalize(parsing_cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
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
}
