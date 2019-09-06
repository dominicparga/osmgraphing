use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::path::Path;

use log::{info, warn};

use crate::osm::geo;
use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

//------------------------------------------------------------------------------------------------//

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        info!("Starting parsing ..");

        //----------------------------------------------------------------------------------------//
        // get reader

        let path = Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        let mut reader = io::BufReader::new(file);

        //----------------------------------------------------------------------------------------//
        // graph

        let mut node_count = None;
        let mut edge_count = None;
        let mut graph_builder = GraphBuilder::new();

        //----------------------------------------------------------------------------------------//
        // Parsing a `.fmi`-file of following structure, where empty lines and comment lines (#) can
        // be addes everywhere.
        //
        // # This is a comment line.
        //
        // # node_count
        // 42
        // # edge_count
        // 16
        //
        // # 42 nodes of structure
        // # id osm-id(ignored) lat lon
        // ...
        //
        // # 16 edges of structure
        // # src dst distance ??? maxspeed
        // ...

        info!("Starting processing given fmi-file ..");
        let mut i = 0;
        for line in reader.by_ref().lines().map(Result::unwrap) {
            if line == "" || line.chars().next() == Some('#') {
                continue;
            }

            // first functional line -> number of nodes
            if i == 0 {
                node_count = Some(line.parse::<usize>().expect(&format!(
                    "Parse node_count ({:?}) from fmi-file into usize.",
                    line
                )));
                i += 1;
            }
            // second functional line -> number of edges
            else if i == 1 {
                edge_count = Some(line.parse::<usize>().expect(&format!(
                    "Parse edge_count ({:?}) from fmi-file into usize.",
                    line
                )));
                i += 1;
                break;
            }
        }

        // set counts
        let node_count = match node_count {
            Some(c) => c,
            None => panic!("The given fmi-file misses the node-count."),
        };
        let _edge_count = match edge_count {
            Some(c) => c,
            None => panic!("The given fmi-file misses the edge-count."),
        };

        // loop over elements
        for line in reader.lines().map(Result::unwrap) {
            if line.trim() == "" || line.chars().next() == Some('#') {
                continue;
            }

            // nodes
            if (1 < i) && (i < node_count + 2) {
                let line_string = line.split_whitespace();
                let params: Vec<&str> = line_string.collect();

                // parse file
                let id = params[0].parse::<i64>().expect(&format!(
                    "Parsing id '{:?}' from fmi-file, which is not i64.",
                    params[0]
                ));
                let lat = params[2].parse::<f64>().expect(&format!(
                    "Parsing lat '{:?}' from fmi-file, which is not f64.",
                    params[2]
                ));
                let lon = params[3].parse::<f64>().expect(&format!(
                    "Parsing lon '{:?}' from fmi-file, which is not f64.",
                    params[3]
                ));
                graph_builder.push_node(id, geo::Coordinate::from(lat, lon));
            }
            // edges
            else if node_count + 2 <= i {
                let line_string = line.split_whitespace();
                let params: Vec<&str> = line_string.collect();

                // parse file
                let way_id = None;
                let src_id = params[0].parse::<i64>().expect(&format!(
                    "Parsing src-id '{:?}' from fmi-file, which is not i64.",
                    params[0]
                ));
                let dst_id = params[1].parse::<i64>().expect(&format!(
                    "Parsing dst-id '{:?}' from fmi-file, which is not i64.",
                    params[1]
                ));
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
                let maxspeed = params[4].parse::<u16>().expect(&format!(
                    "Parse maxspeed in km/h '{:?}' from fmi-file into u16.",
                    params[4]
                ));
                graph_builder.push_edge(way_id, src_id, dst_id, meters, maxspeed);
            }
            i += 1;
        }
        info!("Finished processing given fmi-file.");

        graph_builder.finalize()
    }
}
