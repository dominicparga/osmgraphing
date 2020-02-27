#![allow(non_snake_case)]

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

#[test]
fn small________________routing___shortest___astar______bidirectional_() {
    routing::shortest::astar::bidirectional::small();
}
#[test]
fn small________________routing___shortest___astar______unidirectional() {
    routing::shortest::astar::unidirectional::small();
}
#[test]
fn small________________routing___shortest___dijkstra___bidirectional_() {
    routing::shortest::dijkstra::bidirectional::small();
}
#[test]
fn small________________routing___shortest___dijkstra___unidirectional() {
    routing::shortest::dijkstra::unidirectional::small();
}
#[test]
fn small________________routing___fastest____astar______bidirectional_() {
    routing::fastest::astar::bidirectional::small();
}
#[test]
fn small________________routing___fastest____astar______unidirectional() {
    routing::fastest::astar::unidirectional::small();
}
#[test]
fn small________________routing___fastest____dijkstra___bidirectional_() {
    routing::fastest::dijkstra::bidirectional::small();
}
#[test]
fn small________________routing___fastest____dijkstra___unidirectional() {
    routing::fastest::dijkstra::unidirectional::small();
}
#[test]
fn small________________parsing___fmi_________________________________() {
    parsing::fmi::small();
}
#[test]
fn simple_stuttgart_____routing___shortest___astar______bidirectional_() {
    routing::shortest::astar::bidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___shortest___astar______unidirectional() {
    routing::shortest::astar::unidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___shortest___dijkstra___bidirectional_() {
    routing::shortest::dijkstra::bidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___shortest___dijkstra___unidirectional() {
    routing::shortest::dijkstra::unidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___fastest____astar______bidirectional_() {
    routing::fastest::astar::bidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___fastest____astar______unidirectional() {
    routing::fastest::astar::unidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___fastest____dijkstra___bidirectional_() {
    routing::fastest::dijkstra::bidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____routing___fastest____dijkstra___unidirectional() {
    routing::fastest::dijkstra::unidirectional::simple_stuttgart();
}
#[test]
fn simple_stuttgart_____parsing___fmi_________________________________() {
    parsing::fmi::simple_stuttgart();
}
#[test]
fn bidirectional_bait___routing___shortest___astar______bidirectional_() {
    routing::shortest::astar::bidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___shortest___astar______unidirectional() {
    routing::shortest::astar::unidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___shortest___dijkstra___bidirectional_() {
    routing::shortest::dijkstra::bidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___shortest___dijkstra___unidirectional() {
    routing::shortest::dijkstra::unidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___fastest____astar______bidirectional_() {
    routing::fastest::astar::bidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___fastest____astar______unidirectional() {
    routing::fastest::astar::unidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___fastest____dijkstra___bidirectional_() {
    routing::fastest::dijkstra::bidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___routing___fastest____dijkstra___unidirectional() {
    routing::fastest::dijkstra::unidirectional::bidirectional_bait();
}
#[test]
fn bidirectional_bait___parsing___fmi_________________________________() {
    parsing::fmi::bidirectional_bait();
}
#[test]
fn isle_of_man__________routing___shortest___astar______bidirectional_() {
    routing::shortest::astar::bidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___shortest___astar______unidirectional() {
    routing::shortest::astar::unidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___shortest___dijkstra___bidirectional_() {
    routing::shortest::dijkstra::bidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___shortest___dijkstra___unidirectional() {
    routing::shortest::dijkstra::unidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___fastest____astar______bidirectional_() {
    routing::fastest::astar::bidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___fastest____astar______unidirectional() {
    routing::fastest::astar::unidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___fastest____dijkstra___bidirectional_() {
    routing::fastest::dijkstra::bidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________routing___fastest____dijkstra___unidirectional() {
    routing::fastest::dijkstra::unidirectional::isle_of_man();
}
#[test]
fn isle_of_man__________parsing___pbf_________________________________() {
    parsing::pbf::isle_of_man();
}
#[test]
fn general______________parsing___wrong_extension_____________________() {
    parsing::general::wrong_extension();
}
