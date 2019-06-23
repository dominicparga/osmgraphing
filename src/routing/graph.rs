use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

//--------------------------------------------------------------------------------------------------
// implementing graph

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_count: usize,
    pub edge_count: usize,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{{number of nodes: {}, number of edges: {}}}",
            self.node_count, self.edge_count
        )?;
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        for edge in &self.edges {
            writeln!(f, "{}", edge)?;
        }
        Ok(())
    }
}

impl Graph {
    //----------------------------------------------------------------------------------------------

    pub fn node_count(&self) -> usize {
        self.node_count
    }
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

impl Graph {
    //----------------------------------------------------------------------------------------------
    // edge offset

    pub fn set_edge_offset(&mut self) {
        let now = Instant::now();
        let l = self.node_count;
        let mut i = 0;
        let mut j = 0;
        for edge in self.edges.iter() {
            if edge.src == self.nodes[j].id {
                self.nodes[j].edge_end = i;
            } else {
                j += 1;
                while j < l && edge.src != self.nodes[j].id {
                    self.nodes[j].edge_start = i - 1;
                    self.nodes[j].edge_end = i - 1;
                    j += 1;
                }
                if j < l {
                    self.nodes[j].edge_start = i;
                    self.nodes[j].edge_end = i;
                }
            }
            i += 1;
        }
        println!(
            "Set offset in {} microseconds a.k.a. {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
    }

    //----------------------------------------------------------------------------------------------
    // reading from file

    pub fn read_graph<P: AsRef<Path>>(&mut self, path: P) -> Result<(), io::Error> {
        let now = Instant::now();
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

//--------------------------------------------------------------------------------------------------
// implementing nodes

pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
    pub edge_start: usize,
    pub edge_end: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, coord=({:.2}, {:.2}), edge_start: {}, edge_end: {} }}",
            self.id, self.lat, self.lon, self.edge_start, self.edge_end
        )
    }
}

//--------------------------------------------------------------------------------------------------
// implementing edges

pub struct Edge {
    pub id: usize,
    pub src: usize,
    pub dst: usize,
    pub weight: f64,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{id: {}, source: {}, target: {}, length: {}}}",
            self.id, self.src, self.dst, self.weight
        )
    }
}
