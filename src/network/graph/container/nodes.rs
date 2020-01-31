//------------------------------------------------------------------------------------------------//
// other modules

use crate::network::NodeIdx;
use crate::units::geo::Coordinate;
use std::fmt;
use std::fmt::Display;

//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct Node {
    idx: NodeIdx,
    id: i64,
    coord: Coordinate,
}

impl Node {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn idx(&self) -> NodeIdx {
        self.idx
    }

    pub fn coord(&self) -> Coordinate {
        self.coord
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.idx == other.idx
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node: {{ id: {}, idx: {}, coord: {} }}",
            self.id(),
            self.idx(),
            self.coord(),
        )
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub struct NodeContainer<'a> {
    node_ids: &'a Vec<i64>,
    node_coords: &'a Vec<Coordinate>,
}

impl<'a> NodeContainer<'a> {
    pub fn count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn id(&self, idx: NodeIdx) -> i64 {
        self.node_ids[idx.usize()]
    }

    pub fn coord(&self, idx: NodeIdx) -> Coordinate {
        self.node_coords[idx.usize()]
    }

    pub fn idx_from(&self, id: i64) -> Result<NodeIdx, NodeIdx> {
        match self.node_ids.binary_search(&id) {
            Ok(idx) => Ok(idx.into()),
            Err(idx) => Err(idx.into()),
        }
    }

    pub fn create_from(&self, id: i64) -> Option<Node> {
        let idx = match self.idx_from(id) {
            Ok(idx) => idx,
            Err(_) => return None,
        };
        Some(self.create(idx))
    }

    pub fn create(&self, idx: NodeIdx) -> Node {
        let id = self.id(idx);
        let coord = self.coord(idx);
        Node { id, idx, coord }
    }
}
