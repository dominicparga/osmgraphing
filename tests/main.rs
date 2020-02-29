#![allow(non_snake_case)]

mod configs;
mod parsing;
mod routing;

use osmgraphing::{
    configs::{graph, Config},
    network::Graph,
    Parser,
};

fn parse(cfg: graph::Config) -> Graph {
    let map_file = cfg.map_file.clone().unwrap();
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
    Config::from_map_file(match test_type {
        TestType::BidirectionalBait => "resources/maps/bidirectional_bait.fmi",
        TestType::IsleOfMan => "resources/maps/isle-of-man_2019-09-05.osm.pbf",
        TestType::SimpleStuttgart => "resources/maps/simple_stuttgart.fmi",
        TestType::Small => "resources/maps/small.fmi",
    })
    .expect("Config is tested separatedly.")
}

#[rustfmt::skip]#[test]fn fmi__________________configs___deserialize_________________________() {configs::fmi::deserialize().unwrap();}
#[rustfmt::skip]#[test]fn pbf__________________configs___deserialize_________________________() {configs::pbf::deserialize().unwrap();}
#[rustfmt::skip]#[test]fn small________________routing___shortest___astar______bidirectional_() {routing::shortest::astar::bidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___shortest___astar______unidirectional() {routing::shortest::astar::unidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___shortest___dijkstra___bidirectional_() {routing::shortest::dijkstra::bidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___shortest___dijkstra___unidirectional() {routing::shortest::dijkstra::unidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___fastest____astar______bidirectional_() {routing::fastest::astar::bidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___fastest____astar______unidirectional() {routing::fastest::astar::unidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___fastest____dijkstra___bidirectional_() {routing::fastest::dijkstra::bidirectional::small();}
#[rustfmt::skip]#[test]fn small________________routing___fastest____dijkstra___unidirectional() {routing::fastest::dijkstra::unidirectional::small();}
#[rustfmt::skip]#[test]fn small________________parsing___fmi_________________________________() {parsing::fmi::small();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___shortest___astar______bidirectional_() {routing::shortest::astar::bidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___shortest___astar______unidirectional() {routing::shortest::astar::unidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___shortest___dijkstra___bidirectional_() {routing::shortest::dijkstra::bidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___shortest___dijkstra___unidirectional() {routing::shortest::dijkstra::unidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___fastest____astar______bidirectional_() {routing::fastest::astar::bidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___fastest____astar______unidirectional() {routing::fastest::astar::unidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___fastest____dijkstra___bidirectional_() {routing::fastest::dijkstra::bidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____routing___fastest____dijkstra___unidirectional() {routing::fastest::dijkstra::unidirectional::simple_stuttgart();}
#[rustfmt::skip]#[test]fn simple_stuttgart_____parsing___fmi_________________________________() {parsing::fmi::simple_stuttgart();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___shortest___astar______bidirectional_() {routing::shortest::astar::bidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___shortest___astar______unidirectional() {routing::shortest::astar::unidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___shortest___dijkstra___bidirectional_() {routing::shortest::dijkstra::bidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___shortest___dijkstra___unidirectional() {routing::shortest::dijkstra::unidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___fastest____astar______bidirectional_() {routing::fastest::astar::bidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___fastest____astar______unidirectional() {routing::fastest::astar::unidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___fastest____dijkstra___bidirectional_() {routing::fastest::dijkstra::bidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___routing___fastest____dijkstra___unidirectional() {routing::fastest::dijkstra::unidirectional::bidirectional_bait();}
#[rustfmt::skip]#[test]fn bidirectional_bait___parsing___fmi_________________________________() {parsing::fmi::bidirectional_bait();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___shortest___astar______bidirectional_() {routing::shortest::astar::bidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___shortest___astar______unidirectional() {routing::shortest::astar::unidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___shortest___dijkstra___bidirectional_() {routing::shortest::dijkstra::bidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___shortest___dijkstra___unidirectional() {routing::shortest::dijkstra::unidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___fastest____astar______bidirectional_() {routing::fastest::astar::bidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___fastest____astar______unidirectional() {routing::fastest::astar::unidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___fastest____dijkstra___bidirectional_() {routing::fastest::dijkstra::bidirectional::isle_of_man();}
#[rustfmt::skip]#[test]#[ignore]fn isle_of_man__________routing___fastest____dijkstra___unidirectional() {routing::fastest::dijkstra::unidirectional::isle_of_man();}
#[rustfmt::skip]#[test]fn isle_of_man__________parsing___pbf_________________________________() {parsing::pbf::isle_of_man();}
#[rustfmt::skip]#[test]fn general______________parsing___wrong_extension_____________________() {parsing::general::wrong_extension();}
