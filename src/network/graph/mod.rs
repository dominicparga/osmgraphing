pub mod building;
mod indexing;
pub use indexing::{EdgeIdx, MetricIdx, NodeIdx};

use crate::{
    configs::graph::Config,
    units::{
        geo::Coordinate, length::Meters, speed::KilometersPerHour, time::Milliseconds, Metric,
        MetricU32,
    },
};
use std::{fmt, fmt::Display};

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
    cfg: Config,
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
    metrics: Vec<Vec<MetricU32>>,
}

/// public stuff for accessing the (static) graph
impl Graph {
    pub fn cfg(&self) -> &Config {
        &self.cfg
    }

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
            cfg: &(self.cfg),
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
                let node = &self.nodes().create(NodeIdx(i));
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
                    let edge_idx = EdgeIdx(j);
                    let src_idx = bwd_dsts.dst_idx(edge_idx).unwrap();
                    let half_edge = fwd_dsts.half_edge(edge_idx).unwrap();
                    let metrics: Vec<u32> = (0..self.cfg.edges.metrics.count())
                        .map(|i| *self.metrics[i][*edge_idx])
                        .collect();
                    writeln!(
                        f,
                        "{}edge: {{ idx: {}, ({})-{:?}->({}) }}",
                        xwd_prefix,
                        j,
                        self.node_ids[*src_idx],
                        metrics,
                        self.node_ids[*half_edge.dst_idx()],
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
        self.edge_dsts[*self.idx]
    }

    pub fn length(&self, metric_idx: MetricIdx) -> Option<Meters> {
        self.metrics.length(metric_idx, self.idx)
    }

    pub fn maxspeed(&self, metric_idx: MetricIdx) -> Option<KilometersPerHour> {
        self.metrics.maxspeed(metric_idx, self.idx)
    }

    pub fn duration(&self, metric_idx: MetricIdx) -> Option<Milliseconds> {
        self.metrics.duration(metric_idx, self.idx)
    }

    pub fn lane_count(&self, metric_idx: MetricIdx) -> Option<MetricU32> {
        self.metrics.lane_count(metric_idx, self.idx)
    }

    pub fn metric(&self, metric_idx: MetricIdx) -> Option<MetricU32> {
        self.metrics.get(metric_idx, self.idx)
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
        write!(f, "{{ (src)-{}->({}) }}", self.metrics, self.dst_idx(),)
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
        self.node_ids[*idx]
    }

    pub fn coord(&self, idx: NodeIdx) -> Coordinate {
        self.node_coords[*idx]
    }

    pub fn idx_from(&self, id: i64) -> Result<NodeIdx, NodeIdx> {
        match self.node_ids.binary_search(&id) {
            Ok(idx) => Ok(NodeIdx(idx)),
            Err(idx) => Err(NodeIdx(idx)),
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
        Some(*(self.edge_dsts.get(*idx)?))
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
        let i0 = *(self.offsets.get(*idx)?);
        // (idx + 1) guaranteed by offset-array-length
        let i1 = *(self.offsets.get(*idx + 1)?);

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
    cfg: &'a Config,
    metrics: &'a Vec<Vec<MetricU32>>,
}

impl<'a> Display for MetricContainer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.metrics)
    }
}

impl<'a> MetricContainer<'a> {
    pub fn get(&self, metric_idx: MetricIdx, edge_idx: EdgeIdx) -> Option<MetricU32> {
        let metric_vec = &self.metrics[*metric_idx];
        Some(metric_vec[*edge_idx])
    }

    pub fn length(&self, metric_idx: MetricIdx, edge_idx: EdgeIdx) -> Option<Meters> {
        let length = Meters::from(self.get(metric_idx, edge_idx)?);
        debug_assert!(length > Meters::zero(), "Edge-length should be > 0");
        Some(length)
    }

    pub fn maxspeed(&self, metric_idx: MetricIdx, edge_idx: EdgeIdx) -> Option<KilometersPerHour> {
        let maxspeed = KilometersPerHour::from(self.get(metric_idx, edge_idx)?);
        debug_assert!(
            maxspeed > KilometersPerHour::zero(),
            "Edge-maxspeed should be > 0"
        );
        Some(maxspeed)
    }

    pub fn duration(&self, metric_idx: MetricIdx, edge_idx: EdgeIdx) -> Option<Milliseconds> {
        let duration = Milliseconds::from(self.get(metric_idx, edge_idx)?);
        debug_assert!(
            duration > Milliseconds::zero(),
            "Edge-duration should be > 0"
        );
        Some(duration)
    }

    pub fn lane_count(&self, metric_idx: MetricIdx, edge_idx: EdgeIdx) -> Option<MetricU32> {
        let lane_count = self.get(metric_idx, edge_idx)?;
        debug_assert!(
            lane_count > MetricU32::zero(),
            "Edge-lane-count should be > 0"
        );
        Some(lane_count)
    }
}
