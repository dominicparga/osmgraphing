use std::{
    cmp::Ord,
    fmt::{self, Display},
    ops::{Deref, DerefMut, Range},
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

pub struct NodeIdxIterator {
    range: Range<usize>,
}

impl Iterator for NodeIdxIterator {
    type Item = NodeIdx;

    fn next(&mut self) -> Option<NodeIdx> {
        Some(NodeIdx(self.range.next()?))
    }
}

impl From<Range<NodeIdx>> for NodeIdxIterator {
    fn from(range: Range<NodeIdx>) -> NodeIdxIterator {
        NodeIdxIterator {
            range: range.start.0..range.end.0,
        }
    }
}

impl From<Range<usize>> for NodeIdxIterator {
    fn from(range: Range<usize>) -> NodeIdxIterator {
        NodeIdxIterator { range }
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

pub struct EdgeIdxIterator {
    range: Range<usize>,
}

impl Iterator for EdgeIdxIterator {
    type Item = EdgeIdx;

    fn next(&mut self) -> Option<EdgeIdx> {
        Some(EdgeIdx(self.range.next()?))
    }
}

impl From<Range<EdgeIdx>> for EdgeIdxIterator {
    fn from(range: Range<EdgeIdx>) -> EdgeIdxIterator {
        EdgeIdxIterator {
            range: range.start.0..range.end.0,
        }
    }
}

impl From<Range<usize>> for EdgeIdxIterator {
    fn from(range: Range<usize>) -> EdgeIdxIterator {
        EdgeIdxIterator { range }
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
