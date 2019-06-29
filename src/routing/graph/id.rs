use std::fmt;
use std::num;
use std::str;

//--------------------------------------------------------------------------------------------------
// IDs of nodes and edges

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeId(i64);

impl str::FromStr for NodeId {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeId(s.parse::<i64>()?))
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EdgeId(i64);

impl str::FromStr for EdgeId {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EdgeId(s.parse::<i64>()?))
    }
}

//--------------------------------------------------------------------------------------------------
// fmt::Display

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
