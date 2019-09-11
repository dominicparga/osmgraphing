use std::cmp::Ordering;
use std::collections::BTreeMap;

use log::{error, info};

use super::{geo, Edge, Graph, Node};

//------------------------------------------------------------------------------------------------//

pub struct ProtoNode {
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

//------------------------------------------------------------------------------------------------//

pub struct ProtoEdge {
    way_id: Option<i64>,
    src_id: i64,
    dst_id: i64,
    meters: Option<u32>,
    maxspeed: u16,
}

//------------------------------------------------------------------------------------------------//

pub struct GraphBuilder {
    proto_nodes: BTreeMap<i64, ProtoNode>,
    proto_edges: Vec<ProtoEdge>,
}
impl GraphBuilder {
    pub fn new() -> Self {
        Self {
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

    pub fn is_node_in_edge(&self, id: i64) -> bool {
        if let Some(proto_node) = self.proto_nodes.get(&id) {
            proto_node.is_edge_node
        } else {
            false
        }
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

    pub fn finalize(mut self) -> Result<Graph, String> {
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
                Err(_) => {
                    return Err(format!(
                        "The given src-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                        proto_edge.src_id, proto_edge.way_id
                    ))
                }
            };

            // find destination-index in sorted vec of nodes
            let dst_idx = match graph.node_idx_from(proto_edge.dst_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given dst-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                        proto_edge.dst_id, proto_edge.way_id
                    ))
                }
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

        Ok(graph)
    }
}
