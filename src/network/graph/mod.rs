pub mod building;
use building::ProtoEdge;
mod indexing;
pub use indexing::{EdgeIdx, MetricIdx, NodeIdx};

use crate::{
    configs,
    configs::{graph, MetricType},
    units::{
        geo, geo::Coordinate, length::Meters, speed::KilometersPerHour, time::Milliseconds, Metric,
        MetricU32,
    },
};
use std::{cmp::max, fmt, fmt::Display};

/// Stores graph-data as offset-graph in arrays and provides methods and shallow structs for accessing them.
///
///
/// # Structure
///
/// These structs are called shallow, because they contain only references to the graph's data, like a layer collecting quick-access-methods to the data.
///
/// Though the graph provides access to forward- and backward-edges, all data is stored wrt a forward-graph, meaning metrics are stored for edges, that have been sorted by (src-id, dst-id) in ascending order, with src-id having precedence over dst-id.
/// Exception is the offset-array, as explained in the following.
///
///
/// ## Offset-graph in general
///
/// An offset-graph fulfills the needs of quick component accesses.
/// Following picture shows a small example of the data-structure.
///
/// ```text
/// 4 --- 3 --- 1 --- 0
/// |     |     |
/// +---- 2 ----+
///
/// nodes               0    1      2      3     4    _
/// edge destinations   1  0 2 3  1 3 4  1 2 4  2 3   _
/// offset              0    1      4      7    10   12
/// ```
///
/// If all edges of node `i` are needed, one can look in the offset-array at index `i` and `i+1` to get the range `k`, where the edge-array contains all edges of node `i`.
/// Since `i+1` could lead into an out-of-bounds-exception, the offset-array has one last entry in addition.
///
/// ```text
/// node i=2
/// -> range k=(4..7)
/// -> edges (2->1), (2->3), (2->4)
/// ```
///
///
/// ## Modifications for accessing backward-edges
///
/// Let's remove the edge `1->2` from the graph above to make it partially directed (for asymmetry).
/// The following table shows the resulting graph, all its forward-edges (sorted by src, than dst) and the respective backward-edges (sorted by dst, than src).
///
/// ```text
/// |            |               fwd               |               bwd               |
/// |------------|---------------------------------|---------------------------------|
/// | src        | 0  -1-  --2--  ---3--  --4-   - | 0  --1--  -2-  ---3--  --4-   - |
/// | dst        | 1  0 3  1 3 4  1 2  4  2  3   - | 1  0 2 3  3 4  1 2  4  2  3   - |
/// | metric     | a  b c  d e f  g h  i  j  k   - | b  a d g  h j  c e  k  f  i   - |
/// |            |                                 |                                 |
/// | to-fwd-idx | 0  1 2  3 4 5  6 7  8  9 10   - | 1  0 3 6  7 9  2 4 10  5  8   - |
/// | to-bwd-idx | 1  0 6  2 7 9  3 4 10  5  8   - | 0  1 2 3  4 5  6 7  8  9 10   - |
/// |            |                                 |                                 |
/// | fwd-offset | 0  -1-  --3--  ---6--  --9-  11 |                                 |
/// | bwd-offset |                                 | 0  --1--  -4-  --6---  --9-  11 |
/// ```
///
/// > Note that either to-fwd-idx-array or to-bwd-idx-array is needed.
///
/// However, for quick, direct access to all components (independent of a fwd-/bwd-mapping), the graph should be reordered with focus on the same indices.
/// It is more intuitive to order everything according to the fwd-graph, so in the following, everything is explained for this particular case.
///
/// Remaining problem are the offsets in the bwd-offset-array, since they refer to the specific src-node in an array sorted primarily by src (which is different for fwd and bwd).
/// Further, when asking for leaving-edges of src-idx `i`, in addition to `offset[i]` also `offset[i+1]` is needed.
///
/// Solution is keeping the respective fwd- and bwd-offset-arrays and when accessing them, map the resulting slices with the to-fwd-idx-array to the fwd-dst-array, which are stored intuitively according to the fwd-graph.
#[derive(Debug)]
pub struct Graph {
    // nodes
    node_ids: Vec<i64>,
    // node-metrics
    node_coords: Vec<Coordinate>,
    // edges: offset-graph and mappings, e.g. for metrics
    fwd_dsts: Vec<NodeIdx>,
    fwd_offsets: Vec<usize>,
    fwd_to_fwd_map: Vec<EdgeIdx>,
    bwd_dsts: Vec<NodeIdx>,
    bwd_offsets: Vec<usize>,
    bwd_to_fwd_map: Vec<EdgeIdx>,
    // edge-metrics (sorted according to fwd_dsts)
    metric_ids: Vec<String>,
    metrics: Vec<Vec<MetricU32>>,
}

