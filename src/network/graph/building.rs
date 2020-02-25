use super::{EdgeIdx, Graph, NodeIdx};
use crate::{
    configs::{graph::Config, MetricType},
    network::MetricIdx,
    units::{
        geo, geo::Coordinate, length::Meters, speed::KilometersPerHour, time::Milliseconds,
        MetricU32,
    },
};
use log::info;
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
    pub src_id: i64,
    pub dst_id: i64,
    pub metrics: Vec<Option<MetricU32>>,
}

impl ProtoEdge {
    pub fn _new(src_id: i64, dst_id: i64, metric_count: usize) -> ProtoEdge {
        let proto_edge = ProtoEdge {
            src_id,
            dst_id,
            metrics: vec![None; metric_count],
        };
        proto_edge
    }

    fn metric(&self, idx: MetricIdx) -> Result<Option<MetricU32>, String> {
        let idx = idx.to_usize();
        if let Some(value) = self.metrics.get(idx) {
            Ok(*value)
        } else {
            Err(format!(
                "A proto-edge is expected to have len={}, but len={}.",
                idx + 1,
                self.metrics.len()
            ))
        }
    }

    pub fn _set_metric(&mut self, metric_idx: MetricIdx, value: MetricU32) {
        self.metrics[metric_idx.to_usize()] = Some(value);
    }
}

/// handy for remembering indices after sorting backwards
#[derive(Debug)]
struct MiniProtoEdge {
    src_id: i64,
    dst_id: i64,
    idx: usize,
}

//------------------------------------------------------------------------------------------------//
// graphbuilding

/// private stuff for graph-building
impl Graph {
    fn new(cfg: Config) -> Graph {
        Graph {
            cfg,
            // nodes
            node_ids: Vec::new(),
            // node-metrics
            node_coords: Vec::new(),
            // edges
            fwd_dsts: Vec::new(),
            fwd_offsets: Vec::new(),
            fwd_to_fwd_map: Vec::new(),
            bwd_dsts: Vec::new(),
            bwd_offsets: Vec::new(),
            bwd_to_fwd_map: Vec::new(),
            // edge-metrics
            metrics: Vec::new(),
        }
    }

    /// Optimizes capacity of used data-structures.
    fn shrink_to_fit(&mut self) {
        self.node_ids.shrink_to_fit();
        self.node_coords.shrink_to_fit();
        self.fwd_dsts.shrink_to_fit();
        self.fwd_offsets.shrink_to_fit();
        self.fwd_to_fwd_map.shrink_to_fit();
        self.bwd_dsts.shrink_to_fit();
        self.bwd_offsets.shrink_to_fit();
        self.bwd_to_fwd_map.shrink_to_fit();
        self.metrics.shrink_to_fit();
    }

    fn init_metrics(&mut self, edges_count: usize) {
        self.metrics = vec![Vec::with_capacity(edges_count); self.cfg.edges.metric_count()];
    }

