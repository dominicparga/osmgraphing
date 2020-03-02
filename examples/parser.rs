use log::{error, info};
use osmgraphing::{configs::Config, helpers, Parser};
use std::{path::PathBuf, time::Instant};

//------------------------------------------------------------------------------------------------//

fn main() {
    helpers::init_logging(Some("INFO"), Some(vec!["parser"])).expect("LogLevel 'INFO' does exist.");
    info!("Executing example: parser");

    // get config by provided map-file
    let cfg = {
        let cfg_file = match std::env::args_os().nth(1) {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("resources/configs/isle-of-man.pbf.yaml"),
        };
        match Config::from_yaml(&cfg_file) {
            Ok(cfg) => cfg,
            Err(msg) => {
                error!("{}", msg);
                return;
            }
        }
    };

    // measure parsing-time
    let now = Instant::now();
    // parse and create graph
    let graph = match Parser::parse_and_finalize(cfg.graph) {
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
