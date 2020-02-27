mod network;
mod parsing;
mod routing;

use osmgraphing::{
    configs::{
        graph,
        graph::{edges, edges::metrics, vehicles},
        Config, MetricCategory, VehicleType,
    },
    network::Graph,
    Parser,
};
use std::path::PathBuf;

fn parse(cfg: graph::Config) -> Graph {
    let map_file = cfg.map_file.clone();
    match Parser::parse_and_finalize(cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!("Could not parse {}. ERROR: {}", map_file.display(), msg);
        }
    }
}

enum TestType {
    BidirectionalBait,
    IsleOfMan,
    SimpleStuttgart,
    Small,
}

fn create_config(test_type: TestType) -> Config {
    let map_file = match test_type {
        TestType::BidirectionalBait => PathBuf::from("resources/maps/bidirectional_bait.fmi"),
        TestType::IsleOfMan => PathBuf::from("resources/maps/isle-of-man_2019-09-05.osm.pbf"),
        TestType::SimpleStuttgart => PathBuf::from("resources/maps/simple_stuttgart.fmi"),
        TestType::Small => PathBuf::from("resources/maps/small.fmi"),
    };

    match test_type {
        // fmi-file
        TestType::BidirectionalBait | TestType::SimpleStuttgart | TestType::Small => {
            Config::new(graph::Config {
                map_file,
                vehicles: vehicles::Config {
                    is_driver_picky: false,
                    vehicle_type: VehicleType::Car,
                },
                edges: edges::Config {
                    metrics: metrics::Config::create(vec![
                        (MetricCategory::Id, "src-id".into(), true).into(),
                        (MetricCategory::Id, "dst-id".into(), true).into(),
                        (MetricCategory::Length, "length".into(), true).into(),
                        (MetricCategory::Ignore, "?".into(), false).into(),
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
            })
        }
        // pbf-file
        TestType::IsleOfMan => Config::new(graph::Config {
            map_file,
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
        }),
    }
}