    /// Uses the graph's nodes, so nodes must have been added before this method works properly.
    /// The provided edge is used as forward-edge.
    fn add_metrics(&mut self, proto_edge: &mut ProtoEdge) -> Result<(), String> {
        let (src_idx, dst_idx) = {
            let nodes = self.nodes();
            let src_idx = nodes.src_idx_from(proto_edge.src_id)?;
            let dst_idx = nodes.dst_idx_from(proto_edge.dst_id)?;
            (src_idx, dst_idx)
        };

        // Repeat the calculations n times.
        // In worst case, no metric is provided and all have to be calculated sequentially.
        let mut is_metric_processed = vec![false; self.cfg.edges.metric_types.len()];
        for _ in 0..self.cfg.edges.metric_types.len() {
            // add metrics to graph
            // For every metric holds: if graph expects metric, it should be provided explicetly or implicitly.
            // For calculating metrics from other metrics, multiple loop-runs are necessary.
            for (i, metric_type) in self.cfg.edges.metric_types.iter().enumerate() {
                // calc every metric only once
                if is_metric_processed[i] {
                    continue;
                }

                let metric_idx = self.cfg.edges.metric_idx(&metric_type);
                let mut new_value = None;
                match metric_type {
                    &MetricType::Length { provided } => {
                        // If length is not provided
                        // -> calculate it by coordinates and haversine.
                        if !provided {
                            let src_coord = self.node_coords[src_idx.to_usize()];
                            let dst_coord = self.node_coords[dst_idx.to_usize()];
                            new_value =
                                Some(geo::haversine_distance_m(&src_coord, &dst_coord).into())
                        } else if let Some(metric_idx) = metric_idx {
                            new_value = proto_edge.metric(metric_idx)?
                        }

                        // Length should be at least 1 to prevent errors, e.g. when dividing.
                        if let Some(value) = new_value {
                            new_value = Some(max(MetricU32::from(1u32), value));
                        }
                    }
                    &MetricType::Duration { provided } => {
                        if let Some(metric_idx) = metric_idx {
                            // If not provided, but expected in graph
                            // -> calc with length and maxspeed below
                            if !provided {
                                let doesnt_matter = true;
                                // get length
                                let length = {
                                    if let Some(length_idx) =
                                        self.cfg.edges.metric_idx(&MetricType::Length {
                                            provided: doesnt_matter,
                                        })
                                    {
                                        proto_edge.metric(length_idx)?
                                    } else {
                                        None
                                    }
                                };
                                // get maxspeed
                                let maxspeed = {
                                    if let Some(maxspeed_idx) =
                                        self.cfg.edges.metric_idx(&MetricType::Maxspeed {
                                            provided: doesnt_matter,
                                        })
                                    {
                                        proto_edge.metric(maxspeed_idx)?
                                    } else {
                                        None
                                    }
                                };
                                // calc duration
                                if let (Some(length), Some(maxspeed)) = (length, maxspeed) {
                                    let duration =
                                        Meters::from(*length) / KilometersPerHour::from(*maxspeed);
                                    new_value = Some(duration.into())
                                } else {
                                    continue;
                                }
                            } else {
                                new_value = proto_edge.metric(metric_idx)?
                            }
                        }
                    }
                    &MetricType::Maxspeed { provided } => {
                        if let Some(metric_idx) = metric_idx {
                            // If not provided, but expected in graph
                            // -> calc with length and duration below
                            if !provided {
                                let doesnt_matter = true;
                                // get length
                                let length = {
                                    if let Some(length_idx) =
                                        self.cfg.edges.metric_idx(&MetricType::Length {
                                            provided: doesnt_matter,
                                        })
                                    {
                                        proto_edge.metric(length_idx)?
                                    } else {
                                        None
                                    }
                                };
                                // get duration
                                let duration = {
                                    if let Some(duration_idx) =
                                        self.cfg.edges.metric_idx(&MetricType::Duration {
                                            provided: doesnt_matter,
                                        })
                                    {
                                        proto_edge.metric(duration_idx)?
                                    } else {
                                        None
                                    }
                                };
                                // calc duration
                                if let (Some(length), Some(duration)) = (length, duration) {
                                    let maxspeed =
                                        Meters::from(*length) / Milliseconds::from(*duration);
                                    new_value = Some(maxspeed.into())
                                } else {
                                    continue;
                                }
                            } else {
                                new_value = proto_edge.metric(metric_idx)?
                            }
                        }
                    }
                    MetricType::LaneCount | MetricType::Custom { id: _ } => {
                        if let Some(metric_idx) = metric_idx {
                            new_value = proto_edge.metric(metric_idx)?
                        }
                    }
                    MetricType::Id { id: _ } | MetricType::Ignore { id: _ } => {
                        is_metric_processed[i] = true;
                        continue;
                    }
                };

                // due to continue, new_value is expected to be some
                let new_value = new_value.ok_or(format!(
                    "New value of metric {} should be given here, but isn't.",
                    metric_type
                ))?;
                let metric_idx = metric_idx.unwrap().to_usize();

                // Update proto-edge to calculate other metrics as well.
                // Then update graph's metrics and mark metric as processed.
                proto_edge.metrics[metric_idx] = Some(new_value);
                self.metrics[metric_idx].push(new_value);
                is_metric_processed[i] = true;
            }

            // if all metrics have been calculated -> done
            if is_metric_processed.iter().fold(true, |a, b| (a && *b)) {
                return Ok(());
            }
        }

        // If expected metrics haven't been calculated yet, some metrics are missing!
        Err(format!(
            "Metrics couldn't be calculated since at least one metric is missing for calculations."
        ))
    }
}

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

    pub fn push_edge(&mut self, proto_edge: ProtoEdge) -> &mut Self {
        // add or update src-node
        let src_id = proto_edge.src_id;
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
        let dst_id = proto_edge.dst_id;
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

        // add edge
        self.proto_edges.push(proto_edge);

        self
    }

    pub fn finalize(mut self, cfg: Config) -> Result<Graph, String> {
        //----------------------------------------------------------------------------------------//
        // init graph

        info!(
            "START Finalize graph with {} proto-nodes and {} proto-edges.",
            self.proto_nodes.len(),
            self.proto_edges.len()
        );
        let mut graph = Graph::new(cfg);

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
                return Err(format!(
                    "Proto-node (id: {}) has no coordinates, but belongs to an edge.",
                    proto_node.id
                ));
            }

            // print progress
            if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }
        }
        info!("{}", progress_bar);
        // reduce and optimize memory-usage
        // already dropped via iterator: drop(self.proto_nodes);
        graph.shrink_to_fit();
        assert_eq!(
            graph.node_ids.len() == graph.node_coords.len(),
            node_idx == graph.node_ids.len(),
            "The (maximum index - 1) should not be more than the number of nodes in the graph."
        );
        info!("FINISHED");

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
        // build metrics
        // If metrics are built before indices and offsets are built, the need of memory while
        // building is reduced.

        info!("START Create/store/filter metrics.");
        let mut progress_bar = progressing::MappingBar::new(0..=self.proto_edges.len());
        // init metric-collections in graph
        graph.init_metrics(self.proto_edges.len());
        // start looping
        let mut edge_idx = 0usize;
        for proto_edge in self.proto_edges.iter_mut() {
            graph.add_metrics(proto_edge)?;

            // print progress
            progress_bar.set(edge_idx);
            if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                info!("{}", progress_bar);
            }

            // update edge-idx
            edge_idx += 1;
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        info!("{}", progress_bar.set(edge_idx));
        // reduce and optimize memory-usage
        graph.shrink_to_fit();
        // Drop edges
        let mut new_proto_edges: Vec<MiniProtoEdge> = self
            .proto_edges
            .into_iter()
            .map(|e| MiniProtoEdge {
                src_id: e.src_id,
                dst_id: e.dst_id,
                idx: 0,
            })
            .collect();
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build forward-offset-array and edges

        // logging
        info!("START Create the forward-offset-array and the forward-mapping.");
        let mut progress_bar = progressing::MappingBar::new(0..=new_proto_edges.len());
        // start looping
        let mut src_idx = NodeIdx::zero();
        let mut offset = 0;
        graph.fwd_offsets.push(offset);
        // high-level-idea
        // count offset for each proto_edge (sorted) and apply offset as far as src doesn't change
        let mut edge_idx = 0;
        for proto_edge in new_proto_edges.iter_mut() {
            // find edge-data to compare it with expected data later (when setting offset)
            let src_id = proto_edge.src_id;
            let dst_id = proto_edge.dst_id;

            // Add edge-idx here to remember it for indirect mapping bwd->fwd.
            // Update it at the end of the loop.
            proto_edge.idx = edge_idx;

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
        graph.shrink_to_fit();
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
        info!("{}", progress_bar.set(edge_idx));
        // reduce and optimize memory-usage
        graph.shrink_to_fit();
        info!("FINISHED");

        info!("FINISHED");

        Ok(graph)
    }
}
