use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::err;
use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

//--------------------------------------------------------------------------------------------------

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Result<Graph, err::ParseError> {
        //------------------------------------------------------------------------------------------
        // get reader

        let path = Path::new(&path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        //------------------------------------------------------------------------------------------
        // graph

        let mut node_count;
        let mut edge_count;
        let mut edge_id = 0;
        let mut graph_builder = GraphBuilder::new();

        //------------------------------------------------------------------------------------------
        // parse

        let mut i = 0;
        for line in reader.lines().map(Result::unwrap) {
            if line.trim() == "" || line.chars().next().unwrap() == '#' {
                continue;
            }

            match i {
                // first functional line -> number of nodes
                0 => {
                    node_count = line.parse::<usize>().unwrap();
                    graph_builder.reserve_nodes(node_count);
                }
                // second functional line -> number of edges
                1 => {
                    edge_count = line.parse::<usize>().unwrap();
                    graph_builder.reserve_edges(edge_count);
                }
                // nodes
                _ if (1 < i) && (i < node_count + 2) => {
                    let line_string = line.split_whitespace();
                    let params: Vec<&str> = line_string.collect();
                    graph_builder.push_node(
                        params[0].parse::<usize>()?, // id
                        params[2].parse::<f64>()?,   // lat
                        params[3].parse::<f64>()?,   // lon
                    );
                }
                // edges
                _ if (node_count + 2 <= i) => {
                    let line_string = line.split_whitespace();
                    let params: Vec<&str> = line_string.collect();
                    graph_builder.push_edge(
                        edge_id,                     // id
                        params[0].parse::<usize>()?, // src
                        params[1].parse::<usize>()?, // dst
                        params[2].parse::<f64>()?,   // weight
                    );
                    edge_id += 1;
                }
                _ => (),
            }
            i += 1;
        }

        Ok(graph_builder.finalize())
    }
}
