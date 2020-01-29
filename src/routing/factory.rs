//------------------------------------------------------------------------------------------------//
// other modules

use crate::network;
use network::{geo, units::Meters, Edge, Node};

use super::Astar;
use super::GenericAstar;

//------------------------------------------------------------------------------------------------//
// own modules

pub mod astar {
    use super::{geo, network, Astar, Edge, GenericAstar, Meters, Node};

    pub fn shortest() -> Box<dyn Astar> {
        let cost_fn = |edge: &Edge| edge.meters();
        let estimate_fn = |from: &Node, to: &Node| {
            let meters = geo::haversine_distance(from.coord(), to.coord()) * 1_000.0;
            Meters::from(meters as u32)
        };
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }

    pub fn fastest() -> Box<dyn Astar> {
        let cost_fn = |edge: &Edge| edge.milliseconds();
        let estimate_fn = |from: &Node, to: &Node| {
            let kilometers = geo::haversine_distance(from.coord(), to.coord());
            let maxspeed: u16 = network::defaults::MAX_SPEED_KMH.into();
            (kilometers * ((3600 / maxspeed) as f64)) as u32
        };
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }
}

pub mod dijkstra {
    use super::{Astar, Edge, GenericAstar, Node, Meters};

    pub fn shortest() -> Box<dyn Astar> {
        let cost_fn = |edge: &Edge| edge.meters();
        let estimate_fn = |_from: &Node, _to: &Node| Meters::zero();
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }

    pub fn fastest() -> Box<dyn Astar> {
        // length [m] / velocity [km/h]
        let cost_fn = |edge: &Edge| edge.milliseconds();
        let estimate_fn = |_from: &Node, _to: &Node| 0;
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }
}
