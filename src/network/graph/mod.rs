pub mod building;
mod indexing;
pub use indexing::{EdgeIdx, EdgeIdxIterator, MetricIdx, NodeIdx, NodeIdxIterator};

use crate::{configs::parsing::Config, defaults::capacity::DimVec, helpers::err};
use kissunits::geo::Coordinate;
use std::{
    fmt,
    fmt::Display,
    iter::Iterator,
    ops::{Index, IndexMut},
};

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
    // nodes, ids sorted
    node_ids: Vec<i64>,
    // node-metrics
    node_coords: Vec<Coordinate>,
    node_ch_levels: Vec<usize>,
    // node_heights: Vec<f64>,
    // edges: offset-graph and mappings, e.g. for metrics
    fwd_dsts: Vec<NodeIdx>,
    fwd_offsets: Vec<usize>,
    fwd_to_fwd_map: Vec<EdgeIdx>,
    bwd_dsts: Vec<NodeIdx>,
    bwd_offsets: Vec<usize>,
    bwd_to_fwd_map: Vec<EdgeIdx>,
    // edge-metrics (sorted according to fwd_dsts)
    metrics: Vec<DimVec<f64>>,
    // mapping from id to EdgeIdx, sorted by id
    edge_ids: Vec<Option<usize>>,
    edge_ids_to_idx_map: Vec<(usize, EdgeIdx)>,
    // shortcuts (contraction-hierarchies)
    sc_offsets: Vec<usize>,
    sc_edges: Vec<[EdgeIdx; 2]>,
}

/// public stuff for accessing the (static) graph
impl Graph {
    pub fn cfg(&self) -> &Config {
        &self.cfg
    }

