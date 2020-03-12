use super::{EdgeIdx, Graph, NodeIdx};
use crate::{
    configs::parser::{Config, EdgeCategory},
    defaults::DimVec,
    helpers::ApproxEq,
    network::MetricIdx,
    units::{geo, geo::Coordinate, length::Kilometers, speed::KilometersPerHour, time::Seconds},
};
use log::{debug, info};
use progressing;
use progressing::Bar;
use std::{cmp::Reverse, collections::BTreeMap, mem};

#[derive(Debug)]
pub struct ProtoNode {
    id: i64,
    coord: Option<Coordinate>,
    level: Option<usize>,
    is_in_edge: bool,
}

impl ProtoNode {
    pub fn new(id: i64, coord: Option<Coordinate>, level: Option<usize>) -> ProtoNode {
        ProtoNode {
            id,
            coord,
            level,
            is_in_edge: false,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    fn is_in_edge(&self) -> bool {
        self.is_in_edge
    }
}

#[derive(Debug)]
pub struct ProtoEdge {
    src_id: i64,
    dst_id: i64,
    metrics: DimVec<Option<f32>>,
}

impl ProtoEdge {
    pub fn new(src_id: i64, dst_id: i64, metrics: DimVec<Option<f32>>) -> ProtoEdge {
        ProtoEdge {
            src_id,
            dst_id,
            metrics,
        }
    }
}

/// handy for remembering indices after sorting backwards
#[derive(Debug)]
struct MiniProtoEdge {
    src_id: i64,
    dst_id: i64,
    idx: usize,
}

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

    /// Uses the graph's nodes, so nodes must have been added before this method works properly.
    /// The provided edge is interpreted as forward-edge.
    /// Metric-dependencies between each other are considered by looping enough times
    /// over the calculation-loop.
    fn add_metrics(&mut self, proto_edge: &mut ProtoEdge) -> Result<(), String> {
        let cfg = &self.cfg;

        let (src_idx, dst_idx) = {
            let nodes = self.nodes();
            let src_idx = nodes.src_idx_from(proto_edge.src_id)?;
            let dst_idx = nodes.dst_idx_from(proto_edge.dst_id)?;
            (src_idx, dst_idx)
        };

        // - finalize proto-edge -
        // Repeat the calculations n times.
        // In worst case, no metric is provided and all have to be calculated sequentially.
        for _ in 0..cfg.edges.dim() {
            // just a quick, coarse way of breaking earlier
            let mut are_all_metrics_some = true;
            for metric_idx in (0..cfg.edges.dim()).map(MetricIdx) {
                // if value should be provided, it is already in the proto-edge from parsing
                if cfg.edges.is_metric_provided(metric_idx) {
                    continue;
                }

                let category = cfg.edges.metric_category(metric_idx);

                // Jump if proto-edge has its value.
                if let Some(value) = &mut proto_edge.metrics[*metric_idx] {
                    // But jump only if value is correct.
                    if value.approx_eq(&0.0) && category.must_be_positive() {
                        debug!(
                            "Proto-edge (id:{}->id:{}) has {}=0, hence is corrected to epsilon.",
                            proto_edge.src_id, proto_edge.dst_id, category
                        );
                        *value = std::f32::EPSILON;
                    }
                    continue;
                }
                // now: proto-edge has no value for this metric and has to be updated

                // calculate metric dependent on category
                match category {
                    EdgeCategory::Meters => {
                        let src_coord = self.node_coords[*src_idx];
                        let dst_coord = self.node_coords[*dst_idx];
                        proto_edge.metrics[*metric_idx] =
                            Some(*geo::haversine_distance_km(&src_coord, &dst_coord));
                    }
                    EdgeCategory::Seconds => {
                        // get length and maxspeed to calculate duration
                        let mut length = None;
                        let mut maxspeed = None;
                        // get calculation-rules
                        for &(other_type, other_idx) in cfg.edges.calc_rules(metric_idx) {
                            // get values from edge dependent of calculation-rules
                            match other_type {
                                EdgeCategory::Meters => length = proto_edge.metrics[*other_idx],
                                EdgeCategory::KilometersPerHour => {
                                    maxspeed = proto_edge.metrics[*other_idx];
                                }
                                _ => {
                                    return Err(format!(
                                    "Wrong metric-category {} in calc-rule for metric-category {}",
                                    other_type, category
                                ))
                                }
                            }
                        }
                        // calc duration and update proto-edge
                        if let (Some(length), Some(maxspeed)) = (length, maxspeed) {
                            let duration: Seconds =
                                Kilometers(length) / KilometersPerHour(maxspeed);
                            proto_edge.metrics[*metric_idx] = Some(*duration)
                        } else {
                            are_all_metrics_some = false;
                        }
                    }
                    EdgeCategory::KilometersPerHour => {
                        // get length and duration to calculate maxspeed
                        let mut length = None;
                        let mut duration = None;
                        // get calculation-rules
                        for &(other_type, other_idx) in cfg.edges.calc_rules(metric_idx) {
                            // get values from edge dependent of calculation-rules
                            match other_type {
                                EdgeCategory::Meters => length = proto_edge.metrics[*other_idx],
                                EdgeCategory::Seconds => {
                                    duration = proto_edge.metrics[*other_idx];
                                }
                                _ => {
                                    return Err(format!(
                                    "Wrong metric-category {} in calc-rule for metric-category {}",
                                    other_type, category
                                ))
                                }
                            }
                        }
                        // calc maxspeed and update proto-edge
                        if let (Some(length), Some(duration)) = (length, duration) {
                            let maxspeed: KilometersPerHour =
                                Kilometers(length) / Seconds(duration);
                            proto_edge.metrics[*metric_idx] = Some(*maxspeed)
                        } else {
                            are_all_metrics_some = false;
                        }
                    }
                    EdgeCategory::LaneCount
                    | EdgeCategory::Custom
                    | EdgeCategory::SrcId
                    | EdgeCategory::IgnoredSrcIdx
                    | EdgeCategory::DstId
                    | EdgeCategory::IgnoredDstIdx
                    | EdgeCategory::Ignore => {
                        // Should be set to false here, but being here needs the metric to be none.
                        // This would be bad anyways, because these metrics should be provided, not
                        // calculated.
                        // -> breaking loop for performance is okay
                        // are_all_metrics_some = false;
                    }
                }
            }

            if are_all_metrics_some {
                break;
            }
        }

        // add metrics to graph
        for (i, value) in proto_edge.metrics.iter().enumerate() {
            let metric_idx = MetricIdx(i);
            // If expected metrics haven't been calculated yet, some metrics are missing!
            if let &Some(value) = value {
                self.metrics[*metric_idx].push(value);
            } else {
                if cfg.edges.is_metric_provided(metric_idx) {
                    return Err(format!(
                        "Metric {} should be provided, but is not.",
                        cfg.edges.metric_category(metric_idx)
                    ));
                }
                return Err(format!(
                    "Metric {} couldn't be calculated \
                     since not enough calculation rules were given.",
                    cfg.edges.metric_category(metric_idx)
                ));
            }
        }

        Ok(())
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

    pub fn push_node(&mut self, new_proto_node: ProtoNode) -> &mut Self {
        // if already added -> update coord
        // if not -> add new node
        if let Some(proto_node) = self.proto_nodes.get_mut(&new_proto_node.id) {
            proto_node.coord = new_proto_node.coord;
        } else {
            self.proto_nodes.insert(new_proto_node.id, new_proto_node);
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
            proto_node.is_in_edge = true;
        } else {
            self.proto_nodes.insert(
                src_id,
                ProtoNode {
                    id: src_id,
                    coord: None,
                    level: None,
                    is_in_edge: true,
                },
            );
        }

        // add or update dst-node
        let dst_id = proto_edge.dst_id;
        if let Some(proto_node) = self.proto_nodes.get_mut(&dst_id) {
            proto_node.is_in_edge = true;
        } else {
            self.proto_nodes.insert(
                dst_id,
                ProtoNode {
                    id: dst_id,
                    coord: None,
                    level: None,
                    is_in_edge: true,
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
        graph.node_ids.reserve_exact(self.proto_nodes.len());
        graph.node_coords.reserve_exact(self.proto_nodes.len());
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
        // memory-peak is here when sorting

        // sort reversed to make splice efficient
        self.proto_edges
            .sort_unstable_by_key(|edge| Reverse((edge.src_id, edge.dst_id)));
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // remove duplicates

        info!("START Remove duplicated proto-edges and flatten ch-shortcuts");
        let edge_count = self.proto_edges.len();
        // e.g. edge
        // node-id 314074041 -> node-id 283494218
        // which is part of two ways
        //
        // The metrics contain f32, which is not comparable exactly.
        // This is okay for here, because if two edges of identical src/dst have different
        // metrics, than they are created differently and thus are indeed different.
        self.proto_edges.dedup_by(|e0, e1| {
            (e0.src_id, e0.dst_id, &e0.metrics) == (e1.src_id, e1.dst_id, &e1.metrics)
        });
        if self.proto_edges.len() != edge_count {
            info!(
                "Removed {} duplicates.",
                (edge_count - self.proto_edges.len())
            );
        }
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build metrics
        // If metrics are built before indices and offsets are built, the need of memory while
        // building is reduced.

        info!("START Create/store/filter metrics.");
        let mut progress_bar = progressing::MappingBar::new(0..=self.proto_edges.len());
        let mut edge_idx: usize = 0;

        // Work off proto-edges in chunks to keep memory-usage lower.
        // For example:
        // To keep additional memory-needs below 1 MB, the the maximum amount of four f32-values per
        // worked-off chunk has to be limited to 250_000.
        // Because ids are more expensive than metrics, (2x i64 = 4x f32), the number is much lower.
        let bytes_per_edge =
            2 * mem::size_of::<i64>() + graph.cfg().edges.dim() * mem::size_of::<f32>();
        let max_byte = 200 * 1_000_000;
        let max_chunk_size = max_byte / bytes_per_edge;
        log::debug!("max-chunk-size:                {}", max_chunk_size);
        // init metrics
        graph.metrics = vec![vec![]; graph.cfg().edges.dim()];
        let mut new_proto_edges = vec![];
        log::debug!(
            "initial graph-metric-capacity: {}",
            graph.metrics[0].capacity()
        );

        while self.proto_edges.len() > 0 {
            // Get chunk from proto-edges.
            // Reverse chunk because proto-egdes is sorted reversed to make splice efficient.
            let chunk: Vec<_> = if self.proto_edges.len() > max_chunk_size {
                // ATTENTION! Splicing means that the given range is replaced,
                // hence max_chunk_size has to be, kind of, inverted.
                self.proto_edges
                    .splice((self.proto_edges.len() - max_chunk_size).., vec![])
            } else {
                self.proto_edges.splice(.., vec![])
            }
            .rev()
            .collect();

            // allocate new memory-needs
            self.proto_edges.shrink_to_fit();
            graph
                .metrics
                .iter_mut()
                .for_each(|m| m.reserve_exact(chunk.len()));
            new_proto_edges.reserve_exact(chunk.len());
            log::debug!("chunk-len:             {}", chunk.len());
            log::debug!("graph-metric-capacity: {}", graph.metrics[0].capacity());

            for mut proto_edge in chunk.into_iter() {
                // add to graph and remember ids
                graph.add_metrics(&mut proto_edge)?;
                new_proto_edges.push(MiniProtoEdge {
                    src_id: proto_edge.src_id,
                    dst_id: proto_edge.dst_id,
                    idx: 0, // used later for offset-arrays
                });

                // print progress
                progress_bar.set(edge_idx);
                if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                    info!("{}", progress_bar);
                }

                // update edge-idx
                edge_idx += 1;
            }
        }
        info!("{}", progress_bar.set(edge_idx));
        // Drop edges
        drop(self.proto_edges);
        // reduce and optimize memory-usage
        graph.shrink_to_fit();
        new_proto_edges.shrink_to_fit();
        // last node needs an upper bound as well for `leaving_edges(...)`
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build forward-offset-array and edges

        // logging
        info!("START Create the forward-offset-array and the forward-mapping.");
        let mut progress_bar = progressing::MappingBar::new(0..=new_proto_edges.len());
        // start looping
        let mut src_idx = NodeIdx(0);
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
                *src_idx += 1;
                graph.fwd_offsets.push(offset);
            }
            offset += 1;
            graph.bwd_dsts.push(edge_src_idx);
            graph.fwd_dsts.push(edge_dst_idx);
            // mapping fwd to fwd is just the identity
            graph.fwd_to_fwd_map.push(EdgeIdx(edge_idx));

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
        new_proto_edges.sort_unstable_by_key(|edge| (edge.dst_id, edge.src_id));
        info!("FINISHED");

        //----------------------------------------------------------------------------------------//
        // build backward-offset-array

        // logging
        info!("START Create the backward-offset-array.");
        let mut progress_bar = progressing::MappingBar::new(0..=new_proto_edges.len());
        // start looping
        let mut src_idx = NodeIdx(0);
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
                *src_idx += 1;
                graph.bwd_offsets.push(offset);
            }
            offset += 1;
            // For the backward-mapping, bwd-indices have been remembered above,
            // but applied to forward-sorted-edges.
            // Now, that's used to generate the mapping from backward to forward,
            // which is needed for the offset-arrays.
            graph.bwd_to_fwd_map.push(EdgeIdx(proto_edge.idx));

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
        info!("FINISHED"); // bwd-offset-array

        info!("FINISHED"); // finalize
        Ok(graph)
    }
}
