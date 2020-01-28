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
// indices

// todo

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
    src_idx: usize,
    dst_idx: usize,
    lane_count: u8,
    meters: u32,
    maxspeed: u16,
}

impl Edge {
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
        self.src_idx == other.src_idx
            && self.dst_idx == other.dst_idx
            && self.meters == other.meters
            && self.maxspeed == other.maxspeed
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge: {{ ({})-{}->({}) }}",
            self.src_idx, self.meters, self.dst_idx,
        )
    }
}

//------------------------------------------------------------------------------------------------//
// graph: NodeContainer

#[derive(Debug)]
pub struct NodeContainer<'a> {
    nodes: &'a Vec<Node>,
}

impl<'a> NodeContainer<'a> {
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

impl<'a> ops::Index<usize> for NodeContainer<'a> {
    type Output = Node;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx]
    }
}

//------------------------------------------------------------------------------------------------//
// graph: EdgeContainer

#[derive(Debug)]
pub struct EdgeContainer<'a> {
    graph: &'a Graph,
    edge_indices: &'a Vec<usize>,
    offsets: &'a Vec<usize>,
}

impl<'a> EdgeContainer<'a> {
    //--------------------------------------------------------------------------------------------//
    // getter: counts

    pub fn count(&self) -> usize {
        self.edge_indices.len()
    }

    //--------------------------------------------------------------------------------------------//
    // getter: edges

    pub fn get(&self, idx: usize) -> Option<&Edge> {
        // indirect mapping to safe memory
        let idx = *(self.edge_indices.get(idx)?);
        self.graph.edges.get(idx)
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

    /// uses linear-search, but only on src's leaving edges (±3), so more or less in O(1)
    ///
    /// Returns the index of the edge, which can be used in the function `edge(...)`
    pub fn between(&self, src_idx: usize, dst_idx: usize) -> Option<(&Edge, usize)> {
        // get offsets from offset-array for edge-indices (indirect mapping)
        let range = self.offset_indices(src_idx)?;
        let leaving_indices = &(self.edge_indices[range.clone()]);
        for &idx in leaving_indices {
            // indirect mapping
            let edge = self.get(idx)?;
            if edge.dst_idx == dst_idx {
                return Some((&edge, idx));
            }
        }
        return None;
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

impl<'a> ops::Index<usize> for EdgeContainer<'a> {
    type Output = Edge;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.get(idx).unwrap()
    }
}

//------------------------------------------------------------------------------------------------//
// graph: EdgeContainer

/// Stores nodes and edges and provides methods for accessing them.
///
/// Real edges, not their indices, are stored
///
/// - `(src-id, dst-id)` with `src-id` having precedence over `dst-id`
/// - in ascending order
#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    fwd_indices: Vec<usize>,
    fwd_offsets: Vec<usize>,
    bwd_indices: Vec<usize>,
    bwd_offsets: Vec<usize>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            fwd_indices: Vec::new(),
            fwd_offsets: Vec::new(),
            bwd_indices: Vec::new(),
            bwd_offsets: Vec::new(),
        }
    }

    pub fn nodes<'a>(&'a self) -> NodeContainer<'a> {
        NodeContainer {
            nodes: &(self.nodes),
        }
    }

    pub fn fwd_edges<'a>(&'a self) -> EdgeContainer<'a> {
        EdgeContainer {
            graph: self,
            edge_indices: &(self.fwd_indices),
            offsets: &(self.fwd_offsets),
        }
    }

    pub fn bwd_edges<'a>(&'a self) -> EdgeContainer<'a> {
        EdgeContainer {
            graph: self,
            edge_indices: &(self.bwd_indices),
            offsets: &(self.bwd_offsets),
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph: {{ number of nodes: {}, number of fwd_edges: {} }}",
            self.nodes().count(),
            self.fwd_edges().count()
        )?;

        writeln!(f, "")?;

        let n = 20;
        let m = 20;

        // print nodes
        for mut i in 0..n {
            // if enough nodes are in the graph
            if i < self.nodes().count() {
                if i == n - 1 {
                    // if at least 2 nodes are missing -> print `...`
                    if i + 1 < self.nodes().count() {
                        writeln!(f, "...")?;
                    }
                    // print last node
                    i = self.nodes().count() - 1;
                }
                let node = &self.nodes()[i];
                writeln!(f, "Node: {{ idx: {}, id: {}, {} }}", i, node.id, node.coord,)?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        for (xwd_edges, xwd_offsets, xwd_prefix) in vec![
            (self.fwd_edges(), &(self.fwd_offsets), "fwd-"),
            (self.bwd_edges(), &(self.bwd_offsets), "bwd-"),
        ] {
            // print xwd-edges
            for mut j in 0..m {
                // if enough edges are in the graph
                if j < xwd_edges.count() {
                    if j == m - 1 {
                        // if at least 2 edges are missing -> print `...`
                        if j + 1 < xwd_edges.count() {
                            writeln!(f, "...")?;
                        }
                        // print last edge
                        j = xwd_edges.count() - 1;
                    }
                    let edge = &xwd_edges[j];
                    writeln!(
                        f,
                        "{}edge: {{ idx: {}, ({})-{}->({}) }}",
                        xwd_prefix,
                        j,
                        self.nodes[edge.src_idx].id,
                        edge.meters,
                        self.nodes[edge.dst_idx].id,
                    )?;
                } else {
                    break;
                }
            }

            writeln!(f, "")?;

            // print xwd-offsets
            for mut i in 0..n {
                // if enough offsets are in the graph
                if i < self.nodes().count() {
                    if i == n - 1 {
                        // if at least 2 offsets are missing -> print `...`
                        if i + 1 < self.nodes().count() {
                            writeln!(f, "...")?;
                        }
                        // print last offset
                        i = self.nodes().count() - 1;
                    }
                    writeln!(
                        f,
                        "{{ id: {}, {}offset: {} }}",
                        i, xwd_prefix, xwd_offsets[i]
                    )?;
                } else {
                    break;
                }
            }
            // offset has n+1 entries due to `leaving_edges(...)`
            let i = xwd_offsets.len() - 1;
            writeln!(
                f,
                "{{ __: {}, {}offset: {} }}",
                i, xwd_prefix, xwd_offsets[i]
            )?;
        }

        Ok(())
    }
}
