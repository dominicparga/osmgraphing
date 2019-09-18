use crate::network;
use network::{geo, Edge, Node};

use super::Astar;
use super::GenericAstar;

//------------------------------------------------------------------------------------------------//
// shortest path

pub fn new_shortest_path_astar() -> Box<dyn Astar> {
    let cost_fn = |edge: &Edge| edge.meters();
    let estimate_fn = |from: &Node, to: &Node| {
        (geo::haversine_distance(from.coord(), to.coord()) * 1_000.0) as u32
    };
    Box::new(GenericAstar::from(cost_fn, estimate_fn))
}

pub fn new_shortest_path_dijkstra() -> Box<dyn Astar> {
    let cost_fn = |edge: &Edge| edge.meters();
    let estimate_fn = |_from: &Node, _to: &Node| 0;
    Box::new(GenericAstar::from(cost_fn, estimate_fn))
}

//------------------------------------------------------------------------------------------------//
// fastest path

pub fn new_fastest_path_astar() -> Box<dyn Astar> {
    let cost_fn = |edge: &Edge| edge.milliseconds();
    let estimate_fn = |from: &Node, to: &Node| {
        let kilometers = geo::haversine_distance(from.coord(), to.coord());
        let maxspeed = network::defaults::MAX_SPEED_KMH;
        (kilometers * ((3600 / maxspeed) as f64)) as u32
    };
    Box::new(GenericAstar::from(cost_fn, estimate_fn))
}

pub fn new_fastest_path_dijkstra() -> Box<dyn Astar> {
    // length [m] / velocity [km/h]
    let cost_fn = |edge: &Edge| edge.milliseconds();
    let estimate_fn = |_from: &Node, _to: &Node| 0;
    Box::new(GenericAstar::from(cost_fn, estimate_fn))
}
