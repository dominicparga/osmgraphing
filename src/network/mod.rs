//------------------------------------------------------------------------------------------------//
// other modules

use std::fmt;
use std::ops;

//------------------------------------------------------------------------------------------------//
// own modules

mod building;
pub use building::{GraphBuilder, ProtoEdge, ProtoNode};
pub mod defaults;
pub mod geo;
pub use defaults::StreetType;

//------------------------------------------------------------------------------------------------//
// node

#[derive(Debug)]
pub struct Node {
    id: i64,
    idx: usize,
    coord: geo::Coordinate,
}
impl Node {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn idx(&self) -> usize {
        self.idx
    }
    pub fn coord(&self) -> &geo::Coordinate {
        &self.coord
    }
}
impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id && self.idx == other.idx && self.coord == other.coord
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node: {{ id: {}, idx: {}, {} }}",
            self.id, self.idx, self.coord,
        )
    }
}

//------------------------------------------------------------------------------------------------//
// edge

#[derive(Debug)]
pub struct Edge {
    id: i64,
    src_idx: usize,
    dst_idx: usize,
    lane_count: u8,
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
    pub fn lane_count(&self) -> u8 {
        debug_assert!(self.lane_count > 0, "Edge-lane-count should be > 0");
        self.lane_count
    }
    pub fn meters(&self) -> u32 {
        debug_assert!(self.meters > 0, "Edge-length should be > 0");
        self.meters
    }
    pub fn maxspeed(&self) -> u16 {
        debug_assert!(self.maxspeed > 0, "Edge-maxspeed should be > 0");
        self.maxspeed
    }
    pub fn milliseconds(&self) -> u32 {
        // length [m] / velocity [km/h]
        self.meters * 3_600 / (self.maxspeed as u32)
    }
}
impl Eq for Edge {}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.id == other.id
            && self.src_idx == other.src_idx
            && self.dst_idx == other.dst_idx
            && self.meters == other.meters
            && self.maxspeed == other.maxspeed
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
// graph

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
        self.nodes.binary_search_by(|node| node.id.cmp(&id))
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
        debug_assert_eq!(
            self.nodes[idx].idx, idx,
            "Node's idx in graph and its stored idx should be same."
        );
        &self.nodes[idx]
    }

    pub fn edge(&self, edge_idx: usize) -> &Edge {
        &self.edges[edge_idx]
    }

    pub fn offset(&self, node_idx: usize) -> usize {
        self.offsets[node_idx]
    }

    /// uses binary-search, but only on src's leaving edges (±3), so more or less in O(1)
    ///
    /// Returns the index of the edge, which can be used in the function `edge`
    pub fn edge_from(&self, src_idx: usize, dst_idx: usize) -> Option<(&Edge, usize)> {
        let range = self.offset_indices(src_idx)?;
        let leaving_edges = &self.edges[range.clone()];
        let j = leaving_edges
            .binary_search_by(|edge| edge.dst_idx.cmp(&dst_idx))
            .ok()?;
        let edge_idx = range.start + j;
        debug_assert_eq!(leaving_edges[j], self.edges[edge_idx]);
        Some((&leaving_edges[j], edge_idx))
    }

    pub fn offset_indices(&self, node_idx: usize) -> Option<ops::Range<usize>> {
        // Use offset-array to get indices for the graph's edges belonging to the given node
        let &i0 = self.offsets.get(node_idx)?;
        // (idx + 1) guaranteed by offset-array-length
        let &i1 = self.offsets.get(node_idx + 1)?;

        // check if i0 and i1 are equal
        // <-> if node has leaving edges
        if i0 < i1 {
            Some(i0..i1)
        } else {
            None
        }
    }

    pub fn leaving_edges(&self, node_idx: usize) -> Option<&[Edge]> {
        let range = self.offset_indices(node_idx)?;
        Some(&self.edges[range])
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
