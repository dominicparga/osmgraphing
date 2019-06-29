use std::fmt;
use std::num;
use std::ops;
use std::str;

use super::{Node, Edge};

//--------------------------------------------------------------------------------------------------
// Indices of nodes

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeIdx(usize);

impl str::FromStr for NodeIdx {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeIdx(s.parse::<usize>()?))
    }
}

impl ops::Index<NodeIdx> for Vec<Node> {
    type Output = Node;

    fn index(&self, idx: NodeIdx) -> &Self::Output {
        &self[idx.0]
    }
}

impl ops::IndexMut<NodeIdx> for Vec<u64> {
    type Output = u64;

    fn index_mut(&mut self, index: NodeIdx) -> &mut Self::Output {
        
    }
}

//--------------------------------------------------------------------------------------------------
// Indices of edges

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EdgeIdx(usize);

impl str::FromStr for EdgeIdx {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EdgeIdx(s.parse::<usize>()?))
    }
}

impl ops::Index<EdgeIdx> for Vec<Edge> {
    type Output = Edge;

    fn index(&self, idx: EdgeIdx) -> &Self::Output {
        &self[idx.0]
    }
}

//--------------------------------------------------------------------------------------------------
// fmt::Display

impl fmt::Display for NodeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EdgeIdx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
