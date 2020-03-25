use super::{EdgeIdx, Graph, NodeIdx};
use crate::{
    configs::parser::{Config, EdgeCategory},
    defaults::{
        self,
        capacity::{self, DimVec},
    },
    helpers::{ApproxEq, MemSize},
    network::MetricIdx,
    units::geo::{self, Coordinate},
};
use log::{debug, info};
use progressing::{self, Bar};
use std::{cmp::Reverse, mem, ops::RangeFrom};

/// private stuff for graph-building
impl Graph {
    fn new(cfg: Config) -> Graph {
        Graph {
            cfg,
            // nodes
            node_ids: Vec::new(),
            // node-metrics
            node_coords: Vec::new(),
            node_levels: Vec::new(),
            // edges
            fwd_dsts: Vec::new(),
            fwd_offsets: Vec::new(),
            fwd_to_fwd_map: Vec::new(),
            bwd_dsts: Vec::new(),
            bwd_offsets: Vec::new(),
            bwd_to_fwd_map: Vec::new(),
            // edge-metrics
            metrics: Vec::new(),
            // shortcuts (contraction-hierarchies)
            sc_offsets: Vec::new(),
            sc_edges: Vec::new(),
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
        self.sc_offsets.shrink_to_fit();
        self.sc_edges.shrink_to_fit();
    }

    /// Uses the graph's nodes, so nodes must have been added before this method works properly.
    /// The provided edge is interpreted as forward-edge.
    /// Metric-dependencies between each other are considered by looping enough times
    /// over the calculation-loop.
    fn add_metrics(&mut self, proto_edge: &mut ProtoEdgeB) -> Result<(), String> {
        let cfg = &self.cfg;

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
                            self.nodes().id(proto_edge.src_idx),
                            self.nodes().id(proto_edge.dst_idx),
                            category
                        );
                        *value = std::f64::EPSILON;
                    }
                    continue;
                }
                // now: proto-edge has no value for this metric and has to be updated

                // calculate metric dependent on category
                match category {
                    EdgeCategory::Meters => {
                        let src_coord = self.node_coords[*proto_edge.src_idx];
                        let dst_coord = self.node_coords[*proto_edge.dst_idx];
                        let distance = defaults::distance::TYPE::from(geo::haversine_distance_km(
                            &src_coord, &dst_coord,
                        ));
                        proto_edge.metrics[*metric_idx] = Some(*distance);
                    }
                    EdgeCategory::Seconds => {
                        // get distance and maxspeed to calculate duration
                        let mut raw_distance = None;
                        let mut raw_speed = None;
                        // get calculation-rules
                        for &(other_type, other_idx) in cfg.edges.calc_rules(metric_idx) {
                            // get values from edge dependent of calculation-rules
                            match other_type {
                                EdgeCategory::Meters => {
                                    raw_distance = proto_edge.metrics[*other_idx]
                                }
                                EdgeCategory::KilometersPerHour => {
                                    raw_speed = proto_edge.metrics[*other_idx];
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
                        if let (Some(raw_distance), Some(raw_speed)) = (raw_distance, raw_speed) {
                            let duration: defaults::time::TYPE =
                                (defaults::distance::TYPE::new(raw_distance)
                                    / defaults::speed::TYPE::new(raw_speed))
                                .into();
                            proto_edge.metrics[*metric_idx] = Some(*duration)
                        } else {
                            are_all_metrics_some = false;
                        }
                    }
                    EdgeCategory::KilometersPerHour => {
                        // get distance and duration to calculate maxspeed
                        let mut raw_distance = None;
                        let mut raw_duration = None;
                        // get calculation-rules
                        for &(other_type, other_idx) in cfg.edges.calc_rules(metric_idx) {
                            // get values from edge dependent of calculation-rules
                            match other_type {
                                EdgeCategory::Meters => {
                                    raw_distance = proto_edge.metrics[*other_idx]
                                }
                                EdgeCategory::Seconds => {
                                    raw_duration = proto_edge.metrics[*other_idx];
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
                        if let (Some(raw_distance), Some(raw_duration)) =
                            (raw_distance, raw_duration)
                        {
                            let maxspeed: defaults::speed::TYPE =
                                (defaults::distance::TYPE::new(raw_distance)
                                    / defaults::time::TYPE::new(raw_duration))
                                .into();
                            proto_edge.metrics[*metric_idx] = Some(*maxspeed)
                        } else {
                            are_all_metrics_some = false;
                        }
                    }
                    EdgeCategory::LaneCount
                    | EdgeCategory::Custom
                    | EdgeCategory::ShortcutEdgeIdx
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

#[derive(Debug)]
pub struct ProtoNode {
    pub id: i64,
    pub coord: Coordinate,
    pub level: Option<usize>,
}

pub struct ProtoShortcut {
    pub proto_edge: ProtoEdge,
    pub sc_edges: Option<[EdgeIdx; 2]>,
}

impl ProtoShortcut {
    /// Work off proto-edges in chunks to keep memory-usage lower.
    /// For example:
    /// To keep additional memory-needs below 1 MB, the the maximum amount of four f64-values per
    /// worked-off chunk has to be limited to 250_000.
    fn mem_size_b() -> usize {
        ProtoEdge::mem_size_b()
        // sc_edges: Option<[EdgeIdx; 2]>
        + 2 * mem::size_of::<EdgeIdx>()
    }
}

#[derive(Debug)]
pub struct ProtoEdge {
    pub src_id: i64,
    pub dst_id: i64,
    pub metrics: DimVec<Option<f64>>,
}

impl Into<ProtoShortcut> for ProtoEdge {
    fn into(self) -> ProtoShortcut {
        ProtoShortcut {
            proto_edge: self,
            sc_edges: None,
        }
    }
}

impl MemSize for ProtoEdge {
    /// Work off proto-edges in chunks to keep memory-usage lower.
    /// For example:
    /// To keep additional memory-needs below 1 MB, the the maximum amount of four f64-values per
    /// worked-off chunk has to be limited to 250_000.
    fn mem_size_b() -> usize {
        // src_id: i64
        // dst_id: i64
        2 * mem::size_of::<i64>()
        // metrics: DimVec<Option<f64>>
        + capacity::SMALL_VEC_INLINE_SIZE * mem::size_of::<f64>()
    }
}

struct ProtoEdgeA {
    pub idx: usize,
    pub src_id: i64,
    pub dst_id: i64,
    pub metrics: DimVec<Option<f64>>,
    pub sc_edges: Option<usize>,
}

struct ProtoEdgeB {
    pub idx: usize,
    pub src_idx: NodeIdx,
    pub dst_idx: NodeIdx,
    pub metrics: DimVec<Option<f64>>,
    pub sc_edges: Option<usize>,
}

impl MemSize for ProtoEdgeB {
    fn mem_size_b() -> usize {
        // idx
        mem::size_of::<usize>()
        // src_idx
        // dst_idx
        + 2 * mem::size_of::<usize>()
        // metrics
        + capacity::SMALL_VEC_INLINE_SIZE * mem::size_of::<f64>()
        // sc_edges
        + mem::size_of::<usize>()
    }
}

/// handy for remembering indices after sorting backwards
#[derive(Debug)]
struct ProtoEdgeC {
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    edge_idx: usize,
}

pub struct EdgeBuilder {
    cfg: Config,
    node_ids: Vec<i64>,
    proto_edges: Vec<ProtoEdgeA>,
    proto_shortcuts: Vec<[EdgeIdx; 2]>,
}

impl EdgeBuilder {
    pub fn cfg(&self) -> &Config {
        &self.cfg
    }

    pub fn insert<E>(&mut self, proto_edge: E)
    where
        E: Into<ProtoShortcut>,
    {
        let ProtoShortcut {
            proto_edge,
            sc_edges,
        } = proto_edge.into();

        // Most of the time, nodes are added for edges of one street,
        // so duplicates are next to each other.
        // Duplicates are removed later, but checking here saves a memory.
        // -> check k neighbours
        let n = self.node_ids.len();
        let k = 2;
        if n < k {
            self.node_ids.push(proto_edge.src_id);
            self.node_ids.push(proto_edge.dst_id);
        } else {
            for new_id in &[proto_edge.src_id, proto_edge.dst_id] {
                if !self.node_ids[(n - k)..n].contains(new_id) {
                    self.node_ids.push(*new_id);
                }
            }
        }

        // add edges
        let idx = self.proto_edges.len();
        if let Some(sc_edges) = sc_edges {
            // save index to shortcut in proto-edge
            self.proto_edges.push(ProtoEdgeA {
                idx,
                src_id: proto_edge.src_id,
                dst_id: proto_edge.dst_id,
                metrics: proto_edge.metrics,
                sc_edges: Some(self.proto_shortcuts.len()),
            });
            self.proto_shortcuts.push(sc_edges);
        } else {
            self.proto_edges.push(ProtoEdgeA {
                idx,
                src_id: proto_edge.src_id,
                dst_id: proto_edge.dst_id,
                metrics: proto_edge.metrics,
                sc_edges: None,
            });
        }
    }

    pub fn next(mut self) -> NodeBuilder {
        self.proto_edges.shrink_to_fit();
        self.proto_shortcuts.shrink_to_fit();

        // sort nodes, remove duplicates and shrink array since it can only shrink from now on
        self.node_ids.sort_unstable();
        self.node_ids.dedup();
        self.node_ids.shrink_to_fit();

        let mut node_coords = vec![None; self.node_ids.len()];
        node_coords.shrink_to_fit();
        let mut node_levels = vec![0; self.node_ids.len()];
        node_levels.shrink_to_fit();
        NodeBuilder {
            cfg: self.cfg,
            node_ids: self.node_ids,
            node_coords,
            node_levels,
            proto_edges: self.proto_edges,
            proto_shortcuts: self.proto_shortcuts,
        }
    }
}

pub struct NodeBuilder {
    cfg: Config,
    node_ids: Vec<i64>,
    node_coords: Vec<Option<Coordinate>>,
    node_levels: Vec<usize>,
    proto_edges: Vec<ProtoEdgeA>,
    proto_shortcuts: Vec<[EdgeIdx; 2]>,
}

impl NodeBuilder {
    pub fn cfg(&self) -> &Config {
        &self.cfg
    }

    /// Returns true if node is part of edge and hence has been added.
    pub fn insert(&mut self, proto_node: ProtoNode) -> bool {
        if let Ok(idx) = self.node_ids.binary_search(&proto_node.id) {
            self.node_coords[idx] = Some(proto_node.coord);
            if let Some(level) = proto_node.level {
                self.node_levels[idx] = level;
            }
            true
        } else {
            false
        }
    }

    pub fn next(self) -> Result<GraphBuilder, String> {
        Ok(GraphBuilder {
            cfg: self.cfg,
            node_ids: self.node_ids,
            node_coords: self.node_coords,
            node_levels: self.node_levels,
            proto_edges: self.proto_edges,
            proto_shortcuts: self.proto_shortcuts,
        })
    }
}

pub struct GraphBuilder {
    cfg: Config,
    node_ids: Vec<i64>,
    node_coords: Vec<Option<Coordinate>>,
    node_levels: Vec<usize>,
    proto_edges: Vec<ProtoEdgeA>,
    proto_shortcuts: Vec<[EdgeIdx; 2]>,
}

impl GraphBuilder {
    pub fn new(cfg: Config) -> EdgeBuilder {
        EdgeBuilder {
            cfg,
            node_ids: Vec::new(),
            proto_edges: Vec::new(),
            proto_shortcuts: Vec::new(),
        }
    }

    fn chunk_range(total_len: usize, max_chunk_size: usize) -> RangeFrom<usize> {
        if total_len > max_chunk_size {
            // ATTENTION! Splicing means that the given range is replaced,
            // hence max_chunk_size has to be, kind of, inverted.
            (total_len - max_chunk_size)..
        } else {
            0..
        }
    }

    pub fn finalize(mut self) -> Result<Graph, String> {
        //----------------------------------------------------------------------------------------//
        // init graph

        info!(
            "START Finalize graph with {} proto-nodes and {} proto-edges.",
            self.node_ids.len(),
            self.proto_edges.len()
        );
        let mut graph = Graph::new(self.cfg);

        //----------------------------------------------------------------------------------------//
        // add nodes to graph which belong to edges (sorted by asc id)

        info!("DO Check (sorted) nodes for existing coordinate.");
        {
            // check if every node has a coordinate, since every node is part of an edge
            for (idx, opt_coord) in self.node_coords.iter().enumerate() {
                if opt_coord.is_none() {
                    // should not happen if file is okay
                    return Err(format!(
                        "Proto-node (id: {}) has no coordinates, but belongs to an edge.",
                        self.node_ids[idx]
                    ));
                }
            }
            graph.node_ids = self.node_ids;
            graph.node_coords = self.node_coords.into_iter().map(Option::unwrap).collect();
            graph.node_levels = self.node_levels;
            graph.shrink_to_fit();
        }

        //----------------------------------------------------------------------------------------//
        // replace edges' node-ids by node-indizes for better performance

        info!("DO Replace edges' node-ids by node-indizes.");
        let mut proto_edges = {
            let nodes = graph.nodes();

            let mut new_proto_edges = vec![];

            let mut progress_bar = progressing::MappingBar::new(0..=self.proto_edges.len());

            // Work off proto-edges in chunks to keep memory-usage lower.
            let max_chunk_size = capacity::MAX_BYTE_PER_CHUNK / ProtoEdgeB::mem_size_b();
            debug!("max-chunk-size: {}", max_chunk_size);

            // sort reversed to make splice efficient
            self.proto_edges.reverse();
            while self.proto_edges.len() > 0 {
                // Get chunk from proto-edges.
                // Reverse chunk because proto-egdes is sorted reversed to make splice efficient.
                let chunk: Vec<_> = self
                    .proto_edges
                    .splice(
                        Self::chunk_range(self.proto_edges.len(), max_chunk_size),
                        vec![],
                    )
                    .rev()
                    .collect();

                // allocate new memory-needs
                self.proto_edges.shrink_to_fit();
                new_proto_edges.reserve_exact(chunk.len());
                debug!("chunk-len: {}", chunk.len());

                for edge in chunk.into_iter() {
                    new_proto_edges.push(ProtoEdgeB {
                        idx: edge.idx,
                        src_idx: nodes.idx_from(edge.src_id).expect(&format!(
                            "The given src-id `{:?}` doesn't exist as node",
                            edge.src_id
                        )),
                        dst_idx: nodes.idx_from(edge.dst_id).expect(&format!(
                            "The given dst-id `{:?}` doesn't exist as node",
                            edge.dst_id
                        )),
                        metrics: edge.metrics,
                        sc_edges: edge.sc_edges,
                    });

                    // print progress
                    progress_bar.add(1usize);
                    if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                        info!("{}", progress_bar);
                    }
                }
            }
            info!("{}", progress_bar.set(new_proto_edges.len()));
            // reduce and optimize memory-usage
            new_proto_edges.shrink_to_fit();

            new_proto_edges
        };

        //----------------------------------------------------------------------------------------//
        // sort forward-edges by ascending src-id, then by ascending dst-id -> offset-array

        info!("DO Sort proto-forward-edges by their src/dst-IDs.");
        {
            // - memory-peak is here when sorting
            // - sort by src-id, then level of dst, then dst-id
            //   -> branch prediction in dijkstra when breaking after level is reached
            let nodes = graph.nodes();
            proto_edges
                .sort_unstable_by_key(|e| (e.src_idx, Reverse(nodes.level(e.dst_idx)), e.dst_idx));
        }

        //----------------------------------------------------------------------------------------//
        // shortcuts: map usize to EdgeIdx
        // This has to be done before removing duplicates, because the usize-values depend on len()

        info!("DO Remap ch-shortcut-indices according to new sorted edges.");
        {
            // create mapping: old-idx -> new-idx
            let mut new_indices: Vec<usize> = vec![0; proto_edges.len()];
            proto_edges
                .iter()
                .enumerate()
                .for_each(|(new_idx, edge)| new_indices[edge.idx] = new_idx);
            // update shortcuts due to new sorted proto-edges
            for edge in proto_edges.iter() {
                if let Some(sc_idx) = edge.sc_edges {
                    let shortcuts = &mut self.proto_shortcuts[sc_idx];
                    shortcuts[0] = EdgeIdx(new_indices[*shortcuts[0]]);
                    shortcuts[1] = EdgeIdx(new_indices[*shortcuts[1]]);
                }
            }
        }

        //----------------------------------------------------------------------------------------//
        // remove duplicates
        // This should be done before doing metric to save memory.

        info!("DO Remove duplicated proto-edges and correct remaining ch-shortcuts");
        // count shortcut-edges for later
        let mut sc_count = 0;
        {
            // duplicate is e.g. the edge
            // node-id 314074041 -> node-id 283494218
            // which is part of two ways

            let mut removed_indices = Vec::new();

            let mut w = 1;
            for r in 1..proto_edges.len() {
                // compare edge[w-1] and edge[r]
                let is_duplicate = {
                    let e0 = &proto_edges[w - 1];
                    let e1 = &proto_edges[r];
                    let mut is_eq = true;

                    // compare src-id and dst-id, then metrics approximately
                    if (e0.src_idx, e0.dst_idx) == (e1.src_idx, e1.dst_idx) {
                        for (e0_opt, e1_opt) in e0.metrics.iter().zip(e1.metrics.iter()) {
                            if e0_opt.is_none() && e1_opt.is_none() {
                                // both are none
                                continue;
                            } else {
                                if let (Some(e0_metric), Some(e1_metric)) = (e0_opt, e1_opt) {
                                    // both are some
                                    if e0_metric.approx_eq(&e1_metric) {
                                        continue;
                                    }
                                }
                                // both are different
                                is_eq = false;
                                break;
                            }
                        }
                    } else {
                        is_eq = false;
                    }

                    is_eq
                };

                // if duplicate
                // -> inc r
                // -> remember index for updating shortcuts
                if is_duplicate {
                    removed_indices.push(r);
                }
                // if not a duplicate
                // -> swap edge[w] and edge[r]
                // -> inc w and inc r
                else {
                    proto_edges.swap(w, r);
                    w += 1;
                }
            }

            proto_edges.truncate(proto_edges.len() - removed_indices.len());

            // correct remaining shortcuts
            // -> decrement every index, that is at least as high as a removed-idx
            for edge in proto_edges.iter() {
                if let Some(sc_idx) = edge.sc_edges {
                    let shortcuts = &mut self.proto_shortcuts[sc_idx];
                    sc_count += 1;
                    for removed_idx in removed_indices.iter() {
                        for shortcut in shortcuts.iter_mut().filter(|sc| ***sc >= *removed_idx) {
                            **shortcut -= 1;
                        }
                    }
                }
            }
            info!("Removed {} duplicates.", removed_indices.len());
        }

        //----------------------------------------------------------------------------------------//
        // build metrics
        // If metrics are built before indices and offsets are built, the need of memory while
        // building is reduced.

        info!("START Create/store/filter metrics.");
        let mut new_sc_edges = Vec::with_capacity(sc_count);
        let mut proto_edges = {
            let mut new_proto_edges = vec![];

            let mut progress_bar = progressing::MappingBar::new(0..=proto_edges.len());
            let mut edge_idx: usize = 0;

            // Work off proto-edges in chunks to keep memory-usage lower.
            let max_chunk_size = capacity::MAX_BYTE_PER_CHUNK / ProtoShortcut::mem_size_b();
            debug!("max-chunk-size: {}", max_chunk_size);
            // init metrics
            graph.metrics = vec![vec![]; graph.cfg().edges.dim()];
            debug!(
                "initial graph-metric-capacity: {}",
                graph.metrics[0].capacity()
            );

            // sort reversed to make splice efficient
            proto_edges.reverse();
            while proto_edges.len() > 0 {
                // Get chunk from proto-edges.
                // Reverse chunk because proto-egdes is sorted reversed to make splice efficient.
                let chunk: Vec<_> = proto_edges
                    .splice(Self::chunk_range(proto_edges.len(), max_chunk_size), vec![])
                    .rev()
                    .collect();

                // allocate new memory-needs
                proto_edges.shrink_to_fit();
                graph
                    .metrics
                    .iter_mut()
                    .for_each(|m| m.reserve_exact(chunk.len()));
                new_proto_edges.reserve_exact(chunk.len());
                debug!("chunk-len: {}", chunk.len());
                debug!("graph-metric-capacity: {}", graph.metrics[0].capacity());

                for mut edge in chunk.into_iter() {
                    // add to graph and remember ids
                    // -> nodes are needed to map NodeId -> NodeIdx
                    graph.add_metrics(&mut edge)?;
                    new_proto_edges.push(ProtoEdgeC {
                        src_idx: edge.src_idx,
                        dst_idx: edge.dst_idx,
                        edge_idx: 0, // used later for offset-arrays
                    });

                    // remember sc-edges for setting offsets later
                    if let Some(sc_idx) = edge.sc_edges {
                        new_sc_edges.push((edge_idx, self.proto_shortcuts[sc_idx]));
                    }

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
            // reduce and optimize memory-usage
            graph.shrink_to_fit();
            new_proto_edges.shrink_to_fit();
            // last node needs an upper bound as well for `leaving_edges(...)`

            new_proto_edges
        };

        //----------------------------------------------------------------------------------------//
        // set ch-shortcut-offsets
        // do it here to reduce memory-needs by processing metrics first

        info!("DO Create ch-shortcut-offsets-array");
        {
            graph.sc_offsets = vec![new_sc_edges.len(); proto_edges.len() + 1];
            graph.sc_edges = Vec::with_capacity(sc_count);
            let mut sc_offset = 0;
            for edge_idx in 0..proto_edges.len() {
                // Since sc-offsets have been initialized with the last offset,
                // everything is already correct when this point is reached.
                if sc_offset == new_sc_edges.len() {
                    break;
                }

                let (sc_edge_idx, sc_edges) = new_sc_edges[sc_offset];

                // update shortcut-offset
                graph.sc_offsets[edge_idx] = sc_offset;

                // if this was a shortcut-edge
                // -> increase offset for next edges
                if edge_idx == sc_edge_idx {
                    graph.sc_edges.push(sc_edges);
                    sc_offset += 1;
                }
            }
        }

        //----------------------------------------------------------------------------------------//
        // build forward-offset-array and edges

        // logging
        info!("START Create the forward-offset-array and the forward-mapping.");
        {
            let mut progress_bar = progressing::MappingBar::new(0..=proto_edges.len());
            // start looping
            let mut src_idx = NodeIdx(0);
            let mut offset = 0;
            graph.fwd_offsets.push(offset);
            // high-level-idea
            // count offset for each proto_edge (sorted) and apply offset as far as src doesn't change
            let mut edge_idx = 0;
            for proto_edge in proto_edges.iter_mut() {
                // Add edge-idx here to remember it for indirect mapping bwd->fwd.
                // Update it at the end of the loop.
                proto_edge.edge_idx = edge_idx;

                // do not swap src and dst since this is a forward-edge
                let edge_src_idx = proto_edge.src_idx;
                let edge_dst_idx = proto_edge.dst_idx;

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
        }

        //----------------------------------------------------------------------------------------//
        // sort backward-edges by ascending dst-id, then by ascending src-id -> offset-array

        info!("DO Sort proto-backward-edges by their dst/src-IDs.");
        {
            let nodes = graph.nodes();
            proto_edges.sort_unstable_by_key(|edge| {
                (
                    edge.dst_idx,
                    Reverse(nodes.level(edge.src_idx)),
                    edge.src_idx,
                )
            });
        }

        //----------------------------------------------------------------------------------------//
        // build backward-offset-array

        info!("START Create the backward-offset-array.");
        {
            let mut progress_bar = progressing::MappingBar::new(0..=proto_edges.len());
            // start looping
            let mut src_idx = NodeIdx(0);
            let mut offset = 0;
            graph.bwd_offsets.push(offset);
            // high-level-idea
            // count offset for each proto_edge (sorted) and apply offset as far as src doesn't change
            for edge_idx in 0..proto_edges.len() {
                let proto_edge = &mut proto_edges[edge_idx];

                // swap src and dst since this is the backward-edge
                let edge_src_idx = proto_edge.dst_idx;

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
                graph.bwd_to_fwd_map.push(EdgeIdx(proto_edge.edge_idx));

                // print progress
                progress_bar.set(edge_idx);
                if progress_bar.progress() % (1 + (progress_bar.end() / 10)) == 0 {
                    info!("{}", progress_bar);
                }
            }
            // last node needs an upper bound as well for `leaving_edges(...)`
            debug_assert_eq!(
                offset,
                proto_edges.len(),
                "Last offset-value should be as big as the number of proto-edges."
            );
            graph.bwd_offsets.push(offset);
            info!("{}", progress_bar.set(graph.fwd_dsts.len()));
            // reduce and optimize memory-usage
            graph.shrink_to_fit();
        }

        info!("FINISHED Finalizing graph has finished.");
        Ok(graph)
    }
}
