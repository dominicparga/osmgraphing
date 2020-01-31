//------------------------------------------------------------------------------------------------//
// other modules

use crate::network::EdgeIdx;
use crate::network::NodeIdx;
use crate::units::length::Meters;
use crate::units::speed::KilometersPerHour;
use crate::units::time::Milliseconds;
use crate::units::Metric;
use std::fmt;
use std::fmt::Display;
use std::ops::Range;

//------------------------------------------------------------------------------------------------//
// own modules

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

#[derive(Debug)]
pub struct EdgeContainer<'a> {
    edge_dsts: &'a Vec<NodeIdx>,
    offsets: &'a Vec<EdgeIdx>,
    // indirect mapping to save memory
    idx_map: &'a Vec<EdgeIdx>,
    // metrics
    meters: &'a Vec<Meters>,
    maxspeed: &'a Vec<KilometersPerHour>,
    lane_count: &'a Vec<u8>,
}

impl<'a> EdgeContainer<'a> {
    pub fn count(&self) -> usize {
        self.edge_dsts.len()
    }

    pub fn create(&self, idx: EdgeIdx) -> HalfEdge {
        let dst_idx = self.edge_dsts[idx];
        let mapped_idx = self.idx_map[idx].usize();
        let meters = self.meters[mapped_idx];
        let maxspeed = self.maxspeed[mapped_idx];
        let lane_count = self.lane_count[mapped_idx];

        HalfEdge {
            dst_idx,
            lane_count,
            meters,
            maxspeed,
        }
    }

    /// indirect access
    pub fn dst_idx(&self, idx: EdgeIdx) -> NodeIdx {
        self.edge_dsts[idx]
    }

    pub fn meters(&self, idx: EdgeIdx) -> Meters {
        let mapped_idx = self.idx_map[idx].usize();
        self.meters[mapped_idx]
    }

    pub fn maxspeed(&self, idx: EdgeIdx) -> KilometersPerHour {
        let mapped_idx = self.idx_map[idx].usize();
        self.maxspeed[mapped_idx]
    }

    pub fn lane_count(&self, idx: EdgeIdx) -> u8 {
        let mapped_idx = self.idx_map[idx].usize();
        self.lane_count[mapped_idx]
    }

    /// Creates `HalfEdge`s containing all metric-data.
    /// For only indices, see `dsts_starting_from(...)`
    pub fn starting_from(&self, idx: NodeIdx) -> Option<Vec<HalfEdge>> {
        // get indices by reading offset-array
        let range = self.offset_indices(idx)?;
        let leaving_indices = range.start.usize()..range.end.usize();

        // create array of leaving edges
        let mut leaving_edges = vec![];
        for idx in leaving_indices {
            let edge_idx = EdgeIdx::from(idx);
            let edge = self.create(edge_idx);
            leaving_edges.push(edge);
        }
        Some(leaving_edges)
    }

    /// uses linear-search, but only on src's leaving edges (±3), so more or less in O(1)
    ///
    /// Returns the index of the edge, which can be used in the function `edge(...)`
    pub fn between(&self, src_idx: NodeIdx, dst_idx: NodeIdx) -> Option<(HalfEdge, EdgeIdx)> {
        // get indices by reading offset-array
        let range = self.offset_indices(src_idx)?;
        let leaving_indices = range.start.usize()..range.end.usize();

        // find edge of same dst-idx and create edge
        for idx in leaving_indices {
            let edge_idx = idx.into();
            if self.dst_idx(edge_idx) == dst_idx {
                let edge = self.create(edge_idx);
                return Some((edge, edge_idx));
            }
        }

        // no edge of given src-dst-pair
        return None;
    }

    /// Returns a "real" range, where `start_bound < end_bound`
    fn offset_indices(&self, idx: NodeIdx) -> Option<Range<EdgeIdx>> {
        // Use offset-array to get indices for the graph's edges belonging to the given node
        let i0 = self.offsets[idx];
        // (idx + 1) guaranteed by offset-array-length
        let i1 = self.offsets[idx + 1];

        // i0 < i1 <-> node has leaving edges
        if i0 < i1 {
            Some(i0..i1)
        } else {
            None
        }
    }
}
