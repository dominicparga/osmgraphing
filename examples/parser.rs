use log::{error, info};
use osmgraphing::{
    configs::{
        graph,
        graph::{edges, edges::metrics, vehicles},
        Config, MetricCategory, VehicleType,
    },
    Parser,
};
use std::{path::PathBuf, time::Instant};

//------------------------------------------------------------------------------------------------//

fn init_logging(quietly: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if quiet logging: doesn't log `info` for the server and this repo
    if !quietly {
        builder.filter(Some(env!("CARGO_PKG_NAME")), log::LevelFilter::Info);
        builder.filter(Some("parser"), log::LevelFilter::Info);
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
    init_logging(false);
    info!("Executing example: parser");

    let cfg = Config::new(graph::Config {
        map_file: PathBuf::from("resources/maps/isle-of-man_2019-09-05.osm.pbf"),
        vehicles: vehicles::Config {
            is_driver_picky: false,
            vehicle_type: VehicleType::Car,
        },
        edges: edges::Config {
            metrics: metrics::Config::create(vec![
                (MetricCategory::Id, "src-id".into(), true).into(),
                (MetricCategory::Id, "dst-id".into(), true).into(),
                (MetricCategory::Length, "length".into(), false).into(),
                (MetricCategory::Maxspeed, "maxspeed".into(), true).into(),
                (
                    MetricCategory::Duration,
                    "duration".into(),
                    false,
                    vec!["length".into(), "maxspeed".into()],
                )
                    .into(),
            ])
            .unwrap(),
        },
    });

    let now = Instant::now();
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
