use super::{EdgeIdx, Graph, NodeIdx};
use crate::{
    configs::parsing::{self, generating},
    defaults::{
        self,
        capacity::{self, DimVec},
        routing::IS_USING_CH_LEVEL_SPEEDUP,
    },
    helpers::{approx::ApproxEq, err, MemSize},
};
use kissunits::geo::Coordinate;
use log::{debug, info};
use progressing::{Bar, MappingBar};
use std::{cmp::Reverse, mem, ops::RangeFrom};

/// private stuff for graph-building
impl Graph {
    fn new(cfg: parsing::Config) -> Graph {
        Graph {
            cfg,
            // nodes
            node_ids: Vec::new(),
            // node-metrics
            node_coords: Vec::new(),
            node_ch_levels: Vec::new(),
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

    /// The provided edge is interpreted as forward-edge.
    fn add_metrics(&mut self, proto_edge: &mut ProtoEdgeB) -> err::Feedback {
        let cfg = &self.cfg;

        for (metric_idx, raw_value) in proto_edge.metrics.iter_mut().enumerate() {
            if raw_value.approx_eq(&0.0) {
                debug!(
                    "Proto-edge (id:{}->id:{}) has {}=0, hence is corrected to epsilon.",
                    self.nodes().id(proto_edge.src_idx),
                    self.nodes().id(proto_edge.dst_idx),
                    cfg.edges.metrics.ids[metric_idx]
                );
                *raw_value = std::f64::EPSILON;
            }
        }

        self.metrics.push(proto_edge.metrics.clone());

        Ok(())
    }
}

#[derive(Debug)]
pub struct ProtoNode {
    pub id: i64,
    pub coord: Coordinate,
    pub ch_level: Option<usize>,
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
    pub metrics: DimVec<f64>,
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
    pub metrics: DimVec<f64>,
    pub sc_edges: Option<usize>,
}

struct ProtoEdgeB {
    pub idx: usize,
    pub src_idx: NodeIdx,
    pub dst_idx: NodeIdx,
    pub metrics: DimVec<f64>,
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
    cfg: parsing::Config,
    node_ids: Vec<i64>,
    proto_edges: Vec<ProtoEdgeA>,
    proto_shortcuts: Vec<[EdgeIdx; 2]>,
}

impl EdgeBuilder {
    pub fn cfg(&self) -> &parsing::Config {
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
        let mut node_ch_levels = vec![defaults::network::nodes::LEVEL; self.node_ids.len()];
        node_ch_levels.shrink_to_fit();
        NodeBuilder {
            cfg: self.cfg,
            node_ids: self.node_ids,
            node_coords,
            node_ch_levels,
            proto_edges: self.proto_edges,
            proto_shortcuts: self.proto_shortcuts,
        }
    }
}

pub struct NodeBuilder {
    cfg: parsing::Config,
    node_ids: Vec<i64>,
    node_coords: Vec<Option<Coordinate>>,
    node_ch_levels: Vec<usize>,
    proto_edges: Vec<ProtoEdgeA>,
    proto_shortcuts: Vec<[EdgeIdx; 2]>,
}

impl NodeBuilder {
    pub fn cfg(&self) -> &parsing::Config {
        &self.cfg
    }

    /// Returns true if node is part of edge and hence has been added.
    pub fn insert(&mut self, proto_node: ProtoNode) -> bool {
        if let Ok(idx) = self.node_ids.binary_search(&proto_node.id) {
            self.node_coords[idx] = Some(proto_node.coord);
            if let Some(ch_level) = proto_node.ch_level {
                self.node_ch_levels[idx] = ch_level;
            }
            true
        } else {
            false
        }
    }

    pub fn next(self) -> err::Result<GraphBuilder> {
        Ok(GraphBuilder {
            cfg: self.cfg,
            node_ids: self.node_ids,
            node_coords: self.node_coords,
            node_ch_levels: self.node_ch_levels,
            proto_edges: self.proto_edges,
            proto_shortcuts: self.proto_shortcuts,
        })
    }
}

pub struct GraphBuilder {
    cfg: parsing::Config,
    node_ids: Vec<i64>,
    node_coords: Vec<Option<Coordinate>>,
    node_ch_levels: Vec<usize>,
    proto_edges: Vec<ProtoEdgeA>,
    proto_shortcuts: Vec<[EdgeIdx; 2]>,
}

impl GraphBuilder {
    pub fn new(cfg: parsing::Config) -> EdgeBuilder {
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

    pub fn finalize(mut self) -> err::Result<Graph> {
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
                    )
                    .into());
                }
            }
            graph.node_ids = self.node_ids;
            graph.node_coords = self.node_coords.into_iter().map(Option::unwrap).collect();
            graph.node_ch_levels = self.node_ch_levels;
            graph.shrink_to_fit();
        }

        //----------------------------------------------------------------------------------------//
        // replace edges' node-ids by node-indizes for better performance

        info!("DO Replace edges' node-ids by node-indizes.");
        let mut proto_edges = {
            let nodes = graph.nodes();

            let mut new_proto_edges = vec![];

            let mut progress_bar = MappingBar::new(0..=self.proto_edges.len());

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
            progress_bar.set(new_proto_edges.len());
            info!("{}", progress_bar);
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
            if !IS_USING_CH_LEVEL_SPEEDUP {
                proto_edges.sort_by_key(|e| (e.src_idx, e.dst_idx));
            } else {
                let nodes = graph.nodes();
                proto_edges
                    .sort_by_key(|e| (e.src_idx, Reverse(nodes.level(e.dst_idx)), e.dst_idx));
            }
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
                        for (e0_metric, e1_metric) in e0.metrics.iter().zip(e1.metrics.iter()) {
                            if e0_metric.approx_eq(&e1_metric) {
                                continue;
                            }
                            // values are different
                            is_eq = false;
                            break;
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
                    // replace r by w-1
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
            // This works because the original list has been sorted, meaning duplicates are laying
            // next to each other.
            // Thus decrementing corrects every value.
            for edge in proto_edges.iter() {
                if let Some(sc_idx) = edge.sc_edges {
                    let shortcuts = &mut self.proto_shortcuts[sc_idx];
                    sc_count += 1;
                    for removed_idx in removed_indices.iter().rev() {
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

        info!("START Store metrics.");
        let mut new_sc_edges = Vec::with_capacity(sc_count);
        let mut proto_edges = {
            let mut new_proto_edges = vec![];

            let mut progress_bar = MappingBar::new(0..=proto_edges.len());
            let mut edge_idx: usize = 0;

            // Work off proto-edges in chunks to keep memory-usage lower.
            let max_chunk_size = capacity::MAX_BYTE_PER_CHUNK / ProtoShortcut::mem_size_b();
            debug!("max-chunk-size: {}", max_chunk_size);
            // init metrics
            graph.metrics = Vec::new();
            debug!(
                "initial graph-metric-capacity: {}",
                graph.metrics.capacity()
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
                graph.metrics.reserve_exact(chunk.len());
                new_proto_edges.reserve_exact(chunk.len());
                debug!("chunk-len: {}", chunk.len());
                debug!("graph-metric-capacity: {}", graph.metrics.capacity());

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
            progress_bar.set(edge_idx);
            info!("{}", progress_bar);
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
            let mut progress_bar = MappingBar::new(0..=proto_edges.len());
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
            progress_bar.set(offset);
            info!("{}", progress_bar);
            // reduce and optimize memory-usage
            // already dropped via iterator: drop(self.proto_edges);
            graph.shrink_to_fit();
        }

        //----------------------------------------------------------------------------------------//
        // sort backward-edges by ascending dst-id, then by ascending src-id -> offset-array

        info!("DO Sort proto-backward-edges by their dst/src-IDs.");
        {
            if !IS_USING_CH_LEVEL_SPEEDUP {
                proto_edges.sort_by_key(|edge| (edge.dst_idx, edge.src_idx));
            } else {
                let nodes = graph.nodes();
                proto_edges.sort_by_key(|edge| {
                    (
                        edge.dst_idx,
                        Reverse(nodes.level(edge.src_idx)),
                        edge.src_idx,
                    )
                });
            }
        }

        //----------------------------------------------------------------------------------------//
        // build backward-offset-array

        info!("START Create the backward-offset-array.");
        {
            let mut progress_bar = MappingBar::new(0..=proto_edges.len());
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
            progress_bar.set(graph.fwd_dsts.len());
            info!("{}", progress_bar);
            // reduce and optimize memory-usage
            graph.shrink_to_fit();
        }

        //----------------------------------------------------------------------------------------//
        // generate new metrics

        info!("START Create and convert metrics.");
        if let Some(generating_cfg) = graph.cfg.generating.take() {
            // nodes

            for category in generating_cfg.nodes.categories.iter() {
                match category {
                    generating::nodes::Category::Meta { info, id: new_id } => {
                        match info {
                            generating::nodes::MetaInfo::NodeIdx => {
                                // if id does already exist
                                // -> error

                                if graph.cfg.nodes.categories.iter().any(
                                    |category| match category {
                                        parsing::nodes::Category::Meta { info: _, id }
                                        | parsing::nodes::Category::Metric { unit: _, id } => {
                                            new_id == id
                                        }
                                        parsing::nodes::Category::Ignored => false,
                                    },
                                ) {
                                    return Err(format!(
                                        "Node-meta-info {:?} has id {}, which does already exist.",
                                        info, new_id
                                    )
                                    .into());
                                }

                                // add new category

                                graph.cfg.nodes.categories.push(category.clone().into());
                            }
                            generating::nodes::MetaInfo::NodeId
                            | generating::nodes::MetaInfo::CHLevel => {
                                return Err(format!(
                                    "Node-meta-info {:?} (id: {}) cannot be created \
                                     and has to be provided.",
                                    info, new_id
                                )
                                .into())
                            }
                        }
                    }
                }
            }

            // edges

            // check duplicated id

            for category in generating_cfg.edges.categories.iter() {
                match category {
                    generating::edges::Category::Meta {
                        info: _,
                        id: new_id,
                    }
                    | generating::edges::Category::Copy {
                        from: _,
                        to:
                            generating::edges::metrics::Category {
                                unit: _,
                                id: new_id,
                            },
                    }
                    | generating::edges::Category::Calc {
                        a: _,
                        b: _,
                        result:
                            generating::edges::metrics::Category {
                                unit: _,
                                id: new_id,
                            },
                    }
                    | generating::edges::Category::Custom {
                        unit: _,
                        id: new_id,
                        default: _,
                    }
                    | generating::edges::Category::Haversine {
                        unit: _,
                        id: new_id,
                    } => {
                        // if id does already exist
                        // -> error

                        if graph
                            .cfg
                            .edges
                            .categories
                            .iter()
                            .any(|category| match category {
                                parsing::edges::Category::Meta { info: _, id }
                                | parsing::edges::Category::Metric { unit: _, id } => new_id == id,
                                parsing::edges::Category::Ignored => false,
                            })
                        {
                            return Err(format!(
                                "Id {} should be generated, but does already exist.",
                                new_id
                            )
                            .into());
                        }
                    }
                    generating::edges::Category::Convert { from: _, to: _ } => {
                        // do not check because it's in-place, so duplicates would be removed.
                    }
                }
            }

            // add new data

            for category in generating_cfg.edges.categories.iter() {
                match category {
                    generating::edges::Category::Meta { info, id: new_id } => {
                        match info {
                            generating::edges::MetaInfo::SrcIdx
                            | generating::edges::MetaInfo::DstIdx => {
                                // update graph
                                //
                                // -> already done

                                // update config

                                graph.cfg.edges.categories.push(category.clone().into());
                            }
                            generating::edges::MetaInfo::ShortcutIdx0
                            | generating::edges::MetaInfo::ShortcutIdx1 => {
                                return Err(format!(
                                    "Edge-meta-info {:?} (id: {}) cannot be created \
                                     and has to be provided.",
                                    info, new_id
                                )
                                .into())
                            }
                        }
                    }
                    generating::edges::Category::Custom { unit, id, default } => {
                        // update graph

                        graph
                            .metrics
                            .iter_mut()
                            .for_each(|metric| metric.push(*default));

                        // update config

                        graph.cfg.edges.categories.push(category.clone().into());
                        graph.cfg.edges.metrics.units.push((*unit).into());
                        graph.cfg.edges.metrics.ids.push(id.clone());
                    }
                    generating::edges::Category::Haversine { unit, id } => {
                        // check unit

                        if !match unit {
                            generating::edges::metrics::UnitInfo::Meters => false,
                            generating::edges::metrics::UnitInfo::Kilometers => true,
                            generating::edges::metrics::UnitInfo::Seconds => false,
                            generating::edges::metrics::UnitInfo::Minutes => false,
                            generating::edges::metrics::UnitInfo::Hours => false,
                            generating::edges::metrics::UnitInfo::KilometersPerHour => false,
                            generating::edges::metrics::UnitInfo::LaneCount => false,
                            generating::edges::metrics::UnitInfo::F64 => false,
                        } {
                            return Err(format!(
                                "Haversine creates {:?}, but you may convert \
                                the resulting value afterwards.",
                                generating::edges::metrics::UnitInfo::Kilometers
                            )
                            .into());
                        }

                        // calculate haversine-distance and update graph and config

                        for edge_idx in (0..graph.metrics.len()).map(EdgeIdx) {
                            // get positions

                            let (src_coord, dst_coord) = {
                                let src_idx = graph.bwd_edges().dst_idx(edge_idx);
                                let dst_idx = graph.fwd_edges().dst_idx(edge_idx);
                                let nodes = graph.nodes();
                                (nodes.coord(src_idx), nodes.coord(dst_idx))
                            };

                            // calculate distance
                            let distance = {
                                let km =
                                    kissunits::geo::haversine_distance_km(&src_coord, &dst_coord);
                                generating::edges::metrics::UnitInfo::Kilometers
                                    .try_convert(unit, *km)?
                            };

                            // update graph

                            graph.metrics[*edge_idx].push(distance);
                        }

                        // update config

                        graph.cfg.edges.categories.push(category.clone().into());
                        graph.cfg.edges.metrics.units.push((*unit).into());
                        graph.cfg.edges.metrics.ids.push(id.clone());
                    }
                    generating::edges::Category::Copy { from, to } => {
                        // loop over all edges
                        // and add to their metrics

                        let metric_idx = graph
                            .cfg
                            .edges
                            .metrics
                            .ids
                            .iter()
                            .position(|id| id == &from.id)
                            .expect(&format!(
                                "Id {} is expected in graph, but doesn't exist.",
                                from.id
                            ));
                        for edge_idx in 0..graph.metrics.len() {
                            // get old value
                            // and generate new value

                            let new_raw_value = {
                                let old_raw_value = graph.metrics[edge_idx][metric_idx];
                                from.unit.try_convert(&to.unit, old_raw_value)?
                            };

                            // update graph

                            graph.metrics[edge_idx].push(new_raw_value);
                        }

                        // update config

                        graph.cfg.edges.categories.push(category.clone().into());
                        graph.cfg.edges.metrics.units.push(to.unit.into());
                        graph.cfg.edges.metrics.ids.push(to.id.clone());
                    }
                    generating::edges::Category::Convert { from, to } => {
                        // loop over all edges
                        // and replace their existing metrics

                        let metric_idx = graph
                            .cfg
                            .edges
                            .metrics
                            .ids
                            .iter()
                            .position(|id| id == &from.id)
                            .expect(&format!(
                                "Id {} is expected in graph, but doesn't exist.",
                                from.id
                            ));
                        for edge_idx in 0..graph.metrics.len() {
                            // get old value
                            // and generate new value

                            let new_raw_value = {
                                let old_raw_value = graph.metrics[edge_idx][metric_idx];
                                from.unit.try_convert(&to.unit, old_raw_value)?
                            };

                            // update graph

                            graph.metrics[edge_idx][metric_idx] = new_raw_value;
                        }

                        // update config

                        graph
                            .cfg
                            .edges
                            .categories
                            .iter_mut()
                            .for_each(|category| match category {
                                parsing::edges::Category::Metric {
                                    unit: old_unit,
                                    id: old_id,
                                } => {
                                    if old_id == &from.id {
                                        *old_unit = to.unit.into();
                                        *old_id = to.id.clone();
                                    }
                                }
                                parsing::edges::Category::Meta { info: _, id: _ }
                                | parsing::edges::Category::Ignored => (),
                            });
                        graph.cfg.edges.metrics.units[metric_idx] = to.unit.into();
                        graph.cfg.edges.metrics.ids[metric_idx] = to.id.clone();
                    }
                    generating::edges::Category::Calc { result, a, b } => {
                        // loop over all edges
                        // and replace their existing metrics

                        let idx_a = graph
                            .cfg
                            .edges
                            .metrics
                            .ids
                            .iter()
                            .position(|id| id == &a.id)
                            .expect(&format!(
                                "Id {} is expected in graph, but doesn't exist.",
                                a.id
                            ));
                        let idx_b = graph
                            .cfg
                            .edges
                            .metrics
                            .ids
                            .iter()
                            .position(|id| id == &b.id)
                            .expect(&format!(
                                "Id {} is expected in graph, but doesn't exist.",
                                b.id
                            ));
                        for edge_idx in 0..graph.metrics.len() {
                            // get old value
                            // and generate new value

                            let new_raw_value = {
                                let old_raw_a = graph.metrics[edge_idx][idx_a];
                                let old_raw_b = graph.metrics[edge_idx][idx_b];
                                result
                                    .unit
                                    .try_calc(&a.unit, old_raw_a, &b.unit, old_raw_b)?
                            };

                            // update graph

                            graph.metrics[edge_idx].push(new_raw_value);
                        }

                        // update config

                        graph.cfg.edges.categories.push(category.clone().into());
                        graph.cfg.edges.metrics.units.push(result.unit.into());
                        graph.cfg.edges.metrics.ids.push(result.id.clone());
                    }
                }
            }
        }

        info!("FINISHED Finalizing graph has finished.");
        Ok(graph)
    }
}
