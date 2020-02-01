//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::EdgeContainer;
use crate::network::{EdgeIdx, NodeIdx};
use crate::units::{length::Meters, speed::KilometersPerHour, time::Milliseconds, Metric};
use std::{fmt, fmt::Display};

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct HalfEdge {
    dst_idx: NodeIdx,
    lane_count: u8,
    meters: Meters,
    maxspeed: KilometersPerHour,
}

impl HalfEdge {
    pub fn dst_idx(&self) -> NodeIdx {
        self.dst_idx
    }

    pub fn lane_count(&self) -> u8 {
        debug_assert!(self.lane_count > 0, "Edge-lane-count should be > 0");
        self.lane_count
    }

    pub fn meters(&self) -> Meters {
        debug_assert!(self.meters > Meters::zero(), "Edge-length should be > 0");
        self.meters
    }

    pub fn maxspeed(&self) -> KilometersPerHour {
        debug_assert!(
            self.maxspeed > KilometersPerHour::zero(),
            "Edge-maxspeed should be > 0"
        );
        self.maxspeed
    }

    pub fn milliseconds(&self) -> Milliseconds {
        self.meters() / self.maxspeed()
    }
}

impl Eq for HalfEdge {}

impl PartialEq for HalfEdge {
    fn eq(&self, other: &HalfEdge) -> bool {
        self.dst_idx == other.dst_idx
            && self.meters == other.meters
            && self.maxspeed == other.maxspeed
    }
}

impl Display for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ (src)-{}->({}) }}", self.meters, self.dst_idx,)
    }
}

//------------------------------------------------------------------------------------------------//

impl<'a> EdgeContainer<'a> {
    pub fn count(&self) -> usize {
        self.edge_dsts.len()
    }

    pub fn half_edge(&self, idx: EdgeIdx) -> Option<HalfEdge> {
        let idx = idx.to_usize();

        Some(HalfEdge {
            dst_idx: *(self.edge_dsts.get(idx)?),
            lane_count: *(self.lane_count.get(idx)?),
            meters: *(self.meters.get(idx)?),
            maxspeed: *(self.maxspeed.get(idx)?),
        })
    }

    pub fn dst_idx(&self, idx: EdgeIdx) -> Option<NodeIdx> {
        Some(*(self.edge_dsts.get(idx.to_usize())?))
    }

    pub fn meters(&self, idx: EdgeIdx) -> Option<Meters> {
        Some(*(self.meters.get(idx.to_usize())?))
    }

    pub fn maxspeed(&self, idx: EdgeIdx) -> Option<KilometersPerHour> {
        Some(*(self.maxspeed.get(idx.to_usize())?))
    }

    pub fn lane_count(&self, idx: EdgeIdx) -> Option<u8> {
        Some(*(self.lane_count.get(idx.to_usize())?))
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
