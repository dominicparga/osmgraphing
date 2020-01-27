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
        self.meters() * 3_600 / (self.maxspeed() as u32)
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
// graph: NodeContainer

#[derive(Debug)]
pub struct NodeContainer {
    nodes: Vec<Node>,
}

impl NodeContainer {
    fn new() -> NodeContainer {
        NodeContainer { nodes: Vec::new() }
    }

    pub fn count(&self) -> usize {
        self.nodes.len()
    }

    pub fn idx_from(&self, id: i64) -> Result<usize, usize> {
        self.nodes.binary_search_by(|node| node.id.cmp(&id))
    }

    pub fn get_from(&self, id: i64) -> Option<&Node> {
        let idx = match self.idx_from(id) {
            Ok(idx) => idx,
            Err(_) => return None,
        };
        self.get(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&Node> {
        debug_assert_eq!(
            self.nodes[idx].idx, idx,
            "Node's idx in graph and its stored idx should be same."
        );
        self.nodes.get(idx)
    }
}

impl ops::Index<usize> for NodeContainer {
    type Output = Node;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx]
    }
}

//------------------------------------------------------------------------------------------------//
// graph: EdgeContainer

#[derive(Debug)]
pub struct EdgeContainer {
    edges: Vec<Edge>,
    offsets: Vec<usize>,
}

impl EdgeContainer {
    fn new() -> EdgeContainer {
        EdgeContainer {
            edges: Vec::new(),
            offsets: Vec::new(),
        }
    }

    //--------------------------------------------------------------------------------------------//
    // getter: counts

    pub fn count(&self) -> usize {
        self.edges.len()
    }

    //--------------------------------------------------------------------------------------------//
    // getter: edges

    pub fn get(&self, idx: usize) -> Option<&Edge> {
        self.edges.get(idx)
    }

    pub fn starting_from(&self, node_idx: usize) -> Option<Vec<&Edge>> {
        let range = self.offset_indices(node_idx)?;

        let mut leaving_edges = vec![];
        for i in range {
            if let Some(edge) = self.get(i) {
                leaving_edges.push(edge);
            }
        }
        Some(leaving_edges)
    }

    /// uses binary-search, but only on src's leaving edges (±3), so more or less in O(1)
    ///
    /// Returns the index of the edge, which can be used in the function `edge`
    pub fn between(&self, src_idx: usize, dst_idx: usize) -> Option<(&Edge, usize)> {
        let range = self.offset_indices(src_idx)?;
        let leaving_edges = &self.edges[range.clone()];
        let j = leaving_edges
            .binary_search_by(|edge| edge.dst_idx.cmp(&dst_idx))
            .ok()?;

        let edge_idx = range.start + j;
        debug_assert_eq!(leaving_edges[j], self.edges[edge_idx]);
        let edge = self.get(edge_idx)?;

        Some((edge, edge_idx))
    }

    //--------------------------------------------------------------------------------------------//
    // getter: offsets

    /// Returns a "real" range, where `start_bound < end_bound`
    fn offset_indices(&self, node_idx: usize) -> Option<ops::Range<usize>> {
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
}

impl ops::Index<usize> for EdgeContainer {
    type Output = Edge;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.edges[idx]
    }
}

//------------------------------------------------------------------------------------------------//
// graph: EdgeContainer

#[derive(Debug)]
pub struct Graph {
    pub nodes: NodeContainer,
    pub fwd_edges: EdgeContainer,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: NodeContainer::new(),
            fwd_edges: EdgeContainer::new(),
        }
    }

    pub fn nodes(&self) -> &NodeContainer {
        &(self.nodes)
    }

    pub fn fwd_edges(&self) -> &EdgeContainer {
        &(self.fwd_edges)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph: {{ number of nodes: {}, number of fwd_edges: {} }}",
            self.nodes.count(),
            self.fwd_edges.count()
        )?;

        writeln!(f, "")?;

        let n = 20;
        let m = 20;

        // print nodes
        for mut i in 0..n {
            // if enough nodes are in the graph
            if i < self.nodes.count() {
                if i == n - 1 {
                    // if at least 2 nodes are missing -> print `...`
                    if i + 1 < self.nodes.count() {
                        writeln!(f, "...")?;
                    }
                    // print last node
                    i = self.nodes.count() - 1;
                }
                let node = &self.nodes[i];
                writeln!(f, "Node: {{ idx: {}, id: {}, {} }}", i, node.id, node.coord,)?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        // print fwd-edges
        for mut j in 0..m {
            // if enough edges are in the graph
            if j < self.fwd_edges.count() {
                if j == m - 1 {
                    // if at least 2 edges are missing -> print `...`
                    if j + 1 < self.fwd_edges.count() {
                        writeln!(f, "...")?;
                    }
                    // print last edge
                    j = self.fwd_edges.count() - 1;
                }
                let edge = &self.fwd_edges[j];
                writeln!(
                    f,
                    "Edge: {{ idx: {}, id: {}, ({})-{}->({}) }}",
                    j,
                    edge.id,
                    self.nodes[edge.src_idx].id,
                    edge.meters,
                    self.nodes[edge.dst_idx].id,
                )?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        // print fwd-offsets
        for mut i in 0..n {
            // if enough offsets are in the graph
            if i < self.nodes.count() {
                if i == n - 1 {
                    // if at least 2 offsets are missing -> print `...`
                    if i + 1 < self.nodes.count() {
                        writeln!(f, "...")?;
                    }
                    // print last offset
                    i = self.nodes.count() - 1;
                }
                writeln!(
                    f,
                    "{{ id: {}, fwd-offset: {} }}",
                    i, self.fwd_edges.offsets[i]
                )?;
            } else {
                break;
            }
        }
        // offset has n+1 entries due to `leaving_edges(...)`
        let i = self.fwd_edges.offsets.len() - 1;
        writeln!(
            f,
            "{{ __: {}, fwd-offset: {} }}",
            i, self.fwd_edges.offsets[i]
        )?;

        // print bwd-offsets
        // todo

        Ok(())
    }
}
