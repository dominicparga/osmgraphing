use std::fs::File;
use std::io::BufRead;

use log::{info, warn};

use crate::network::GraphBuilder;

//------------------------------------------------------------------------------------------------//

mod fmi {
    pub use std::io::BufReader as Reader;
    use std::str;

    use crate::network::geo;

    //--------------------------------------------------------------------------------------------//

    pub struct ProtoNode {
        pub id: i64,
        pub coord: geo::Coordinate,
    }

    impl str::FromStr for ProtoNode {
        type Err = String;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let params: Vec<&str> = line.split_whitespace().collect();

            let n = 4;
            if params.len() < n {
                return Err(format!(
                    "Not enough params for a node ({}, but should be {}).",
                    params.len(),
                    n
                ));
            }

            let id = match params[0].parse::<i64>() {
                Ok(id) => id,
                Err(_) => {
                    return Err(format!(
                        "Parsing id '{:?}' from fmi-file, which is not i64.",
                        params[0]
                    ))
                }
            };
            let lat = match params[2].parse::<f64>() {
                Ok(lat) => lat,
                Err(_) => {
                    return Err(format!(
                        "Parsing lat '{:?}' from fmi-file, which is not f64.",
                        params[2]
                    ))
                }
            };
            let lon = match params[3].parse::<f64>() {
                Ok(lon) => lon,
                Err(_) => {
                    return Err(format!(
                        "Parsing lon '{:?}' from fmi-file, which is not f64.",
                        params[3]
                    ))
                }
            };

            Ok(ProtoNode {
                id,
                coord: geo::Coordinate::from(lat, lon),
            })
        }
    }

    //--------------------------------------------------------------------------------------------//

    pub struct ProtoEdge {
        pub way_id: Option<i64>,
        pub src_id: i64,
        pub dst_id: i64,
        pub meters: Option<u32>,
        pub maxspeed: u16,
    }

    impl str::FromStr for ProtoEdge {
        type Err = String;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let params: Vec<&str> = line.split_whitespace().collect();

            let n = 5;
            if params.len() < n {
                return Err(format!(
                    "Not enough params for an edge ({}, but should be {}).",
                    params.len(),
                    n
                ));
            }

            let src_id = match params[0].parse::<i64>() {
                Ok(src_id) => src_id,
                Err(_) => {
                    return Err(format!(
                        "Parsing src-id '{:?}' from fmi-file, which is not i64.",
                        params[0]
                    ))
                }
            };
            let dst_id = match params[1].parse::<i64>() {
                Ok(dst_id) => dst_id,
                Err(_) => {
                    return Err(format!(
                        "Parsing dst-id '{:?}' from fmi-file, which is not i64.",
                        params[1]
                    ))
                }
            };
            let meters = match params[2] {
                "calc" => None,
                _ => match params[2].parse::<u32>() {
                    Ok(meters) => Some(meters),
                    Err(_) => {
                        return Err(format!(
                            "Parsing length '{}' of edge ({}->{}) didn't work.",
                            params[2], src_id, dst_id
                        ))
                    }
                },
            };
            let maxspeed = match params[4].parse::<u16>() {
                Ok(maxspeed) => maxspeed,
                Err(_) => {
                    return Err(format!(
                        "Parse maxspeed in km/h '{:?}' from fmi-file into u16.",
                        params[4]
                    ))
                }
            };

            Ok(ProtoEdge {
                way_id: None,
                src_id,
                dst_id,
                meters,
                maxspeed,
            })
        }
    }
}

//------------------------------------------------------------------------------------------------//

pub struct Parser;
impl Parser {
    fn is_line_functional(line: &String) -> bool {
        line != "" && line.chars().next() != Some('#')
    }
}
impl super::Parsing for Parser {
    fn parse_ways(file: File, graph_builder: &mut GraphBuilder) {
        info!("Starting edge-creation ..");
        for line in fmi::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            if let Ok(proto_edge) = line.parse::<fmi::ProtoEdge>() {
                graph_builder.push_edge(
                    proto_edge.way_id,
                    proto_edge.src_id,
                    proto_edge.dst_id,
                    proto_edge.meters,
                    proto_edge.maxspeed,
                );
            } else {
                if line.parse::<fmi::ProtoNode>().is_err() {
                    warn!("Could not parse line `{}`", line);
                }
            }
        }
        info!("Finished edge-creation");
    }

    fn parse_nodes(file: File, graph_builder: &mut GraphBuilder) {
        info!("Starting node-creation ..");
        for line in fmi::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            if let Ok(proto_node) = line.parse::<fmi::ProtoNode>() {
                graph_builder.push_node(proto_node.id, proto_node.coord);
            }
        }
        info!("Finished node-creation");
    }
}