impl Default for Graph {
    fn default() -> Graph {
        Graph {
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
            metric_ids: Vec::new(),
            metrics: Vec::new(),
        }
    }
}

/// private stuff for graph-building
impl Graph {
    fn new() -> Graph {
        Graph {
            ..Default::default()
        }
    }

    fn init_metrics(&mut self, cfg: &graph::Config, capacity: usize) {
        for metric_type in cfg.edges.metric_types.iter() {
            match metric_type {
                MetricType::Length { provided: _ }
                | MetricType::Duration { provided: _ }
                | MetricType::Maxspeed { provided: _ }
                | MetricType::LaneCount
                | MetricType::Custom { id: _ } => {
                    self.metric_ids.push(String::from(metric_type.id()));
                    self.metrics.push(Vec::with_capacity(capacity));
                }
                MetricType::Id { id: _ } | MetricType::Ignore { id: _ } => (),
            }
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
        self.metric_ids.shrink_to_fit();
        self.metrics.shrink_to_fit();
    }

    /// Uses the graph's nodes, so nodes must have been added before this method works properly.
    /// The provided edge is used as forward-edge.
    fn add_metrics(
        &mut self,
        proto_edge: &mut ProtoEdge,
        cfg: &graph::Config,
    ) -> Result<(), String> {
        let (src_idx, dst_idx) = {
            let nodes = self.nodes();
            let src_idx = nodes.src_idx_from(*proto_edge.src_id())?;
            let dst_idx = nodes.dst_idx_from(*proto_edge.dst_id())?;
            (src_idx, dst_idx)
        };

        // Repeat the calculations n times.
        // In worst case, no metric is provided and all have to be calculated sequentially.
        let mut is_metric_processed = vec![false; cfg.edges.metric_types.len()];
        for _ in 0..cfg.edges.metric_types.len() {
            // add metrics to graph
            // For every metric holds: if graph expects metric, it should be provided explicetly or implicitly.
            // For calculating metrics from other metrics, multiple loop-runs are necessary.
            for i in 0..cfg.edges.metric_types.len() {
                // calc every metric only once
                if is_metric_processed[i] {
                    continue;
                }

                let metric_type = &cfg.edges.metric_types[i];

                // get metric-vec (if metric has to be stored)
                let metric_vec = {
                    match metric_type {
                        MetricType::Length { provided: _ }
                        | MetricType::Duration { provided: _ }
                        | MetricType::Maxspeed { provided: _ }
                        | MetricType::LaneCount
                        | MetricType::Custom { id: _ } => (),
                        MetricType::Id { id: _ } | MetricType::Ignore { id: _ } => {
                            is_metric_processed[i] = true;
                            continue;
                        }
                    }
                    // Should always return ok, since graph should be initialized properly
                    // and special metric-types (e.g. ignored) are filtered above.
                    let metric_vec_idx = self.metrics().metric_vec_idx(metric_type.id())?;
                    &mut self.metrics[metric_vec_idx]
                };

                match metric_type {
                    &MetricType::Length { provided } => {
                        // If length is not provided -> calculate it by coordinates and haversine.
                        let length = {
                            if !provided {
                                let src_coord = self.node_coords[src_idx.to_usize()];
                                let dst_coord = self.node_coords[dst_idx.to_usize()];
                                geo::haversine_distance_m(&src_coord, &dst_coord).into()
                            } else {
                                *(proto_edge.metric(metric_type.id()).ok_or(format!(
                                    "Metric {} should be provided, but isn't.",
                                    MetricType::Length { provided }
                                ))?)
                            }
                        };

                        // Length should be at least 1 to prevent errors, e.g. when dividing.
                        let length = max(MetricU32::from(1u32), length).into();
                        proto_edge.add_metric(metric_type.id(), length);
                        metric_vec.push(length);
                        is_metric_processed[i] = true;
                    }
                    &MetricType::Duration { provided } => {
                        // If not provided, but expected in graph
                        // -> calc with length and maxspeed below
                        let duration = {
                            if !provided {
                                let length = proto_edge.metric(configs::constants::ids::LENGTH);
                                let maxspeed = proto_edge.metric(configs::constants::ids::MAXSPEED);
                                if let (Some(length), Some(maxspeed)) = (length, maxspeed) {
                                    Meters::from(*length) / KilometersPerHour::from(*maxspeed)
                                } else {
                                    continue;
                                }
                            } else {
                                Milliseconds::from(
                                    *(proto_edge.metric(metric_type.id()).ok_or(format!(
                                        "Metric {} should be provided, but isn't.",
                                        MetricType::Duration { provided }
                                    ))?),
                                )
                            }
                        };

                        let duration = duration.into();
                        proto_edge.add_metric(metric_type.id(), duration);
                        metric_vec.push(duration);
                        is_metric_processed[i] = true;
                    }
                    &MetricType::Maxspeed { provided } => {
                        // If not provided, but expected in graph
                        // -> calc with length and duration below
                        let maxspeed = {
                            if !provided {
                                let length = proto_edge.metric(configs::constants::ids::LENGTH);
                                let duration = proto_edge.metric(configs::constants::ids::DURATION);
                                if let (Some(length), Some(duration)) = (length, duration) {
                                    Meters::from(*length) / Milliseconds::from(*duration)
                                } else {
                                    continue;
                                }
                            } else {
                                KilometersPerHour::from(
                                    *(proto_edge.metric(metric_type.id()).ok_or(format!(
                                        "Metric {} should be provided, but isn't.",
                                        MetricType::Maxspeed { provided }
                                    ))?),
                                )
                            }
                        };

                        let maxspeed = maxspeed.into();
                        proto_edge.add_metric(metric_type.id(), maxspeed);
                        metric_vec.push(maxspeed);
                        is_metric_processed[i] = true;
                    }
                    MetricType::LaneCount => {
                        let lane_count = {
                            *(proto_edge.metric(metric_type.id()).ok_or(format!(
                                "Metric {} should be provided, but isn't.",
                                MetricType::LaneCount
                            ))?)
                        };

                        metric_vec.push(lane_count);
                        is_metric_processed[i] = true;
                    }
                    MetricType::Custom { id: _ } => {
                        let custom_metric = {
                            *(proto_edge.metric(metric_type.id()).ok_or(format!(
                                "Metric {} should be provided, but isn't.",
                                MetricType::Custom {
                                    id: metric_type.id().to_owned()
                                }
                            ))?)
                        };

                        metric_vec.push(custom_metric);
                        is_metric_processed[i] = true;
                    }
                    MetricType::Id { id: _ } | MetricType::Ignore { id: _ } => (),
                }
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

/// public stuff for accessing the (static) graph
impl Graph {
    pub fn nodes<'a>(&'a self) -> NodeContainer<'a> {
        NodeContainer {
            node_ids: &self.node_ids,
            node_coords: &self.node_coords,
        }
    }

    pub fn fwd_edges<'a>(&'a self) -> EdgeContainer<'a> {
        EdgeContainer {
            edge_dsts: &(self.fwd_dsts),
            offsets: &(self.fwd_offsets),
            xwd_to_fwd_map: &(self.fwd_to_fwd_map),
            metrics: self.metrics(),
        }
    }

    pub fn bwd_edges<'a>(&'a self) -> EdgeContainer<'a> {
        EdgeContainer {
            edge_dsts: &(self.bwd_dsts),
            offsets: &(self.bwd_offsets),
            xwd_to_fwd_map: &(self.bwd_to_fwd_map),
            metrics: self.metrics(),
        }
    }

    pub fn metrics<'a>(&'a self) -> MetricContainer<'a> {
        MetricContainer {
            metric_ids: &(self.metric_ids),
            metrics: &(self.metrics),
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph: {{ number of nodes: {}, number of fwd_edges: {} }}",
            self.nodes().count(),
            self.fwd_edges().count()
        )?;

        writeln!(f, "")?;

        let n = 20;
        let m = 20;

        // print nodes
        for mut i in 0..n {
            // if enough nodes are in the graph
            if i < self.nodes().count() {
                if i == n - 1 {
                    // if at least 2 nodes are missing -> print `...`
                    if i + 1 < self.nodes().count() {
                        writeln!(f, "...")?;
                    }
                    // print last node
                    i = self.nodes().count() - 1;
                }
                let node = &self.nodes().create(NodeIdx::new(i));
                writeln!(
                    f,
                    "Node: {{ idx: {}, id: {}, {} }}",
                    i,
                    node.id(),
                    node.coord(),
                )?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        let graph_stuff = vec![
            (
                self.fwd_edges(),
                self.bwd_edges(),
                &(self.fwd_offsets),
                "fwd-",
            ),
            (
                self.bwd_edges(),
                self.fwd_edges(),
                &(self.bwd_offsets),
                "bwd-",
            ),
        ];
        for stuff_idx in 0..graph_stuff.len() {
            let (fwd_dsts, bwd_dsts, xwd_offsets, xwd_prefix) = &graph_stuff[stuff_idx];

            // print up to m xwd-edges
            for mut j in 0..m {
                // if enough edges are in the graph
                if j < fwd_dsts.count() {
                    // if last edge that gets printed
                    if j == m - 1 {
                        // if at least 2 edges are missing -> print `...`
                        if j + 1 < fwd_dsts.count() {
                            writeln!(f, "...")?;
                        }
                        // print last edge
                        j = fwd_dsts.count() - 1;
                    }
                    let edge_idx = EdgeIdx::new(j);
                    let half_edge = fwd_dsts.half_edge(edge_idx).unwrap();
                    let src_idx = bwd_dsts.dst_idx(edge_idx).unwrap();
                    writeln!(
                        f,
                        "{}edge: {{ idx: {}, ({})-{}->({}) }}",
                        xwd_prefix,
                        j,
                        self.node_ids[src_idx.to_usize()],
                        half_edge.length().unwrap(),
                        self.node_ids[half_edge.dst_idx().to_usize()],
                    )?;
                } else {
                    break;
                }
            }

            writeln!(f, "")?;

            // print up to n xwd-offsets
            for mut i in 0..n {
                // if enough offsets are in the graph
                if i < self.nodes().count() {
                    if i == n - 1 {
                        // if at least 2 offsets are missing -> print `...`
                        if i + 1 < self.nodes().count() {
                            writeln!(f, "...")?;
                        }
                        // print last offset
                        i = self.nodes().count() - 1;
                    }
                    writeln!(
                        f,
                        "{{ id: {}, {}offset: {} }}",
                        i, xwd_prefix, xwd_offsets[i]
                    )?;
                } else {
                    break;
                }
            }
            // offset has n+1 entries due to `leaving_edges(...)`
            let i = xwd_offsets.len() - 1;
            writeln!(
                f,
                "{{ __: {}, {}offset: {} }}",
                i, xwd_prefix, xwd_offsets[i]
            )?;

            // if not graph-stuff -> print empty line
            if stuff_idx < graph_stuff.len() - 1 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Node {
    idx: NodeIdx,
    id: i64,
    coord: Coordinate,
}

impl Node {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn idx(&self) -> NodeIdx {
        self.idx
    }

    pub fn coord(&self) -> Coordinate {
        self.coord
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.idx == other.idx
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node: {{ id: {}, idx: {}, coord: {} }}",
            self.id(),
            self.idx(),
            self.coord(),
        )
    }
}

#[derive(Debug)]
pub struct HalfEdge<'a> {
    idx: EdgeIdx,
    edge_dsts: &'a Vec<NodeIdx>,
    metrics: &'a MetricContainer<'a>,
}

impl<'a> HalfEdge<'a> {
    pub fn dst_idx(&self) -> NodeIdx {
        self.edge_dsts[self.idx.to_usize()]
    }

    pub fn length(&self) -> Option<Meters> {
        self.metrics.length(self.idx)
    }

    pub fn maxspeed(&self) -> Option<KilometersPerHour> {
        self.metrics.maxspeed(self.idx)
    }

    pub fn duration(&self) -> Option<Milliseconds> {
        self.metrics.duration(self.idx)
    }

    pub fn lane_count(&self) -> Option<MetricU32> {
        self.metrics.lane_count(self.idx)
    }

    pub fn metric<S>(&self, metric_id: S) -> Option<MetricU32>
    where
        S: Into<String>,
    {
        self.metrics.get(metric_id, self.idx)
    }
}

impl<'a> Eq for HalfEdge<'a> {}

impl<'a> PartialEq for HalfEdge<'a> {
    fn eq(&self, other: &HalfEdge) -> bool {
        self.idx == other.idx
    }
}

impl<'a> Display for HalfEdge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ (src)-{}->({}) }}",
            match self.length() {
                Some(meters) => format!("{}", meters),
                None => format!(""),
            },
            self.dst_idx(),
        )
    }
}

