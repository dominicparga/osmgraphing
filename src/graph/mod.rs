use std::fmt;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph {
    pub number_of_nodes: usize,
    pub number_of_edges: usize,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{number of nodes: {}, number of edges: {}}}", self.number_of_nodes, self.number_of_edges);
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        for edge in &self.edges {
            writeln!(f, "{}", edge)?;
        }
        return Ok(());
    }
}

pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{ id: {}, coord=({:.2}, {:.2}) }}", self.id, self.lat, self.lon);
    }
}

pub struct Edge {
    pub id: usize,
    pub src: usize,
    pub dst: usize,
    pub length: usize,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{id: {}, source: {}, target: {}, length: {}}}", self.id, self. src, self.dst, self.length);
    }
}

pub trait Read {
    fn read_graph(&mut self, file_name: &str) -> Result<(), io::Error>;
}

impl Read for Graph {
    fn read_graph(&mut self, file_name: &str) -> Result<(), io::Error> {
        let file = File::open(file_name)?;
        let mut n_nodes;
        let mut n_edges;
        let mut i = 0;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let curr_line = line.unwrap();
            if curr_line.chars().next().unwrap() == '#' {
                continue;
            }
            match i {
                0 => {
                    n_nodes = curr_line.parse::<usize>().unwrap();
                    self.nodes.reserve(n_nodes);
                    self.number_of_nodes = n_nodes;
                },
                1 => {
                    n_edges = curr_line.parse::<usize>().unwrap();
                    self.edges.reserve(n_edges);
                    self.number_of_edges = n_edges;
                },
                j if j > 1 && j <= self.number_of_nodes+1 => {
                    let line_string = curr_line.split_whitespace();
                    let param: Vec<&str> = line_string.collect(); 
                    self.nodes.push(Node {
                        id: param[0].parse::<usize>().unwrap(),
                        lat: param[1].parse::<f64>().unwrap(),
                        lon: param[2].parse::<f64>().unwrap()
                    });
                },
                j if j > self.number_of_nodes+1 => {
                    let line_string = curr_line.split_whitespace();
                    let param: Vec<&str> = line_string.collect(); 
                    self.edges.push(Edge {
                        id: param[0].parse::<usize>().unwrap(),
                        src: param[1].parse::<usize>().unwrap(),
                        dst: param[2].parse::<usize>().unwrap(),
                        length: param[3].parse::<usize>().unwrap()
                    });
                }

                _ => {}
            }
            i += 1;
        }
        Ok(())
    }
}
