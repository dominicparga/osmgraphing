//------------------------------------------------------------------------------------------------//
// other modules

use super::NodeIdx;
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Index, IndexMut};

//------------------------------------------------------------------------------------------------//
// basic stuff

#[derive(Copy, Clone, Debug, Hash)]
pub struct EdgeIdx {
    value: usize,
}

impl EdgeIdx {
    pub fn usize(&self) -> usize {
        self.value
    }

    pub fn zero() -> EdgeIdx {
        EdgeIdx { value: 0 }
    }

    pub fn from<I: Into<usize>>(n: I) -> EdgeIdx {
        EdgeIdx { value: n.into() }
    }
}

impl Display for EdgeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

//------------------------------------------------------------------------------------------------//
// conversion from/to usize

impl Into<usize> for EdgeIdx {
    fn into(self) -> usize {
        self.value
    }
}

impl From<usize> for EdgeIdx {
    fn from(idx: usize) -> Self {
        EdgeIdx { value: idx }
    }
}

//------------------------------------------------------------------------------------------------//
// ordering

impl Ord for EdgeIdx {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for EdgeIdx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for EdgeIdx {}

impl PartialEq for EdgeIdx {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

//------------------------------------------------------------------------------------------------//
// indexing

impl Index<EdgeIdx> for Vec<EdgeIdx> {
    type Output = EdgeIdx;

    fn index(&self, idx: EdgeIdx) -> &Self::Output {
        let idx: usize = idx.into();
        &self[idx]
    }
}

impl IndexMut<EdgeIdx> for Vec<EdgeIdx> {
    fn index_mut(&mut self, idx: EdgeIdx) -> &mut Self::Output {
        let idx: usize = idx.into();
        &mut self[idx]
    }
}

impl Index<EdgeIdx> for Vec<Option<EdgeIdx>> {
    type Output = Option<EdgeIdx>;

    fn index(&self, idx: EdgeIdx) -> &Self::Output {
        let idx: usize = idx.into();
        &self[idx]
    }
}

impl IndexMut<EdgeIdx> for Vec<Option<EdgeIdx>> {
    fn index_mut(&mut self, idx: EdgeIdx) -> &mut Self::Output {
        let idx: usize = idx.into();
        &mut self[idx]
    }
}

impl Index<EdgeIdx> for Vec<NodeIdx> {
    type Output = NodeIdx;

    fn index(&self, idx: EdgeIdx) -> &Self::Output {
        let idx: usize = idx.into();
        &self[idx]
    }
}

impl IndexMut<EdgeIdx> for Vec<NodeIdx> {
    fn index_mut(&mut self, idx: EdgeIdx) -> &mut Self::Output {
        let idx: usize = idx.into();
        &mut self[idx]
    }
}

impl Index<EdgeIdx> for Vec<Option<NodeIdx>> {
    type Output = Option<NodeIdx>;

    fn index(&self, idx: EdgeIdx) -> &Self::Output {
        let idx: usize = idx.into();
        &self[idx]
    }
}

impl IndexMut<EdgeIdx> for Vec<Option<NodeIdx>> {
    fn index_mut(&mut self, idx: EdgeIdx) -> &mut Self::Output {
        let idx: usize = idx.into();
        &mut self[idx]
    }
}

//------------------------------------------------------------------------------------------------//
// operations

impl Add<EdgeIdx> for EdgeIdx {
    type Output = Self;

    fn add(self, other: EdgeIdx) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<usize> for EdgeIdx {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self {
            value: self.value + other,
        }
    }
}