/// A shallow container for accessing nodes.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct NodeContainer<'a> {
    node_ids: &'a Vec<i64>,
    node_coords: &'a Vec<Coordinate>,
}

impl<'a> NodeContainer<'a> {
    pub fn count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn id(&self, idx: NodeIdx) -> i64 {
        self.node_ids[idx.to_usize()]
    }

    pub fn coord(&self, idx: NodeIdx) -> Coordinate {
        self.node_coords[idx.to_usize()]
    }

    pub fn idx_from(&self, id: i64) -> Result<NodeIdx, NodeIdx> {
        match self.node_ids.binary_search(&id) {
            Ok(idx) => Ok(NodeIdx::new(idx)),
            Err(idx) => Err(NodeIdx::new(idx)),
        }
    }

    fn src_idx_from(&self, src_id: i64) -> Result<NodeIdx, String> {
        match self.idx_from(src_id) {
            Ok(idx) => Ok(idx),
            Err(_) => {
                return Err(format!(
                    "The given src-id `{:?}` doesn't exist as node",
                    src_id
                ))
            }
        }
    }

    fn dst_idx_from(&self, dst_id: i64) -> Result<NodeIdx, String> {
        match self.idx_from(dst_id) {
            Ok(idx) => Ok(idx),
            Err(_) => {
                return Err(format!(
                    "The given dst-id `{:?}` doesn't exist as node",
                    dst_id
                ))
            }
        }
    }