    pub fn nodes<'a>(&'a self) -> NodeAccessor<'a> {
        NodeAccessor {
            node_ids: &self.node_ids,
            node_coords: &self.node_coords,
            node_ch_levels: &self.node_ch_levels,
        }
    }

    pub fn fwd_edges<'a>(&'a self) -> EdgeAccessor<'a> {
        EdgeAccessor {
            edge_ids: &self.edge_ids,
            edge_ids_to_idx_map: &self.edge_ids_to_idx_map,
            edge_dsts: &self.fwd_dsts,
            offsets: &self.fwd_offsets,
            xwd_to_fwd_map: &self.fwd_to_fwd_map,
            metrics: self.metrics(),
            sc_offsets: &self.sc_offsets,
            sc_edges: &self.sc_edges,
        }
    }

    pub fn bwd_edges<'a>(&'a self) -> EdgeAccessor<'a> {
        EdgeAccessor {
            edge_ids: &self.edge_ids,
            edge_ids_to_idx_map: &self.edge_ids_to_idx_map,
            edge_dsts: &(self.bwd_dsts),
            offsets: &(self.bwd_offsets),
            xwd_to_fwd_map: &(self.bwd_to_fwd_map),
            metrics: self.metrics(),
            sc_offsets: &self.sc_offsets,
            sc_edges: &self.sc_edges,
        }
    }

    pub fn metrics<'a>(&'a self) -> MetricAccessor<'a> {
        MetricAccessor {
            cfg: &self.cfg,
            metrics: &self.metrics,
        }
    }

    pub fn metrics_mut<'a>(&'a mut self) -> MetricAccessorMut<'a> {
        MetricAccessorMut {
            cfg: &self.cfg,
            metrics: &mut self.metrics,
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

        writeln!(f, "Config: {:?}", self.cfg)?;

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
                        writeln!(f, "Node: ...")?;
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
            let (fwd_edges, bwd_edges, xwd_offsets, xwd_prefix) = &graph_stuff[stuff_idx];

            // print up to m xwd-edges
            for mut j in 0..m {
                // if enough edges are in the graph
                if j < fwd_edges.count() {
                    // if last edge that gets printed
                    if j == m - 1 {
                        // if at least 2 edges are missing -> print `...`
                        if j + 1 < fwd_edges.count() {
                            writeln!(f, "{}edge: ...", xwd_prefix)?;
                        }
                        // print last edge
                        j = fwd_edges.count() - 1;
                    }
                    let edge_idx = EdgeIdx(j);
                    let src_idx = bwd_edges.dst_idx(edge_idx);
                    let half_edge = fwd_edges.half_edge(edge_idx);
                    let metrics = half_edge.metrics();
                    writeln!(
                        f,
                        "{}edge: {{ {}idx: {}, sc-offset: {}, (idx: {})-{:?}->(idx: {}) }}",
                        xwd_prefix,
                        match fwd_edges.try_id(edge_idx) {
                            Some(id) => format!("id: {}, ", id),
                            None => String::from(""),
                        },
                        j,
                        self.sc_offsets[j],
                        *src_idx,
                        metrics,
                        *half_edge.dst_idx(),
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
                            writeln!(f, "{}offset: ...", xwd_prefix)?;
                        }
                        // print last offset
                        i = self.nodes().count() - 1;
                    }
                    writeln!(
                        f,
                        "{}offset: {{ node-id: {}, offset: {} }}",
                        xwd_prefix, i, xwd_offsets[i]
                    )?;
                } else {
                    break;
                }
            }
            // offset has n+1 entries due to `leaving_edges(...)`
            let i = xwd_offsets.len() - 1;
            writeln!(
                f,
                "{}offset: {{ __: {}, offset: {} }}",
                xwd_prefix, i, xwd_offsets[i]
            )?;

            writeln!(f, "")?;
        }

        // print up to m shortcut-edges
        if self.sc_edges.len() == 0 {
            writeln!(f, "No shortcuts in graph.")?;
        } else {
            let mut edge_idx = 0;
            for mut j in 0..m {
                // if enough edges are in the graph
                if j < self.sc_edges.len() {
                    // if last edge that gets printed
                    if j == m - 1 {
                        // if at least 2 edges are missing -> print `...`
                        if j + 1 < self.sc_edges.len() {
                            writeln!(f, "shortcut: ...")?;
                        }
                        // print last edge
                        j = self.sc_edges.len() - 1;
                    }
                    // get edge-idx from sc-edge
                    while self.sc_offsets[edge_idx] <= j {
                        edge_idx += 1;
                    }
                    edge_idx -= 1;
                    writeln!(
                        f,
                        "shortcut: {{ edge-idx: {}, sc-offset: {}, replaced: {:?} }}",
                        edge_idx,
                        self.sc_offsets[edge_idx],
                        self.sc_edges[self.sc_offsets[edge_idx]],
                    )?;
                } else {
                    break;
                }
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
    level: usize,
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

    pub fn ch_level(&self) -> usize {
        self.level
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
    edge_accessor: &'a EdgeAccessor<'a>,
}

impl<'a> HalfEdge<'a> {
    pub fn idx(&self) -> EdgeIdx {
        self.idx
    }

    pub fn dst_idx(&self) -> NodeIdx {
        self.edge_accessor.dst_idx(self.idx)
    }

    pub fn is_shortcut(&self) -> bool {
        self.edge_accessor.is_shortcut(self.idx)
    }

    pub fn sc_edges(&self) -> Option<&[EdgeIdx; 2]> {
        self.edge_accessor.sc_edges(self.idx)
    }

    pub fn metrics(&self) -> &DimVec<f64> {
        &self.edge_accessor.metrics[self.idx]
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
            "{{ (src)-{:?}->(idx: {}) }}",
            self.edge_accessor.metrics[self.idx],
            self.dst_idx(),
        )
    }
}

/// A shallow container for accessing nodes.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct NodeAccessor<'a> {
    node_ids: &'a Vec<i64>,
    node_coords: &'a Vec<Coordinate>,
    node_ch_levels: &'a Vec<usize>,
}

impl IntoIterator for NodeAccessor<'_> {
    type Item = NodeIdx;
    type IntoIter = NodeIdxIterator;

    fn into_iter(self) -> NodeIdxIterator {
        (0..self.count()).into()
    }
}

impl<'a> IntoIterator for &'a NodeAccessor<'_> {
    type Item = NodeIdx;
    type IntoIter = NodeIdxIterator;

    fn into_iter(self) -> NodeIdxIterator {
        (0..self.count()).into()
    }
}

impl<'a> NodeAccessor<'a> {
    pub fn iter(&self) -> NodeIdxIterator {
        self.into_iter()
    }

    pub fn count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn id(&self, idx: NodeIdx) -> i64 {
        self.node_ids[*idx]
    }

    pub fn coord(&self, idx: NodeIdx) -> Coordinate {
        self.node_coords[*idx]
    }

