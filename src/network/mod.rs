mod building;
pub use building::{GraphBuilder, ProtoEdge, ProtoNode};
pub mod defaults;
pub mod geo;
pub use defaults::StreetType;

//------------------------------------------------------------------------------------------------//

use std::fmt;

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct Node {
    id: i64,
    coord: geo::Coordinate,
}
impl Node {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn coord(&self) -> &geo::Coordinate {
        &self.coord
    }
    pub fn lat(&self) -> f64 {
        self.coord.lat()
    }
    pub fn lon(&self) -> f64 {
        self.coord.lon()
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node: {{ id: {}, {} }}", self.id, self.coord,)
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct Edge {
    id: i64,
    src_idx: usize,
    dst_idx: usize,
    meters: u32,
    maxspeed: u16,
}
impl Edge {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn src_idx(&self) -> usize {
        self.src_idx
    }
    pub fn dst_idx(&self) -> usize {
        self.dst_idx
    }
    pub fn meters(&self) -> u32 {
        self.meters
    }
    pub fn maxspeed(&self) -> u16 {
        self.maxspeed
    }
    pub fn seconds(&self) -> u32 {
        // length [m] / velocity [km/h]
        ((self.meters / (self.maxspeed as u32)) as f64 * 3.6) as u32
    }
}
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge: {{ id: {}, ({})-{}->({}) }}",
            self.id, self.src_idx, self.meters, self.dst_idx,
        )
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    offsets: Vec<usize>,
}
impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            offsets: Vec::new(),
        }
    }

    //--------------------------------------------------------------------------------------------//
    // id <-> idx

    pub fn node_idx_from(&self, id: i64) -> Result<usize, usize> {
        match self.nodes.binary_search_by(|node| node.id.cmp(&id)) {
            Ok(idx) => Ok(idx),
            Err(idx) => Err(idx),
        }
    }

    //--------------------------------------------------------------------------------------------//
    // getter

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn node(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }

    pub fn edge(&self, src_idx: usize, dst_idx: usize) -> Option<&Edge> {
        match self.leaving_edges(src_idx) {
            Some(leaving_edges) => {
                match leaving_edges.binary_search_by(|edge| edge.dst_idx.cmp(&dst_idx)) {
                    Ok(j) => Some(&leaving_edges[j]),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }

    pub fn leaving_edges(&self, node_idx: usize) -> Option<&[Edge]> {
        // Use offset-array to get indices for the graph's edges belonging to the given node
        match self.offsets.get(node_idx) {
            // (idx + 1) guaranteed by offset-array-length
            Some(&i0) => match self.offsets.get(node_idx + 1) {
                Some(&i1) => {
                    // check if i0 and i1 are equal
                    // <-> if node has leaving edges
                    if i0 < i1 {
                        Some(&self.edges[i0..i1])
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }
}
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph: {{ number of nodes: {}, number of edges: {} }}",
            self.node_count(),
            self.edge_count()
        )?;

        writeln!(f, "")?;

        let n = 20;
        let m = 20;

        // print nodes
        for mut i in 0..n {
            // if enough nodes are in the graph
            if i < self.node_count() {
                if i == n - 1 {
                    // if at least 2 nodes are missing -> print `...`
                    if i + 1 < self.node_count() {
                        writeln!(f, "...")?;
                    }
                    // print last node
                    i = self.node_count() - 1;
                }
                let node = &self.nodes[i];
                writeln!(f, "Node: {{ idx: {}, id: {}, {} }}", i, node.id, node.coord,)?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        // print edges
        for mut j in 0..m {
            // if enough edges are in the graph
            if j < self.edge_count() {
                if j == m - 1 {
                    // if at least 2 edges are missing -> print `...`
                    if j + 1 < self.edge_count() {
                        writeln!(f, "...")?;
                    }
                    // print last edge
                    j = self.edge_count() - 1;
                }
                let edge = &self.edges[j];
                writeln!(
                    f,
                    "Edge: {{ idx: {}, id: {}, ({})-{}->({}) }}",
                    j,
                    edge.id,
                    self.node(edge.src_idx).id,
                    edge.meters,
                    self.node(edge.dst_idx).id,
                )?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        // print offsets
        for mut i in 0..n {
            // if enough offsets are in the graph
            if i < self.node_count() {
                if i == n - 1 {
                    // if at least 2 offsets are missing -> print `...`
                    if i + 1 < self.node_count() {
                        writeln!(f, "...")?;
                    }
                    // print last offset
                    i = self.node_count() - 1;
                }
                writeln!(f, "{{ id: {}, offset: {} }}", i, self.offsets[i])?;
            } else {
                break;
            }
        }
        // offset has n+1 entries due to `leaving_edges(...)`
        let i = self.offsets.len() - 1;
        writeln!(f, "{{ __: {}, offset: {} }}", i, self.offsets[i])?;

        Ok(())
    }
}
