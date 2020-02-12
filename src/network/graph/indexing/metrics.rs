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
pub struct MetricIdx {
    value: usize,
}

impl MetricIdx {
    pub fn new(n: usize) -> MetricIdx {
        MetricIdx { value: n }
    }

    pub fn to_usize(&self) -> usize {
        self.value
    }

    pub fn zero() -> MetricIdx {
        MetricIdx { value: 0 }
    }
}

impl Display for MetricIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

//------------------------------------------------------------------------------------------------//
// ordering

impl Ord for MetricIdx {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for MetricIdx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MetricIdx {}

impl PartialEq for MetricIdx {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

//------------------------------------------------------------------------------------------------//
// operations

impl Add<MetricIdx> for MetricIdx {
    type Output = Self;

    fn add(self, other: MetricIdx) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<MetricIdx> for MetricIdx {
    fn add_assign(&mut self, other: MetricIdx) {
        self.value += other.value;
    }
}

impl Add<usize> for MetricIdx {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self {
            value: self.value + other,
        }
    }
}

impl AddAssign<usize> for MetricIdx {
    fn add_assign(&mut self, other: usize) {
        self.value += other;
    }
}
