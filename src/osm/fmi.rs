use std::ffi::OsStr;
use std::fs::File;
use std::path;

use log::{info, warn};

use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

//------------------------------------------------------------------------------------------------//

use std::io::BufRead;
mod fmi {
    use log::warn;
    use std::str;

    use crate::osm::geo;

    pub use std::io::BufReader as Reader;

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

            let way_id = None;
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
            let meters = match params[2].parse::<u32>() {
                Ok(kilometers) => Some(kilometers * 1_000),
                Err(_) => match params[2].parse::<f64>() {
                    Ok(kilometers) => Some((kilometers * 1_000.0) as u32),
                    Err(_) => {
                        warn!(
                            "Parsing length '{}' of edge didn't work, \
                             so straight-line is taken.",
                            params[2]
                        );
                        None
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
                way_id,
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
    fn open_reader<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> fmi::Reader<File> {
        let path = path::Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        fmi::Reader::new(file)
    }

    fn is_line_functional(line: &String) -> bool {
        line != "" && line.chars().next() != Some('#')
    }

    fn parse_ways<S: AsRef<OsStr> + ?Sized>(&self, path: &S, graph_builder: &mut GraphBuilder) {
        for line in self
            .open_reader(&path)
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
    }

    fn parse_nodes<S: AsRef<OsStr> + ?Sized>(&self, path: &S, graph_builder: &mut GraphBuilder) {
        for line in self
            .open_reader(&path)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            if let Ok(proto_node) = line.parse::<fmi::ProtoNode>() {
                graph_builder.push_node(proto_node.id, proto_node.coord);
            }
        }
    }

    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        info!("Starting parsing ..");

        let mut graph_builder = GraphBuilder::new();

        info!("Starting processing given fmi-file ..");
        self.parse_ways(&path, &mut graph_builder);
        self.parse_nodes(&path, &mut graph_builder);
        info!("Finished processing given fmi-file");

        let graph = graph_builder.finalize();
        info!("Finished parsing");
        graph
    }
}
