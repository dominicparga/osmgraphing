use std::fmt;

use crate::osm::geo;

//--------------------------------------------------------------------------------------------------
// graphbuilder

struct ProtoNode {
    id: Option<i64>,
    coord: geo::Coordinate,
}

struct ProtoEdge {
    id: Option<i64>,
    src_id: i64,
    dst_id: i64,
    meters: Option<u64>,
    maxspeed: u16,
}

pub struct GraphBuilder {
    nodes: Vec<ProtoNode>,
    edges: Vec<ProtoEdge>,
}
impl GraphBuilder {
    //----------------------------------------------------------------------------------------------
    // init self

    pub fn new() -> Self {
        GraphBuilder {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        GraphBuilder {
            nodes: Vec::with_capacity(node_capacity),
            edges: Vec::with_capacity(edge_capacity),
        }
    }

    //----------------------------------------------------------------------------------------------
    // build graph

    pub fn reserve(&mut self, additional_nodes: usize, additional_edges: usize) -> &mut Self {
        self.reserve_nodes(additional_nodes)
            .reserve_edges(additional_edges)
    }

    pub fn reserve_nodes(&mut self, additional: usize) -> &mut Self {
        self.nodes.reserve(additional);
        self
    }

    pub fn reserve_edges(&mut self, additional: usize) -> &mut Self {
        self.edges.reserve(additional);
        self
    }

    pub fn push_node(&mut self, id: Option<i64>, coord: geo::Coordinate) -> &mut Self {
        self.nodes.push(ProtoNode { id, coord });
        self
    }

    pub fn push_edge(
        &mut self,
        id: Option<i64>,
        src_id: i64,
        dst_id: i64,
        meters: Option<u64>,
        maxspeed: u16,
    ) -> &mut Self {
        self.edges.push(ProtoEdge {
            id,
            src_id,
            dst_id,
            meters,
            maxspeed,
        });
        self
    }

    pub fn finalize(mut self) -> Graph {
        //------------------------------------------------------------------------------------------
        // init graph and reserve capacity for (hopefully) better performance

        let node_count = self.nodes.len();
        let edge_count = self.edges.len();
        info!(
            "Start finalizing graph ({} nodes and {} edges)..",
            node_count, edge_count
        );
        let mut graph = Graph::new();
        graph.edges.reserve(edge_count);
        // offsets -> n+1 due to method `leaving_edges(...)`
        graph.offsets.reserve(node_count + 1);

        //------------------------------------------------------------------------------------------
        // apply IDs if given one is None (TODO check for uniqueness)
        // sort nodes by ascending id

        info!("Start sorting nodes by ID..");
        let mut i = -1;
        graph.nodes = self
            .nodes
            .iter()
            .map(|proto_node| Node {
                id: match proto_node.id {
                    Some(id) => id,
                    None => {
                        i += 1;
                        i
                    }
                },
                coord: proto_node.coord,
            })
            .collect();
        graph.nodes.sort_by(|n0, n1| n0.id.cmp(&n1.id));
        info!("Finished sorting nodes.");

        //------------------------------------------------------------------------------------------
        // sort edges by ascending src-id, then by ascending dst-id -> offset-array
        // then give edges IDs

        info!("Start sorting edges by their src/dst-IDs..");
        self.edges.sort_by(|e0, e1| {
            e0.src_id
                .cmp(&e1.src_id)
                .then_with(|| e0.dst_id.cmp(&e1.dst_id))
        });
        info!("Finished sorting edges.");

        //------------------------------------------------------------------------------------------
        // build offset-array and edges

        info!("Start creating the offset-array..");
        let mut node_idx = 0;
        let mut offset = 0;
        graph.offsets.push(offset);
        // high-level-idea: count offset for each proto_edge and apply if src changes
        for edge_idx in 0..edge_count {
            let proto_edge = &self.edges[edge_idx];
            // set id to index - TODO: uniqueness not guaranteed if only some (small) IDs are given
            let edge_id = match proto_edge.id {
                Some(id) => id,
                None => edge_idx as i64,
            };

            // find source-index in sorted vec of nodes
            let src_idx = match graph.node_idx_from(proto_edge.src_id) {
                Ok(idx) => idx,
                Err(_) => panic!(
                    "The given source-id `{:?}` of edge-id `{:?}` doesn't exist as node.",
                    proto_edge.src_id, proto_edge.id
                ),
            };

            // find destination-index in sorted vec of nodes
            let dst_idx = match graph.node_idx_from(proto_edge.dst_id) {
                Ok(idx) => idx,
                Err(_) => panic!(
                    "The given destination-id `{:?}` of edge-id `{:?}` doesn't exist as node.",
                    proto_edge.dst_id, proto_edge.id
                ),
            };

            // calculate distance if not provided
            let meters = match proto_edge.meters {
                Some(meters) => meters,
                None => {
                    let src = graph.node(src_idx);
                    let dst = graph.node(dst_idx);
                    (geo::haversine_distance(&src.coord, &dst.coord) * 1_000.0) as u64
                }
            };

            // add new edge to graph
            let edge = Edge {
                id: edge_id,
                src_idx,
                dst_idx,
                meters,
                maxspeed: proto_edge.maxspeed,
            };

            // if coming edges have new src
            // then update offset of new src
            if node_idx != src_idx {
                node_idx += 1;
                graph.offsets.push(offset);
            }
            graph.edges.push(edge);
            offset += 1;
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        graph.offsets.push(offset);
        info!("Finished creating offset-array.");

        graph
    }
}

//--------------------------------------------------------------------------------------------------
// original graph

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

pub struct Edge {
    id: i64,
    src_idx: usize,
    dst_idx: usize,
    meters: u64,
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
    pub fn meters(&self) -> u64 {
        self.meters
    }
    pub fn maxspeed(&self) -> u16 {
        self.maxspeed
    }
}

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

    //----------------------------------------------------------------------------------------------
    // id <-> idx

    pub fn node_idx_from(&self, id: i64) -> Result<usize, usize> {
        match self.nodes.binary_search_by(|node| node.id.cmp(&id)) {
            Ok(idx) => Ok(idx),
            Err(idx) => Err(idx),
        }
    }

    //----------------------------------------------------------------------------------------------
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

    pub fn edge(&self, src_idx: usize, dst_idx: usize) -> &Edge {
        let edges = self.leaving_edges(src_idx);
        let j = match edges.binary_search_by(|edge| edge.dst_idx.cmp(&dst_idx)) {
            Ok(j) => j,
            Err(_) => panic!(
                "Edge (({})->({})) doesn't exist in the graph.",
                src_idx, dst_idx
            ),
        };
        &self.edges[j]
    }

    pub fn leaving_edges(&self, node_idx: usize) -> &[Edge] {
        let i0 = self.offsets[node_idx];
        let i1 = self.offsets[node_idx + 1]; // guaranteed by array-length
        &self.edges[i0..i1]
    }
}

//--------------------------------------------------------------------------------------------------
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
                let node = self.node(i);
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