    pub fn level(&self, idx: NodeIdx) -> usize {
        self.node_ch_levels[*idx]
    }

    pub fn idx_from(&self, id: i64) -> Result<NodeIdx, NodeIdx> {
        match self.node_ids.binary_search(&id) {
            Ok(idx) => Ok(NodeIdx(idx)),
            Err(idx) => Err(NodeIdx(idx)),
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
        let level = self.level(idx);
        Node {
            id,
            idx,
            coord,
            level,
        }
    }
}

/// A shallow container for accessing edges.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct EdgeAccessor<'a> {
    edge_ids: &'a Vec<Option<usize>>,
    edge_ids_to_idx_map: &'a Vec<(usize, EdgeIdx)>,
    edge_dsts: &'a Vec<NodeIdx>,
    offsets: &'a Vec<usize>,
    // indirect mapping to save memory
    xwd_to_fwd_map: &'a Vec<EdgeIdx>,
    metrics: MetricAccessor<'a>,
    // shortcuts
    sc_offsets: &'a Vec<usize>,
    sc_edges: &'a Vec<[EdgeIdx; 2]>,
}

impl IntoIterator for EdgeAccessor<'_> {
    type Item = EdgeIdx;
    type IntoIter = EdgeIdxIterator;

    fn into_iter(self) -> EdgeIdxIterator {
        (0..self.count()).into()
    }
}

impl<'a> IntoIterator for &'a EdgeAccessor<'_> {
    type Item = EdgeIdx;
    type IntoIter = EdgeIdxIterator;

    fn into_iter(self) -> EdgeIdxIterator {
        (0..self.count()).into()
    }
}

impl<'a> EdgeAccessor<'a> {
    pub fn iter(&self) -> EdgeIdxIterator {
        self.into_iter()
    }

    pub fn count(&self) -> usize {
        self.edge_dsts.len()
    }

    pub fn half_edge(&'a self, idx: EdgeIdx) -> HalfEdge {
        HalfEdge {
            idx,
            edge_accessor: self,
        }
    }

    pub fn try_id(&self, idx: EdgeIdx) -> Option<usize> {
        self.edge_ids[*idx]
    }

    pub fn id(&self, idx: EdgeIdx) -> usize {
        self.edge_ids[*idx].expect(&format!("Edge-id expected at edge-idx {}.", *idx))
    }

    pub fn try_idx_from(&self, id: usize) -> err::Result<EdgeIdx> {
        // edge-ids are sorted in this "map" (vector)
        // -> mapped from id to edge-idx
        match self
            .edge_ids_to_idx_map
            .binary_search_by_key(&id, |(edge_id, _edge_idx)| *edge_id)
        {
            Ok(idx) => {
                let (_edge_id, edge_idx) = self.edge_ids_to_idx_map[idx];
                Ok(edge_idx)
            }
            Err(_idx) => Err(err::Msg::from(format!(
                "The provided edge-id {} is expected to be in the graph, but is not.",
                id
            ))),
        }
    }

    pub fn idx_from(&self, id: usize) -> EdgeIdx {
        match self.try_idx_from(id) {
            Ok(edge_idx) => edge_idx,
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn dst_idx(&self, idx: EdgeIdx) -> NodeIdx {
        self.edge_dsts[*idx]
    }

    pub fn metrics(&self) -> &MetricAccessor<'a> {
        &self.metrics
    }

    pub fn is_shortcut(&self, idx: EdgeIdx) -> bool {
        // no overflow due to (len + 1)
        self.sc_offsets[(*idx) + 1] - self.sc_offsets[*idx] != 0
    }

    pub fn sc_edges(&self, idx: EdgeIdx) -> Option<&[EdgeIdx; 2]> {
        if self.is_shortcut(idx) {
            Some(&self.sc_edges[self.sc_offsets[*idx]])
        } else {
            None
        }
    }

    pub fn starting_from(&'a self, idx: NodeIdx) -> impl Iterator<Item = HalfEdge<'a>> {
        self.offset_indices(idx)
            .map(move |edge_idx| self.half_edge(edge_idx))
    }

    /// uses linear-search, but only on src's leaving edges (±3), so more or less in O(1)
    ///
    /// Returns the index of the edge, which can be used in the function `half_edge(...)`
    pub fn between(&self, src_idx: NodeIdx, dst_idx: NodeIdx) -> Option<HalfEdge> {
        // find edge of same dst-idx and create edge
        for edge_idx in self.offset_indices(src_idx) {
            if self.dst_idx(edge_idx) == dst_idx {
                return Some(self.half_edge(edge_idx));
            }
        }

        None
    }

    fn offset_indices(&'a self, idx: NodeIdx) -> impl Iterator<Item = EdgeIdx> + 'a {
        // Use offset-array to get indices for the graph's edges belonging to the given node
        // (idx + 1) guaranteed by offset-array-length
        // i0 <= i1 <-> node has 0 or more leaving edges
        (self.offsets[*idx]..self.offsets[*idx + 1])
            .into_iter()
            .map(move |i| self.xwd_to_fwd_map[i])
    }
}

/// A shallow container for accessing metrics.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct MetricAccessor<'a> {
    cfg: &'a Config,
    metrics: &'a Vec<DimVec<f64>>,
}

impl<'a> MetricAccessor<'a> {
    pub fn dim(&self) -> usize {
        self.cfg.edges.metrics.units.len()
    }
}

impl<'a> Index<EdgeIdx> for MetricAccessor<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: EdgeIdx) -> &DimVec<f64> {
        &self.metrics[*edge_idx]
    }
}

