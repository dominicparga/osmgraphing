//----------------------------------------------------------------------------------------------
// TODO unused and broken
//----------------------------------------------------------------------------------------------

use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::Path;

//--------------------------------------------------------------------------------------------------

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> io::Result<()> {
        let file = File::open(&path)?;
        let mut n_nodes;
        let mut n_edges;
        let mut i = 0;
        let reader = io::BufReader::new(file);

        let mut hax = 0;
        for line in reader.lines() {
            let curr_line = line.unwrap();
            if curr_line == "" || curr_line.chars().next().unwrap() == '#' {
                continue;
            }
            match i {
                0 => {
                    n_nodes = curr_line.parse::<usize>().unwrap();
                    self.nodes.reserve(n_nodes);
                    self.node_count = n_nodes;
                }
                1 => {
                    n_edges = curr_line.parse::<usize>().unwrap();
                    self.edges.reserve(n_edges);
                    self.edge_count = n_edges;
                }
                j if j > 1 && j <= self.node_count + 1 => {
                    let line_string = curr_line.split_whitespace();
                    let param: Vec<&str> = line_string.collect();
                    self.nodes.push(Node {
                        id: param[0].parse::<usize>().unwrap(),
                        lat: param[2].parse::<f64>().unwrap(),
                        lon: param[3].parse::<f64>().unwrap(),
                        edge_start: 0,
                        edge_end: 0,
                    });
                }
                j if j > self.node_count + 1 => {
                    let line_string = curr_line.split_whitespace();
                    let param: Vec<&str> = line_string.collect();
                    self.edges.push(Edge {
                        id: hax,
                        src: param[0].parse::<usize>().unwrap(),
                        dst: param[1].parse::<usize>().unwrap(),
                        weight: param[2].parse::<f64>().unwrap(),
                    });
                    hax += 1;
                }

                _ => {}
            }
            i += 1;
        }
        println!(
            "Read graph in {} microseconds a.k.a. {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        Ok(())
    }
}
