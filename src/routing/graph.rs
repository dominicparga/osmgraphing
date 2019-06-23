use std::fmt;
use std::time::Instant;

//--------------------------------------------------------------------------------------------------

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_count: usize,
    pub edge_count: usize,
}

pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
    pub edge_start: usize,
    pub edge_end: usize,
}

pub struct Edge {
    pub id: usize,
    pub src: usize,
    pub dst: usize,
    pub weight: f64,
}

//--------------------------------------------------------------------------------------------------

impl Graph {
    //----------------------------------------------------------------------------------------------

    pub fn node_count(&self) -> usize {
        self.node_count
    }
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

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

}


//--------------------------------------------------------------------------------------------------
// fmt::Display

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, coord=({:.2}, {:.2}), edge_start: {}, edge_end: {} }}",
            self.id, self.lat, self.lon, self.edge_start, self.edge_end
        )
    }
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
