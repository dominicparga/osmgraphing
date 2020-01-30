//------------------------------------------------------------------------------------------------//
// other modules

//------------------------------------------------------------------------------------------------//
// own modules

pub use edges::EdgeIdx;
pub use nodes::NodeIdx;

//------------------------------------------------------------------------------------------------//
// nodes

mod nodes {
    //--------------------------------------------------------------------------------------------//
    // other modules

    use super::EdgeIdx;
    use std::cmp::{Ord, Ordering};
    use std::fmt;
    use std::fmt::Display;
    use std::ops::{Add, Index, IndexMut};

    //--------------------------------------------------------------------------------------------//
    // basic stuff

    #[derive(Copy, Clone, Debug, Hash)]
    pub struct NodeIdx {
        value: usize,
    }

    impl NodeIdx {
        pub fn zero() -> NodeIdx {
            NodeIdx { value: 0 }
        }

        pub fn from<I: Into<usize>>(n: I) -> NodeIdx {
            NodeIdx { value: n.into() }
        }
    }

    impl Display for NodeIdx {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.value.fmt(f)
        }
    }

    //--------------------------------------------------------------------------------------------//
    // conversion from/to usize

    impl Into<usize> for NodeIdx {
        fn into(self) -> usize {
            self.value
        }
    }

    impl From<usize> for NodeIdx {
        fn from(idx: usize) -> Self {
            NodeIdx { value: idx }
        }
    }

    //--------------------------------------------------------------------------------------------//
    // iterating

    impl Iterator for NodeIdx {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.value += 1;
            Some(self.value)
        }
    }

    //--------------------------------------------------------------------------------------------//
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

    //--------------------------------------------------------------------------------------------//
    // indexing

    impl Index<NodeIdx> for Vec<NodeIdx> {
        type Output = NodeIdx;

        fn index(&self, idx: NodeIdx) -> &Self::Output {
            let idx: usize = idx.into();
            &self[idx]
        }
    }

    impl IndexMut<NodeIdx> for Vec<NodeIdx> {
        fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
            let idx: usize = idx.into();
            &mut self[idx]
        }
    }

    impl Index<NodeIdx> for Vec<Option<NodeIdx>> {
        type Output = Option<NodeIdx>;

        fn index(&self, idx: NodeIdx) -> &Self::Output {
            let idx: usize = idx.into();
            &self[idx]
        }
    }

    impl IndexMut<NodeIdx> for Vec<Option<NodeIdx>> {
        fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
            let idx: usize = idx.into();
            &mut self[idx]
        }
    }

    impl Index<NodeIdx> for Vec<EdgeIdx> {
        type Output = EdgeIdx;

        fn index(&self, idx: NodeIdx) -> &Self::Output {
            let idx: usize = idx.into();
            &self[idx]
        }
    }

    impl IndexMut<NodeIdx> for Vec<EdgeIdx> {
        fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
            let idx: usize = idx.into();
            &mut self[idx]
        }
    }

    impl Index<NodeIdx> for Vec<Option<EdgeIdx>> {
        type Output = Option<EdgeIdx>;

        fn index(&self, idx: NodeIdx) -> &Self::Output {
            let idx: usize = idx.into();
            &self[idx]
        }
    }

    impl IndexMut<NodeIdx> for Vec<Option<EdgeIdx>> {
        fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
            let idx: usize = idx.into();
            &mut self[idx]
        }
    }

    //--------------------------------------------------------------------------------------------//
    // operations

    impl Add<NodeIdx> for NodeIdx {
        type Output = Self;

        fn add(self, other: NodeIdx) -> Self {
            Self {
                value: self.value + other.value,
            }
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
}

//------------------------------------------------------------------------------------------------//
// edges

mod edges {
    #[derive(Debug)]
    pub struct EdgeIdx {}
}
