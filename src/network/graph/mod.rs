//------------------------------------------------------------------------------------------------//
// other modules

use crate::units::geo::Coordinate;
use crate::units::length::Meters;
use crate::units::speed::KilometersPerHour;
use std::fmt;
use std::fmt::Display;

//------------------------------------------------------------------------------------------------//
// own modules

pub mod building;
mod indexing;
pub use indexing::EdgeIdx;
pub use indexing::NodeIdx;
mod container;
use container::EdgeContainer;
pub use container::HalfEdge;
pub use container::Node;
use container::NodeContainer;

//------------------------------------------------------------------------------------------------//

/// Stores nodes and edges and provides methods for accessing them.
///
/// Real edges, not their indices, are stored
///
/// - `(src-id, dst-id)` with `src-id` having precedence over `dst-id`
/// - in ascending order
#[derive(Debug)]
pub struct Graph {
    // nodes
    node_ids: Vec<i64>,
    // node-metrics
    node_coords: Vec<Coordinate>,
    // edges: offset-graph and mappings, e.g. for metrics
    fwd_edges: Vec<NodeIdx>,
    fwd_offsets: Vec<EdgeIdx>,
    fwd_mapping: Vec<EdgeIdx>,
    bwd_edges: Vec<NodeIdx>,
    bwd_offsets: Vec<EdgeIdx>,
    bwd_mapping: Vec<EdgeIdx>,
    // edge-metrics (sorted according to fwd_edges)
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
            fwd_edges: Vec::new(),
            fwd_offsets: Vec::new(),
            fwd_mapping: Vec::new(),
            bwd_edges: Vec::new(),
            bwd_offsets: Vec::new(),
            bwd_mapping: Vec::new(),
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
            edge_dsts: &(self.fwd_edges),
            offsets: &(self.fwd_offsets),
            idx_map: &(self.fwd_mapping),
            meters: &self.meters,
            maxspeed: &self.maxspeed,
            lane_count: &self.lane_count,
        }
    }

    pub fn bwd_edges<'a>(&'a self) -> EdgeContainer<'a> {
        EdgeContainer {
            edge_dsts: &(self.bwd_edges),
            offsets: &(self.bwd_offsets),
            idx_map: &(self.bwd_mapping),
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
                let node = &self.nodes().create(i.into());
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

        for (fwd_edges, bwd_edges, xwd_offsets, xwd_prefix) in vec![
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
            // print xwd-edges
            for mut j in 0..m {
                // if enough edges are in the graph
                if j < fwd_edges.count() {
                    if j == m - 1 {
                        // if at least 2 edges are missing -> print `...`
                        if j + 1 < fwd_edges.count() {
                            writeln!(f, "...")?;
                        }
                        // print last edge
                        j = fwd_edges.count() - 1;
                    }
                    let halfedge = &fwd_edges.create(j.into());
                    let src_idx = bwd_edges.dst_idx(j.into());
                    writeln!(
                        f,
                        "{}edge: {{ idx: {}, ({})-{}->({}) }}",
                        xwd_prefix,
                        j,
                        self.node_ids[src_idx.usize()],
                        halfedge.meters(),
                        self.node_ids[halfedge.dst_idx().usize()],
                    )?;
                } else {
                    break;
                }
            }

            writeln!(f, "")?;

            // print xwd-offsets
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