    pub fn create_from(&self, id: i64) -> Option<Node> {
        let idx = match self.idx_from(id) {
            Ok(idx) => idx,
            Err(_) => return None,
        };
        Some(self.create(idx))
    }

    pub fn create(&self, idx: NodeIdx) -> Node {
        let id = self.id(idx);
        let coord = self.coord(idx);
        Node { id, idx, coord }
    }
}

/// A shallow container for accessing edges.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct EdgeContainer<'a> {
    edge_dsts: &'a Vec<NodeIdx>,
    offsets: &'a Vec<usize>,
    // indirect mapping to save memory
    xwd_to_fwd_map: &'a Vec<EdgeIdx>,
    metrics: MetricContainer<'a>,
}

impl<'a> EdgeContainer<'a> {
    pub fn count(&self) -> usize {
        self.edge_dsts.len()
    }

    pub fn half_edge(&'a self, idx: EdgeIdx) -> Option<HalfEdge> {
        Some(HalfEdge {
            idx,
            edge_dsts: &self.edge_dsts,
            metrics: &self.metrics,
        })
    }

    pub fn dst_idx(&self, idx: EdgeIdx) -> Option<NodeIdx> {
        Some(*(self.edge_dsts.get(idx.to_usize())?))
    }

    /// Creates `HalfEdge`s containing all metric-data.
    /// For only indices, see `dsts_starting_from(...)`
    pub fn starting_from(&self, idx: NodeIdx) -> Option<Vec<HalfEdge>> {
        // get indices by reading offset-array
        let leaving_indices = self.offset_indices(idx)?;

        // create array of leaving edges
        let mut leaving_edges = vec![];
        for edge_idx in leaving_indices {
            let edge = self.half_edge(edge_idx)?;
            leaving_edges.push(edge);
        }
        Some(leaving_edges)
    }

