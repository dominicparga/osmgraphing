use std::{
    cmp::{Ord, Ordering},
    fmt,
    fmt::Display,
    ops::{Add, AddAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Debug, Hash)]
pub struct EdgeIdx {
    value: usize,
}

impl EdgeIdx {
    pub fn new(n: usize) -> EdgeIdx {
        EdgeIdx { value: n }
    }

    pub fn to_usize(&self) -> usize {
        self.value
    }

    pub fn zero() -> EdgeIdx {
        EdgeIdx { value: 0 }
    }
}

impl Display for EdgeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
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
// operations

impl Add<EdgeIdx> for EdgeIdx {
    type Output = Self;

    fn add(self, other: EdgeIdx) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<EdgeIdx> for EdgeIdx {
    fn add_assign(&mut self, other: EdgeIdx) {
        self.value += other.value;
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

impl AddAssign<usize> for EdgeIdx {
    fn add_assign(&mut self, other: usize) {
        self.value += other;
    }
}
