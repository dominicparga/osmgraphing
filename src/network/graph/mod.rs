//------------------------------------------------------------------------------------------------//
// own modules

pub mod building;
mod indexing;
pub use indexing::{EdgeIdx, NodeIdx};
mod container;
pub use container::{HalfEdge, Node};

//------------------------------------------------------------------------------------------------//
// other modules

use crate::units::{geo::Coordinate, length::Meters, speed::KilometersPerHour};
use std::{fmt, fmt::Display};

//------------------------------------------------------------------------------------------------//

/// A shallow container for accessing nodes.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct NodeContainer<'a> {
    node_ids: &'a Vec<i64>,
    node_coords: &'a Vec<Coordinate>,
}

/// A shallow container for accessing edges.
/// Shallow means that it does only contain references to the graph's data-arrays.
#[derive(Debug)]
pub struct EdgeContainer<'a> {
    edge_dsts: &'a Vec<NodeIdx>,
    offsets: &'a Vec<usize>,
    // indirect mapping to save memory
    xwd_to_fwd_map: &'a Vec<EdgeIdx>,
    // metrics
    meters: &'a Vec<Meters>,
    maxspeed: &'a Vec<KilometersPerHour>,
    lane_count: &'a Vec<u8>,
}

//------------------------------------------------------------------------------------------------//

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
    meters: Vec<Meters>,
    maxspeed: Vec<KilometersPerHour>,
    lane_count: Vec<u8>,
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
            meters: Vec::new(),
            maxspeed: Vec::new(),
            lane_count: Vec::new(),
        }
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            ..Default::default()
        }
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
            meters: &self.meters,
            maxspeed: &self.maxspeed,
            lane_count: &self.lane_count,
        }
    }

    pub fn bwd_edges<'a>(&'a self) -> EdgeContainer<'a> {
        EdgeContainer {
            edge_dsts: &(self.bwd_dsts),
            offsets: &(self.bwd_offsets),
            xwd_to_fwd_map: &(self.bwd_to_fwd_map),
            meters: &self.meters,
            maxspeed: &self.maxspeed,
            lane_count: &self.lane_count,
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

        for (fwd_dsts, bwd_dsts, xwd_offsets, xwd_prefix) in vec![
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
        ] {
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
                        half_edge.meters(),
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
        }

        Ok(())
    }
}