    /// uses linear-search, but only on src's leaving edges (±3), so more or less in O(1)
    ///
    /// Returns the index of the edge, which can be used in the function `half_edge(...)`
    pub fn between(&self, src_idx: NodeIdx, dst_idx: NodeIdx) -> Option<(HalfEdge, EdgeIdx)> {
        // get indices by reading offset-array if src-node has leaving edges
        let leaving_indices = self.offset_indices(src_idx)?;

        // find edge of same dst-idx and create edge
        for edge_idx in leaving_indices {
            if self.dst_idx(edge_idx)? == dst_idx {
                return Some((self.half_edge(edge_idx)?, edge_idx));
            }
        }

        None
    }

    /// Returns None if
    ///
    /// - no node with given idx is in the graph
    /// - if this node has no leaving edges
    fn offset_indices(&self, idx: NodeIdx) -> Option<Vec<EdgeIdx>> {
        // Use offset-array to get indices for the graph's edges belonging to the given node
        let i0 = *(self.offsets.get(idx.to_usize())?);
        // (idx + 1) guaranteed by offset-array-length
        let i1 = *(self.offsets.get(idx.to_usize() + 1)?);

        // i0 < i1 <-> node has leaving edges
        if i0 < i1 {
            // map usizes to respective EdgeIdx
            let mut edge_indices = vec![];
            for i in i0..i1 {
                edge_indices.push(self.xwd_to_fwd_map[i])
            }
            Some(edge_indices)
        } else {
            None
        }
    }
}

