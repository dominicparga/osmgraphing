use crate::network::GraphBuilder;
use log::{info, warn};
use std::{fs::File, io::BufRead};

pub struct Parser;
impl Parser {
    fn is_line_functional(line: &String) -> bool {
        line != "" && line.chars().next() != Some('#')
    }
}
impl super::Parsing for Parser {
    fn parse_ways(file: File, graph_builder: &mut GraphBuilder) {
        info!("START Create edges from input-file.");
        for line in intern::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            if let Ok(proto_edge) = line.parse::<intern::ProtoEdge>() {
                graph_builder.push_edge(
                    proto_edge.src_id,
                    proto_edge.dst_id,
                    proto_edge.meters,
                    proto_edge.maxspeed,
                    None, // TODO metrics_u8
                );
            } else {
                if line.parse::<intern::ProtoNode>().is_err() {
                    warn!("Could not parse line `{}`", line);
                }
            }
        }
        info!("FINISHED");
    }

    fn parse_nodes(file: File, graph_builder: &mut GraphBuilder) {
        info!("START Create nodes from input-file.");
        for line in intern::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            if let Ok(proto_node) = line.parse::<intern::ProtoNode>() {
                graph_builder.push_node(proto_node.id, proto_node.coord);
            }
        }
        info!("FINISHED");
    }
}

mod intern {
    use crate::units::{geo, length::Meters, speed::KilometersPerHour};
    pub use std::{io::BufReader as Reader, str};

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
                coord: geo::Coordinate::from_f64(lat, lon),
            })
        }
    }

    //--------------------------------------------------------------------------------------------//

    pub struct ProtoEdge {
        pub src_id: i64,
        pub dst_id: i64,
        pub meters: Option<Meters>,
        pub maxspeed: KilometersPerHour,
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
                    Ok(meters) => Some(Meters::new(meters)),
                    Err(_) => {
                        return Err(format!(
                            "Parsing length '{}' of edge ({}->{}) didn't work.",
                            params[2], src_id, dst_id
                        ))
                    }
                },
            };
            let maxspeed = match params[4].parse::<u16>() {
                Ok(maxspeed) => KilometersPerHour::new(maxspeed),
                Err(_) => {
                    return Err(format!(
                        "Parse maxspeed in km/h '{:?}' from fmi-file into u16.",
                        params[4]
                    ))
                }
            };

            Ok(ProtoEdge {
                src_id,
                dst_id,
                meters,
                maxspeed,
            })
        }
    }
}
