use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::path::Path;

use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

//--------------------------------------------------------------------------------------------------

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        //------------------------------------------------------------------------------------------
        // get reader

        let path = Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        let mut reader = io::BufReader::new(file);

        //------------------------------------------------------------------------------------------
        // graph

        let mut node_count = None;
        let mut edge_count = None;
        let mut edge_id = 0;
        let mut graph_builder = GraphBuilder::new();

        //------------------------------------------------------------------------------------------
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

        let mut i = 0;
        for line in reader.by_ref().lines().map(Result::unwrap) {
            if line == "" || line.chars().next() == Some('#') {
                continue;
            }

            match i {
                // first functional line -> number of nodes
                0 => {
                    node_count = Some(line.parse::<usize>().expect(&format!(
                        "Parse node_count ({:?}) from fmi-file into usize.",
                        line
                    )));
                    i += 1;
                }
                // second functional line -> number of edges
                1 => {
                    edge_count = Some(line.parse::<usize>().expect(&format!(
                        "Parse edge_count ({:?}) from fmi-file into usize.",
                        line
                    )));
                    i += 1;
                    break;
                }
                _ => (),
            }
        }

        // set counts
        let node_count = match node_count {
            Some(c) => c,
            None => panic!("The given fmi-file misses the node-count."),
        };
        let edge_count = match edge_count {
            Some(c) => c,
            None => panic!("The given fmi-file misses the edge-count."),
        };
        graph_builder.reserve(node_count, edge_count);

        // loop over elements
        for line in reader.lines().map(Result::unwrap) {
            if line.trim() == "" || line.chars().next() == Some('#') {
                continue;
            }

            match i {
                // nodes
                _ if (1 < i) && (i < node_count + 2) => {
                    let line_string = line.split_whitespace();
                    let params: Vec<&str> = line_string.collect();
                    graph_builder.push_node(
                        params[0].parse::<usize>().expect(&format!(
                            "Parse id ({:?}) from fmi-file into usize.",
                            params[0]
                        )),
                        params[2].parse::<f64>().expect(&format!(
                            "Parse lat ({:?}) from fmi-file into f64.",
                            params[2]
                        )),
                        params[3].parse::<f64>().expect(&format!(
                            "Parse lon ({:?}) from fmi-file into f64.",
                            params[3]
                        )),
                    );
                }
                // edges
                _ if (node_count + 2 <= i) => {
                    let line_string = line.split_whitespace();
                    let params: Vec<&str> = line_string.collect();
                    graph_builder.push_edge(
                        edge_id,
                        params[0].parse::<usize>().expect(&format!(
                            "Parse src ({:?}) from fmi-file into usize.",
                            params[0]
                        )),
                        params[1].parse::<usize>().expect(&format!(
                            "Parse dst ({:?}) from fmi-file into usize.",
                            params[1]
                        )),
                        params[2].parse::<u64>().expect(&format!(
                            "Parse weight ({:?}) from fmi-file into u64.",
                            params[2]
                        )) * 1_000,
                    );
                    edge_id += 1;
                }
                _ => (),
            }
            i += 1;
        }

        graph_builder.finalize()
    }
}
