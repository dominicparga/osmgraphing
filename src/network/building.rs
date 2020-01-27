use std::cmp::{max, Ordering};
use std::collections::BTreeMap;

use log::{error, info};

use super::{geo, Edge, Graph, Node};

//------------------------------------------------------------------------------------------------//

pub struct ProtoNode {
    id: i64,
    coord: Option<geo::Coordinate>,
    edge_count: u16,
}
impl ProtoNode {
    fn is_in_edge(&self) -> bool {
        self.edge_count > 0
    }
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
    lane_count: u8,
    meters: Option<u32>,
    maxspeed: u16,
}
impl Eq for ProtoEdge {}
impl PartialEq for ProtoEdge {
    fn eq(&self, other: &ProtoEdge) -> bool {
        self.way_id == other.way_id
            && self.src_id == other.src_id
            && self.dst_id == other.dst_id
            && self.lane_count == other.lane_count
            && self.meters == other.meters
            && self.maxspeed == other.maxspeed
    }
}

//------------------------------------------------------------------------------------------------//
// graphbuilding

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
                    edge_count: 0,
                },
            );
        }
        self
    }

    pub fn is_node_in_edge(&self, id: i64) -> bool {
        if let Some(proto_node) = self.proto_nodes.get(&id) {
            proto_node.is_in_edge()
        } else {
            false
        }
    }

    pub fn push_edge(
        &mut self,
        way_id: Option<i64>,
        src_id: i64,
        dst_id: i64,
        lane_count: u8,
        meters: Option<u32>,
        maxspeed: u16,
    ) -> &mut Self {
        // add edge
        self.proto_edges.push(ProtoEdge {
            way_id,
            src_id,
            dst_id,
            lane_count,
            meters,
            maxspeed,
        });

        // add or update src-node
        if let Some(proto_node) = self.proto_nodes.get_mut(&src_id) {
            proto_node.edge_count += 1;
        } else {
            self.proto_nodes.insert(
                src_id,
                ProtoNode {
                    id: src_id,
                    coord: None,
                    edge_count: 1,
                },
            );
        }

        // add or update dst-node
        if let Some(proto_node) = self.proto_nodes.get_mut(&dst_id) {
            proto_node.edge_count += 1;
        } else {
            self.proto_nodes.insert(
                dst_id,
                ProtoNode {
                    id: dst_id,
                    coord: None,
                    edge_count: 1,
                },
            );
        }

        self
    }

    pub fn finalize(mut self) -> Result<Graph, String> {
        //----------------------------------------------------------------------------------------//
        // init graph

        info!(
            "Starting finalizing graph ({} proto-nodes and {} proto-edges) ..",
            self.proto_nodes.len(),
            self.proto_edges.len()
        );
        let mut graph = Graph::new();

        //----------------------------------------------------------------------------------------//
        // sort edges by ascending src-id, then by ascending dst-id -> offset-array

        info!("Starting sorting proto-edges by their src/dst-IDs ..");
        self.proto_edges.sort_by(|e0, e1| {
            e0.src_id
                .cmp(&e1.src_id)
                .then_with(|| e0.dst_id.cmp(&e1.dst_id))
        });
        info!("Finished sorting proto-edges");

        //----------------------------------------------------------------------------------------//
        // add nodes to graph which belong to edges (sorted by asc id)

        info!("Starting adding nodes (sorted) which belongs to an edge ..");
        let mut node_idx = 0;
        // BTreeMap's iter returns sorted by key (asc)
        for (_id, proto_node) in self.proto_nodes.iter() {
            // add nodes only if they belong to an edge
            if !proto_node.is_in_edge() {
                continue;
            }

            // add new node
            if let Some(coord) = proto_node.coord {
                graph.nodes.push(Node {
                    id: proto_node.id,
                    idx: node_idx,
                    coord,
                });
                node_idx += 1;
            } else {
                // should not happen if file is okay
                error!(
                    "Proto-node (id: {}) has no coordinates, but belongs to an edge",
                    proto_node.id
                );
            }
        }
        debug_assert_eq!(
            graph.nodes.len(),
            node_idx,
            "The (maximum index - 1) should not be more than the number of nodes in the graph."
        );
        info!("Finished adding nodes");

        //----------------------------------------------------------------------------------------//
        // build offset-array and edges

        info!("Starting creating the offset-array ..");
        let mut offset_node_idx = 0;
        let mut offset = 0;
        graph.fwd_offsets.push(offset);
        // high-level-idea
        // count offset for each proto_edge (sorted) and apply offset as far as src changes
        for edge_idx in 0..self.proto_edges.len() {
            let proto_edge = &self.proto_edges[edge_idx];
            // set way-id to index
            let edge_way_id = proto_edge.way_id.unwrap_or(edge_idx as i64);

            // find source-index in sorted vec of nodes
            let edge_src_idx = match graph.nodes().idx_from(proto_edge.src_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given src-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                        proto_edge.src_id, proto_edge.way_id
                    ))
                }
            };

            // find destination-index in sorted vec of nodes
            let edge_dst_idx = match graph.nodes().idx_from(proto_edge.dst_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given dst-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                        proto_edge.dst_id, proto_edge.way_id
                    ))
                }
            };

            // calculate distance if not provided
            let meters = max(
                1,
                match proto_edge.meters {
                    Some(meters) => meters,
                    None => {
                        let src = graph
                            .nodes
                            .get(edge_src_idx)
                            .expect("src-node should exist.");
                        let dst = graph
                            .nodes
                            .get(edge_dst_idx)
                            .expect("dst-node should exist.");
                        (geo::haversine_distance(&src.coord, &dst.coord) * 1_000.0) as u32
                    }
                },
            );

            // add new edge to graph
            let edge = Edge {
                id: edge_way_id,
                src_idx: edge_src_idx,
                dst_idx: edge_dst_idx,
                lane_count: proto_edge.lane_count,
                meters,
                maxspeed: proto_edge.maxspeed,
            };

            // if coming edges have new src
            // then update offset of new src
            while offset_node_idx != edge_src_idx {
                offset_node_idx += 1;
                graph.fwd_offsets.push(offset);
            }
            graph.edges.push(edge);
            graph.fwd_indices.push(edge_idx);
            offset += 1;
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        graph.fwd_offsets.push(offset);

        // optimize memory a little
        graph.nodes.shrink_to_fit();
        graph.edges.shrink_to_fit();
        graph.fwd_indices.shrink_to_fit();
        graph.fwd_offsets.shrink_to_fit();
        info!("Finished creating offset-array");

        Ok(graph)
    }
}
