use std::{
    cmp::Ord,
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

//------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeIdx(pub usize);

impl Display for NodeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for NodeIdx {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl DerefMut for NodeIdx {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EdgeIdx(pub usize);

impl Display for EdgeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for EdgeIdx {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl DerefMut for EdgeIdx {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetricIdx(pub usize);

impl Display for MetricIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for MetricIdx {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

impl DerefMut for MetricIdx {
    fn deref_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}
