use std::fmt;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_count : usize,
    pub edge_count: usize
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{number of nodes: {}, number of edges: {}}}", self.node_count, self.edge_count);
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
    pub edge_start: usize,
    pub edge_end: usize
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{ id: {}, coord=({:.2}, {:.2}) }}", self.id, self.lat, self.lon);
    }
}

pub struct Edge {
    pub id: usize,
    pub src: usize,
    pub dest: usize,
    pub weight: usize
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{id: {}, source: {}, target: {}, length: {}}}", self.id, self. src, self.dest, self.weight);
    }
}