impl<'a> Index<EdgeIdx> for &MetricAccessor<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: EdgeIdx) -> &DimVec<f64> {
        &self.metrics[*edge_idx]
    }
}

impl<'a> Index<&EdgeIdx> for MetricAccessor<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: &EdgeIdx) -> &DimVec<f64> {
        &self.metrics[**edge_idx]
    }
}

impl<'a> Index<&EdgeIdx> for &MetricAccessor<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: &EdgeIdx) -> &DimVec<f64> {
        &self.metrics[**edge_idx]
    }
}

/// A shallow container for accessing metrics.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct MetricAccessorMut<'a> {
    cfg: &'a Config,
    metrics: &'a mut Vec<DimVec<f64>>,
}

impl<'a> MetricAccessorMut<'a> {
    pub fn dim(&self) -> usize {
        self.cfg.edges.metrics.units.len()
    }
}

impl<'a> Index<EdgeIdx> for MetricAccessorMut<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: EdgeIdx) -> &DimVec<f64> {
        &self.metrics[*edge_idx]
    }
}

impl<'a> IndexMut<EdgeIdx> for MetricAccessorMut<'a> {
    fn index_mut(&mut self, edge_idx: EdgeIdx) -> &mut DimVec<f64> {
        &mut self.metrics[*edge_idx]
    }
}

impl<'a> Index<EdgeIdx> for &MetricAccessorMut<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: EdgeIdx) -> &DimVec<f64> {
        &self.metrics[*edge_idx]
    }
}

impl<'a> Index<EdgeIdx> for &mut MetricAccessorMut<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: EdgeIdx) -> &DimVec<f64> {
        &self.metrics[*edge_idx]
    }
}

impl<'a> IndexMut<EdgeIdx> for &mut MetricAccessorMut<'a> {
    fn index_mut(&mut self, edge_idx: EdgeIdx) -> &mut DimVec<f64> {
        &mut self.metrics[*edge_idx]
    }
}

impl<'a> Index<&EdgeIdx> for MetricAccessorMut<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: &EdgeIdx) -> &DimVec<f64> {
        &self.metrics[**edge_idx]
    }
}

impl<'a> IndexMut<&EdgeIdx> for MetricAccessorMut<'a> {
    fn index_mut(&mut self, edge_idx: &EdgeIdx) -> &mut DimVec<f64> {
        &mut self.metrics[**edge_idx]
    }
}

impl<'a> Index<&EdgeIdx> for &MetricAccessorMut<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: &EdgeIdx) -> &DimVec<f64> {
        &self.metrics[**edge_idx]
    }
}

impl<'a> Index<&EdgeIdx> for &mut MetricAccessorMut<'a> {
    type Output = DimVec<f64>;

    fn index(&self, edge_idx: &EdgeIdx) -> &DimVec<f64> {
        &self.metrics[**edge_idx]
    }
}

impl<'a> IndexMut<&EdgeIdx> for &mut MetricAccessorMut<'a> {
    fn index_mut(&mut self, edge_idx: &EdgeIdx) -> &mut DimVec<f64> {
        &mut self.metrics[**edge_idx]
    }
}
