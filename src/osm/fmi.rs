use std::ffi::OsStr;
use std::fs::File;
use std::path;

use log::{info, warn};

use crate::osm::geo;
use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

//------------------------------------------------------------------------------------------------//

use std::io::BufRead;
mod fmi {
    pub use std::io::BufReader as Reader;
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
            let params: Vec<&str> = line.split_whitespace().collect();

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
    }

    fn parse_nodes<S: AsRef<OsStr> + ?Sized>(&self, path: &S, graph_builder: &mut GraphBuilder) {
        for line in self
            .open_reader(&path)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            let params: Vec<&str> = line.split_whitespace().collect();

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
