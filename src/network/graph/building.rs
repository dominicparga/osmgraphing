use super::{EdgeIdx, Graph, NodeIdx};
use crate::units::{geo, geo::Coordinate, length::Meters, speed::KilometersPerHour, MetricU8};
use log::{error, info, trace};
use progressing;
use progressing::Bar;
use std::{cmp::max, collections::BTreeMap};

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct ProtoNode {
    id: i64,
    coord: Option<Coordinate>,
    edge_count: u16,
}

impl ProtoNode {
    fn is_in_edge(&self) -> bool {
        self.edge_count > 0
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct ProtoEdge {
    src_id: i64,
    dst_id: i64,
    length: Option<Meters>,
    maxspeed: KilometersPerHour,
    lane_count: MetricU8,
}

/// handy for remembering indices after sorting backwards
struct MiniProtoEdge {
    src_id: i64,
    dst_id: i64,
    idx: usize,
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
        src_id: i64,
        dst_id: i64,
        length: Option<Meters>,
        maxspeed: KilometersPerHour,
        lane_count: MetricU8,
    ) -> &mut Self {
        // add edge
        self.proto_edges.push(ProtoEdge {
            src_id,
            dst_id,
            length: match length {
                Some(meters) => Some(Meters::from(meters)),
                None => None,
            },
            maxspeed,
            lane_count,
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
            "START Finalize graph with {} proto-nodes and {} proto-edges.",
            self.proto_nodes.len(),
            self.proto_edges.len()
        );
        let mut graph = Graph::new();

        //----------------------------------------------------------------------------------------//
        // add nodes to graph which belong to edges (sorted by asc id)

        // logging
        info!("START Add nodes (sorted) which belongs to an edge.");
        let mut progress_bar = progressing::BernoulliBar::from_goal(self.proto_nodes.len() as u32);
        info!("{}", progress_bar);
        // start looping
        let mut node_idx = 0;
        // BTreeMap's iter returns sorted by key (asc)
        for (_id, proto_node) in self.proto_nodes.into_iter() {
            // add nodes only if they belong to an edge
            if !proto_node.is_in_edge() {
                trace!(
                    "Proto-node (id: {}) has no coordinates, but belongs to an edge",
                    proto_node.id
                );
                progress_bar.add((0, 1));
                continue;
            }

            // add new node
            if let Some(coord) = proto_node.coord {
                graph.node_ids.push(proto_node.id);
                graph.node_coords.push(coord);
                node_idx += 1;
                progress_bar.add((1, 1));
            } else {
                // should not happen if file is okay
                error!(
                    "Proto-node (id: {}) has no coordinates, but belongs to an edge",
                    proto_node.id
                );
                progress_bar.add((0, 1));
            }

            // print progress
            if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }
        }
        info!("{}", progress_bar);
        // reduce and optimize memory-usage
        // already dropped via iterator: drop(self.proto_nodes);
        graph.node_ids.shrink_to_fit();
        graph.node_coords.shrink_to_fit();
        info!("FINISHED");
        assert_eq!(
            graph.node_ids.len() == graph.node_coords.len(),
            node_idx == graph.node_ids.len(),
            "The (maximum index - 1) should not be more than the number of nodes in the graph."
        );

        //----------------------------------------------------------------------------------------//
        // sort forward-edges by ascending src-id, then by ascending dst-id -> offset-array

        info!("START Sort proto-forward-edges by their src/dst-IDs.");
        self.proto_edges.sort_by(|e0, e1| {
            e0.src_id
                .cmp(&e1.src_id)
                .then_with(|| e0.dst_id.cmp(&e1.dst_id))
        });
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build forward-offset-array and edges

        // logging
        info!("START Create the forward-offset-array and the forward-mapping.");
        let mut progress_bar = progressing::MappingBar::new(0..=self.proto_edges.len());
        info!("{}", progress_bar);
        // start looping
        let mut src_idx = NodeIdx::zero();
        let mut offset = 0;
        graph.fwd_offsets.push(offset);
        // high-level-idea
        // count offset for each proto_edge (sorted) and apply offset as far as src doesn't change
        let mut edge_idx = 0;
        let mut new_proto_edges = Vec::with_capacity(self.proto_edges.len());
        for proto_edge in self.proto_edges.into_iter() {
            // find edge-data to compare it with expected data later (when setting offset)
            let src_id = proto_edge.src_id;
            let dst_id = proto_edge.dst_id;

            // Add edge-idx here to remember it for indirect mapping bwd->fwd.
            // Update it at the end of the loop.
            new_proto_edges.push(MiniProtoEdge {
                src_id,
                dst_id,
                idx: edge_idx,
            });

            // do not swap src and dst since this is a forward-edge
            let edge_src_idx = match graph.nodes().idx_from(src_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given src-id `{:?}` doesn't exist as node",
                        proto_edge.src_id
                    ))
                }
            };
            let edge_dst_idx = match graph.nodes().idx_from(dst_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given dst-id `{:?}` doesn't exist as node",
                        proto_edge.dst_id
                    ))
                }
            };

            // If coming edges have new src, then update offset of new src.
            // Loop because of nodes with no leaving edges.
            // Nodes of id y with no leaving edge must have the same offset as the node of id (y+1)
            // to remember it.
            while src_idx != edge_src_idx.into() {
                src_idx += 1;
                graph.fwd_offsets.push(offset);
            }
            offset += 1;
            graph.bwd_dsts.push(edge_src_idx);
            graph.fwd_dsts.push(edge_dst_idx);
            // mapping fwd to fwd is just the identity
            graph.fwd_to_fwd_map.push(EdgeIdx::new(edge_idx));

            // metrics
            // calculate distance if not provided
            let length = max(
                Meters::new(1),
                match proto_edge.length {
                    Some(meters) => meters,
                    None => {
                        let src_coord = graph.nodes().coord(edge_src_idx);
                        let dst_coord = graph.nodes().coord(edge_dst_idx);
                        geo::haversine_distance_m(&src_coord, &dst_coord)
                    }
                },
            );
            graph.lengths.push(length);
            graph.maxspeeds.push(proto_edge.maxspeed.into());
            graph.lane_counts.push(proto_edge.lane_count);

            // print progress
            progress_bar.set(edge_idx);
            if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }

            // update edge-idx
            edge_idx += 1;
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        graph.fwd_offsets.push(offset);
        info!("{}", progress_bar.set(offset));
        // reduce and optimize memory-usage
        // already dropped via iterator: drop(self.proto_edges);
        graph.fwd_dsts.shrink_to_fit();
        graph.fwd_offsets.shrink_to_fit();
        graph.fwd_to_fwd_map.shrink_to_fit();
        graph.bwd_dsts.shrink_to_fit();
        graph.lengths.shrink_to_fit();
        graph.maxspeeds.shrink_to_fit();
        graph.lane_counts.shrink_to_fit();
        graph.metrics_u32.shrink_to_fit();
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // sort backward-edges by ascending dst-id, then by ascending src-id -> offset-array

        info!("START Sort proto-backward-edges by their dst/src-IDs.");
        new_proto_edges.sort_by(|e0, e1| {
            e0.dst_id
                .cmp(&e1.dst_id)
                .then_with(|| e0.src_id.cmp(&e1.src_id))
        });
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build backward-offset-array

        // logging
        info!("START Create the backward-offset-array.");
        let mut progress_bar = progressing::MappingBar::new(0..=new_proto_edges.len());
        info!("{}", progress_bar);
        // start looping
        let mut src_idx = NodeIdx::zero();
        let mut offset = 0;
        graph.bwd_offsets.push(offset);
        // high-level-idea
        // count offset for each proto_edge (sorted) and apply offset as far as src doesn't change
        for edge_idx in 0..new_proto_edges.len() {
            let proto_edge = &mut new_proto_edges[edge_idx];

            // find edge-data to compare it with expected data later (when setting offset)
            let dst_id = proto_edge.dst_id;
            // swap src and dst since this is the backward-edge
            let edge_src_idx = match graph.nodes().idx_from(dst_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given dst-id `{:?}` doesn't exist as node",
                        proto_edge.dst_id
                    ));
                }
            };

            // If coming edges have new src, then update offset of new src.
            // Loop because of nodes with no leaving edges.
            // Nodes of id y with no leaving edge must have the same offset as the node of id (y+1)
            // to remember it.
            while src_idx != edge_src_idx {
                src_idx += 1;
                graph.bwd_offsets.push(offset);
            }
            offset += 1;
            // For the backward-mapping, bwd-indices have been remembered above,
            // but applied to forward-sorted-edges.
            // Now, that's used to generate the mapping from backward to forward,
            // which is needed for the offset-arrays.
            graph.bwd_to_fwd_map.push(EdgeIdx::new(proto_edge.idx));

            // print progress
            progress_bar.set(edge_idx);
            if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        debug_assert_eq!(
            offset,
            new_proto_edges.len(),
            "Last offset-value should be as big as the number of proto-edges."
        );
        graph.bwd_offsets.push(offset);
        info!("{}", progress_bar.set(offset));
        // reduce and optimize memory-usage
        graph.bwd_offsets.shrink_to_fit();
        graph.bwd_to_fwd_map.shrink_to_fit();
        info!("FINISHED");

        info!("FINISHED");

        Ok(graph)
    }
}
