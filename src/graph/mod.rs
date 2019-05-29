use std::fmt;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
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
}
