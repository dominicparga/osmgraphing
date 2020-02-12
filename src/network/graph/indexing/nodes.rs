//------------------------------------------------------------------------------------------------//
// other modules

use std::{
    cmp::{Ord, Ordering},
    fmt,
    fmt::Display,
    ops::{Add, AddAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Debug, Hash)]
pub struct NodeIdx {
    value: usize,
}

impl NodeIdx {
    pub fn new(n: usize) -> NodeIdx {
        NodeIdx { value: n }
    }

    pub fn to_usize(&self) -> usize {
        self.value
    }

    pub fn zero() -> NodeIdx {
        NodeIdx { value: 0 }
    }
}

impl Display for NodeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

//------------------------------------------------------------------------------------------------//
// ordering

impl Ord for NodeIdx {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for NodeIdx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NodeIdx {}

impl PartialEq for NodeIdx {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

//------------------------------------------------------------------------------------------------//
// operations

impl Add<NodeIdx> for NodeIdx {
    type Output = Self;

    fn add(self, other: NodeIdx) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<NodeIdx> for NodeIdx {
    fn add_assign(&mut self, other: NodeIdx) {
        self.value += other.value;
    }
}

impl Add<usize> for NodeIdx {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self {
            value: self.value + other,
        }
    }
}

impl AddAssign<usize> for NodeIdx {
    fn add_assign(&mut self, other: usize) {
        self.value += other;
    }
}