/// A shallow container for accessing metrics.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct MetricContainer<'a> {
    metric_ids: &'a Vec<String>,
    metrics: &'a Vec<Vec<MetricU32>>,
}

impl<'a> MetricContainer<'a> {
    fn metric_vec_idx<S>(&self, metric_id: S) -> Result<usize, String>
    where
        S: Into<String>,
    {
        let metric_id = metric_id.into();

        if let Some(metric_idx) = self.metric_ids.iter().position(|id| id == &metric_id) {
            Ok(metric_idx)
        } else {
            Err(format!(
                "Graph is expected to have a metric of given id {} but doesn't.",
                metric_id
            ))
        }
    }

    fn metric_vec<S>(&self, metric_id: S) -> Result<&Vec<MetricU32>, String>
    where
        S: Into<String>,
    {
        let metric_idx = self.metric_vec_idx(metric_id)?;
        Ok(&self.metrics[metric_idx])
    }

    pub fn get<S>(&self, metric_id: S, edge_idx: EdgeIdx) -> Option<MetricU32>
    where
        S: Into<String>,
    {
        let metric = self.metric_vec(metric_id).ok()?;
        let value = *(metric.get(edge_idx.to_usize())?);
        Some(value)
    }

    pub fn length(&self, edge_idx: EdgeIdx) -> Option<Meters> {
        let length = self
            .get(crate::configs::constants::ids::LENGTH, edge_idx)?
            .into();
        debug_assert!(length > Meters::zero(), "Edge-length should be > 0");
        Some(length)
    }

    pub fn maxspeed(&self, edge_idx: EdgeIdx) -> Option<KilometersPerHour> {
        let maxspeed = self
            .get(crate::configs::constants::ids::MAXSPEED, edge_idx)?
            .into();
        debug_assert!(
            maxspeed > KilometersPerHour::zero(),
            "Edge-maxspeed should be > 0"
        );
        Some(maxspeed)
    }

    pub fn duration(&self, edge_idx: EdgeIdx) -> Option<Milliseconds> {
        let duration = self
            .get(crate::configs::constants::ids::DURATION, edge_idx)?
            .into();
        debug_assert!(
            duration > Milliseconds::zero(),
            "Edge-duration should be > 0"
        );
        Some(duration)
    }

    pub fn lane_count(&self, edge_idx: EdgeIdx) -> Option<MetricU32> {
        let lane_count = self
            .get(crate::configs::constants::ids::LANE_COUNT, edge_idx)?
            .into();
        debug_assert!(
            lane_count > MetricU32::zero(),
            "Edge-lane-count should be > 0"
        );
        Some(lane_count)
    }
}
