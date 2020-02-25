mod network;
mod parsing;
mod routing;

use osmgraphing::{
    configs::{
        graph,
        graph::{edges, vehicles},
        Config, MetricType, VehicleType,
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
    match test_type {
        TestType::BidirectionalBait => Config::new(graph::Config {
            map_file: PathBuf::from("resources/maps/bidirectional_bait.fmi"),
            vehicles: vehicles::Config {
                is_driver_picky: false,
                vehicle_type: VehicleType::Car,
            },
            edges: edges::Config {
                metric_types: vec![
                    MetricType::Id {
                        id: "src-id".to_owned(),
                    },
                    MetricType::Id {
                        id: "dst-id".to_owned(),
                    },
                    MetricType::Length { provided: true },
                    MetricType::Ignore { id: "?".to_owned() },
                    MetricType::Maxspeed { provided: true },
                    MetricType::Duration { provided: false },
                ],
            },
        }),
        TestType::IsleOfMan => Config::new(graph::Config {
            map_file: PathBuf::from("resources/maps/isle-of-man_2019-09-05.osm.pbf"),
            vehicles: vehicles::Config {
                is_driver_picky: false,
                vehicle_type: VehicleType::Car,
            },
            edges: edges::Config {
                metric_types: vec![
                    MetricType::Id {
                        id: "src-id".to_owned(),
                    },
                    MetricType::Id {
                        id: "dst-id".to_owned(),
                    },
                    MetricType::Length { provided: false },
                    MetricType::Maxspeed { provided: true },
                    MetricType::Duration { provided: false },
                ],
            },
        }),
        TestType::SimpleStuttgart => Config::new(graph::Config {
            map_file: PathBuf::from("resources/maps/simple_stuttgart.fmi"),
            vehicles: vehicles::Config {
                is_driver_picky: false,
                vehicle_type: VehicleType::Car,
            },
            edges: edges::Config {
                metric_types: vec![
                    MetricType::Id {
                        id: "src-id".to_owned(),
                    },
                    MetricType::Id {
                        id: "dst-id".to_owned(),
                    },
                    MetricType::Length { provided: true },
                    MetricType::Ignore { id: "?".to_owned() },
                    MetricType::Maxspeed { provided: true },
                    MetricType::Duration { provided: false },
                ],
            },
        }),
        TestType::Small => Config::new(graph::Config {
            map_file: PathBuf::from("resources/maps/small.fmi"),
            vehicles: vehicles::Config {
                is_driver_picky: false,
                vehicle_type: VehicleType::Car,
            },
            edges: edges::Config {
                metric_types: vec![
                    MetricType::Id {
                        id: "src-id".to_owned(),
                    },
                    MetricType::Id {
                        id: "dst-id".to_owned(),
                    },
                    MetricType::Length { provided: true },
                    MetricType::Ignore { id: "?".to_owned() },
                    MetricType::Maxspeed { provided: true },
                    MetricType::Duration { provided: false },
                ],
            },
        }),
    }
}
