use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use log::{error, info};

use crate::osm::geo;

//------------------------------------------------------------------------------------------------//
// graphbuilder

struct ProtoNode {
    id: i64,
    coord: Option<geo::Coordinate>,
    is_edge_node: bool,
}

impl Ord for ProtoNode {
    fn cmp(&self, other: &ProtoNode) -> Ordering {
        // inverse order since BinaryHeap is max-heap, but min-heap is needed
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for ProtoNode {
    fn partial_cmp(&self, other: &ProtoNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ProtoNode {}

impl PartialEq for ProtoNode {
    fn eq(&self, other: &ProtoNode) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

struct ProtoEdge {
    way_id: Option<i64>,
    src_id: i64,
    dst_id: i64,
    meters: Option<u32>,
    maxspeed: u16,
}

pub struct GraphBuilder {
    proto_nodes: BTreeMap<i64, ProtoNode>,
    proto_edges: Vec<ProtoEdge>,
}

impl GraphBuilder {
    //--------------------------------------------------------------------------------------------//
    // init self

    pub fn new() -> Self {
        GraphBuilder {
            proto_nodes: BTreeMap::new(),
            proto_edges: Vec::new(),
        }
    }

    pub fn push_node(&mut self, id: i64, coord: geo::Coordinate) -> &mut Self {
        // if already added -> update coord
        // if not -> add new node
        if let Some(proto_node) = self.proto_nodes.get_mut(&id) {
            proto_node.coord = Some(coord);
        } else {
            self.proto_nodes.insert(
                id,
                ProtoNode {
                    id,
                    coord: Some(coord),
                    is_edge_node: false,
                },
            );
        }
        self
    }

    pub fn push_edge(
        &mut self,
        way_id: Option<i64>,
        src_id: i64,
        dst_id: i64,
        meters: Option<u32>,
        maxspeed: u16,
    ) -> &mut Self {
        // add edge
        self.proto_edges.push(ProtoEdge {
            way_id,
            src_id,
            dst_id,
            meters,
            maxspeed,
        });

        // add or update src-node
        if let Some(proto_node) = self.proto_nodes.get_mut(&src_id) {
            proto_node.is_edge_node = true;
        } else {
            self.proto_nodes.insert(
                src_id,
                ProtoNode {
                    id: src_id,
                    coord: None,
                    is_edge_node: true,
                },
            );
        }

        // add or update dst-node
        if let Some(proto_node) = self.proto_nodes.get_mut(&dst_id) {
            proto_node.is_edge_node = true;
        } else {
            self.proto_nodes.insert(
                dst_id,
                ProtoNode {
                    id: dst_id,
                    coord: None,
                    is_edge_node: true,
                },
            );
        }

        self
    }

    pub fn finalize(mut self) -> Graph {
        //----------------------------------------------------------------------------------------//
        // init graph

        let node_count = self.proto_nodes.len();
        let edge_count = self.proto_edges.len();
        info!(
            "Starting finalizing graph ({} proto-nodes and {} proto-edges) ..",
            node_count, edge_count
        );
        let mut graph = Graph::new();

        //----------------------------------------------------------------------------------------//
        // add nodes to graph which belong to edges (sorted by asc id)

        info!("Starting adding nodes (sorted) which belongs to an edge ..");
        // BTreeMap's iter returns sorted by key (asc)
        for (_id, proto_node) in self.proto_nodes.iter() {
            // add nodes only if they belong to an edge
            if !proto_node.is_edge_node {
                continue;
            }

            // add new node
            if let Some(coord) = proto_node.coord {
                graph.nodes.push(Node {
                    id: proto_node.id,
                    coord,
                });
            } else {
                // should not happen if file is okay
                error!(
                    "Proto-node (id: {}) has no coordinates, but belongs to an edge",
                    proto_node.id
                );
            }
        }
        info!("Finished adding nodes");

        //----------------------------------------------------------------------------------------//
        // sort edges by ascending src-id, then by ascending dst-id -> offset-array
        // then give edges IDs

        info!("Starting sorting proto-edges by their src/dst-IDs ..");
        self.proto_edges.sort_by(|e0, e1| {
            e0.src_id
                .cmp(&e1.src_id)
                .then_with(|| e0.dst_id.cmp(&e1.dst_id))
        });
        info!("Finished sorting proto-edges");

        //----------------------------------------------------------------------------------------//
        // build offset-array and edges

        info!("Starting creating the offset-array ..");
        let mut node_idx = 0;
        let mut offset = 0;
        graph.offsets.push(offset);
        // high-level-idea: count offset for each proto_edge and apply if src changes
        for edge_idx in 0..edge_count {
            let proto_edge = &self.proto_edges[edge_idx];
            // set way-id to index
            let edge_way_id = match proto_edge.way_id {
                Some(id) => id,
                None => edge_idx as i64,
            };

            // find source-index in sorted vec of nodes
            let src_idx = match graph.node_idx_from(proto_edge.src_id) {
                Ok(idx) => idx,
                Err(_) => panic!(
                    "The given source-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                    proto_edge.src_id, proto_edge.way_id
                ),
            };

            // find destination-index in sorted vec of nodes
            let dst_idx = match graph.node_idx_from(proto_edge.dst_id) {
                Ok(idx) => idx,
                Err(_) => panic!(
                    "The given destination-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                    proto_edge.dst_id, proto_edge.way_id
                ),
            };

            // calculate distance if not provided
            let meters = match proto_edge.meters {
                Some(meters) => meters,
                None => {
                    let src = graph.node(src_idx);
                    let dst = graph.node(dst_idx);
                    (geo::haversine_distance(&src.coord, &dst.coord) * 1_000.0) as u32
                }
            };

            // add new edge to graph
            let edge = Edge {
                id: edge_way_id,
                src_idx,
                dst_idx,
                meters,
                maxspeed: proto_edge.maxspeed,
            };

            // if coming edges have new src
            // then update offset of new src
            while node_idx != src_idx {
                node_idx += 1;
                graph.offsets.push(offset);
            }
            graph.edges.push(edge);
            offset += 1;
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        graph.offsets.push(offset);
        info!("Finished creating offset-array");

        graph
    }
}

//------------------------------------------------------------------------------------------------//
// original graph

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
}

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

//------------------------------------------------------------------------------------------------//
// fmt::Display

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node: {{ id: {}, {} }}", self.id, self.coord,)
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
