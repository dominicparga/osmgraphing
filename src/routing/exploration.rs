// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use super::paths::Path;
use crate::{
    configs::routing::Config,
    helpers,
    network::{EdgeIdx, Graph, Node, NodeIdx},
};
use nd_triangulation;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct ConvexHullExplorator {}

impl ConvexHullExplorator {
    pub fn explore(&self, src_idx: NodeIdx, dst_idx: NodeIdx) {
        let mut has_new_route = true;

        while has_new_route {
            has_new_route = false;

            // check every facet, if it is already sharp enough
        }
    }
}
