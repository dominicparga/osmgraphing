use super::{EdgeIdx, Graph, Meters, NodeIdx};
use crate::units::{geo, geo::Coordinate};
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
    way_id: Option<i64>,
    src_id: i64,
    dst_id: i64,
    lane_count: u8,
    meters: Option<Meters>,
    maxspeed: u16,
    idx: usize, // handy for remembering indices after sorting backwards
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
            meters: match meters {
                Some(meters) => Some(Meters::from(meters)),
                None => None,
            },
            maxspeed,
            idx: 0, // needed below in finalize
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
        for (_id, proto_node) in self.proto_nodes.iter() {
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
        for edge_idx in 0..self.proto_edges.len() {
            let proto_edge = &mut self.proto_edges[edge_idx];

            // find edge-data to compare it with expected data later (when setting offset)
            let src_id = proto_edge.src_id;
            let dst_id = proto_edge.dst_id;
            // do not swap src and dst since this is a forward-edge
            let edge_src_idx = match graph.nodes().idx_from(src_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given src-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                        proto_edge.src_id, proto_edge.way_id
                    ))
                }
            };
            let edge_dst_idx = match graph.nodes().idx_from(dst_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given dst-id `{:?}` of edge-id `{:?}` doesn't exist as node",
                        proto_edge.dst_id, proto_edge.way_id
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
            // remember index for indirect mapping bwd->fwd
            proto_edge.idx = edge_idx;

            // metrics
            // calculate distance if not provided
            let meters = max(
                Meters::from(1),
                match proto_edge.meters {
                    Some(meters) => meters,
                    None => {
                        let src_coord = graph.nodes().coord(edge_src_idx);
                        let dst_coord = graph.nodes().coord(edge_dst_idx);
                        geo::haversine_distance_m(&src_coord, &dst_coord)
                    }
                },
            );
            graph.meters.push(meters);
            graph.maxspeed.push(proto_edge.maxspeed.into());
            graph.lane_count.push(proto_edge.lane_count);

            // print progress
            progress_bar.set(edge_idx);
            if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        graph.fwd_offsets.push(offset);
        info!("{}", progress_bar.set(offset));
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // sort backward-edges by ascending dst-id, then by ascending src-id -> offset-array

        info!("START Sort proto-backward-edges by their dst/src-IDs.");
        self.proto_edges.sort_by(|e0, e1| {
            e0.dst_id
                .cmp(&e1.dst_id)
                .then_with(|| e0.src_id.cmp(&e1.src_id))
        });
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build backward-offset-array

        // logging
        info!("START Create the backward-offset-array.");
        let mut progress_bar = progressing::MappingBar::new(0..=self.proto_edges.len());
        info!("{}", progress_bar);
        // start looping
        let mut src_idx = NodeIdx::zero();
        let mut offset = 0;
        graph.bwd_offsets.push(offset);
        // high-level-idea
        // count offset for each proto_edge (sorted) and apply offset as far as src doesn't change
        for edge_idx in 0..self.proto_edges.len() {
            let proto_edge = &mut self.proto_edges[edge_idx];

            // find edge-data to compare it with expected data later (when setting offset)
            let dst_id = proto_edge.dst_id;
            // swap src and dst since this is the backward-edge
            let edge_src_idx = match graph.nodes().idx_from(dst_id) {
                Ok(idx) => idx,
                Err(_) => {
                    return Err(format!(
                        "The given dst-id `{:?}` of way-id `{:?}` doesn't exist as node",
                        proto_edge.dst_id, proto_edge.way_id
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
            if progress_bar.progress() % (1 + (progress_bar.end() % 10)) == 0 {
                info!("{}", progress_bar);
            }
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        debug_assert_eq!(
            offset,
            self.proto_edges.len(),
            "Last offset-value should be as big as the number of proto-edges."
        );
        graph.bwd_offsets.push(offset);
        info!("{}", progress_bar.set(offset));
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // optimize memory a little

        graph.node_ids.shrink_to_fit();
        graph.node_coords.shrink_to_fit();
        graph.fwd_dsts.shrink_to_fit();
        graph.fwd_offsets.shrink_to_fit();
        graph.fwd_to_fwd_map.shrink_to_fit();
        graph.bwd_dsts.shrink_to_fit();
        graph.bwd_offsets.shrink_to_fit();
        graph.bwd_to_fwd_map.shrink_to_fit();
        graph.meters.shrink_to_fit();
        graph.maxspeed.shrink_to_fit();
        graph.lane_count.shrink_to_fit();

        info!("FINISHED");

        Ok(graph)
    }
}
