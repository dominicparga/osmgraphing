use crate::network;
use network::{geo, Edge, Node};

use super::Astar;

//------------------------------------------------------------------------------------------------//
// shortest path

pub fn new_shortest_path_astar() -> Astar {
    let cost_fn = Box::new(|edge: &Edge| edge.meters());
    let estimate_fn = Box::new(|from: &Node, to: &Node| {
        (geo::haversine_distance(from.coord(), to.coord()) * 1_000.0) as u32
    });
    Astar::from(cost_fn, estimate_fn)
}

pub fn new_shortest_path_dijkstra() -> Astar {
    let cost_fn = Box::new(|edge: &Edge| edge.meters());
    let estimate_fn = Box::new(|_from: &Node, _to: &Node| 0);
    Astar::from(cost_fn, estimate_fn)
}

//------------------------------------------------------------------------------------------------//
// fastest path

pub fn new_fastest_path_astar() -> Astar {
    let cost_fn = Box::new(|edge: &Edge| edge.seconds());
    let estimate_fn = Box::new(|from: &Node, to: &Node| {
        let kilometers = geo::haversine_distance(from.coord(), to.coord());
        let maxspeed = network::defaults::MAX_SPEED_KMH;
        (kilometers * ((3600 / maxspeed) as f64)) as u32
    });
    Astar::from(cost_fn, estimate_fn)
}

pub fn new_fastest_path_dijkstra() -> Astar {
    // length [m] / velocity [km/h]
    let cost_fn = Box::new(|edge: &Edge| edge.seconds());
    let estimate_fn = Box::new(|_from: &Node, _to: &Node| 0);
    Astar::from(cost_fn, estimate_fn)
}
