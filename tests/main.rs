mod network;
mod parsing;
mod routing;

use osmgraphing::{
    configs::{edges, graph, paths, MetricType, VehicleType},
    network::Graph,
    Parser,
};
use std::path::PathBuf;

fn parse(cfg: &graph::Config) -> Graph {
    match Parser::parse_and_finalize(cfg) {
        Ok(graph) => graph,
        Err(msg) => {
            panic!(
                "Could not parse {}. ERROR: {}",
                cfg.paths.map_file.display(),
                msg
            );
        }
    }
}

enum TestType {
    BidirectionalBait,
    IsleOfMan,
    SimpleStuttgart,
    Small,
}

fn create_config(test_type: TestType) -> graph::Config {
    match test_type {
        TestType::BidirectionalBait => graph::Config {
            is_graph_suitable: false,
            vehicle_type: VehicleType::Car,
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/bidirectional_bait.fmi"),
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
        },
        TestType::IsleOfMan => graph::Config {
            is_graph_suitable: false,
            vehicle_type: VehicleType::Car,
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/isle-of-man_2019-09-05.osm.pbf"),
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
        },
        TestType::SimpleStuttgart => graph::Config {
            is_graph_suitable: false,
            vehicle_type: VehicleType::Car,
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/simple_stuttgart.fmi"),
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
        },
        TestType::Small => graph::Config {
            is_graph_suitable: false,
            vehicle_type: VehicleType::Car,
            paths: paths::Config {
                map_file: PathBuf::from("resources/maps/small.fmi"),
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
        },
    }
}
