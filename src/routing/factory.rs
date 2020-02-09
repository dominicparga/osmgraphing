pub mod astar {
    use crate::{
        network,
        network::{HalfEdge, Node},
        routing::astar::{Astar, GenericAstar},
        units::{geo, length::Meters, speed::KilometersPerHour, time::Milliseconds},
    };

    pub fn shortest() -> Box<dyn Astar<Meters>> {
        let cost_fn = |edge: &HalfEdge| edge.meters();
        let estimate_fn =
            |from: &Node, to: &Node| geo::haversine_distance_m(&from.coord(), &to.coord());
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }

    pub fn fastest() -> Box<dyn Astar<Milliseconds>> {
        let cost_fn = |edge: &HalfEdge| edge.milliseconds();
        let estimate_fn = |from: &Node, to: &Node| {
            let meters = geo::haversine_distance_m(&from.coord(), &to.coord());
            let maxspeed: KilometersPerHour = (network::defaults::MAX_SPEED_KMH as u16).into();
            meters / maxspeed
        };
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }
}

pub mod dijkstra {
    use crate::{
        network::{HalfEdge, Node},
        routing::astar::{Astar, GenericAstar},
        units::{length::Meters, time::Milliseconds, Metric},
    };

    pub fn shortest() -> Box<dyn Astar<Meters>> {
        let cost_fn = |edge: &HalfEdge| edge.meters();
        let estimate_fn = |_from: &Node, _to: &Node| Meters::zero();
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }

    pub fn fastest() -> Box<dyn Astar<Milliseconds>> {
        // length [m] / velocity [km/h]
        let cost_fn = |edge: &HalfEdge| edge.milliseconds();
        let estimate_fn = |_from: &Node, _to: &Node| Milliseconds::zero();
        Box::new(GenericAstar::from(cost_fn, estimate_fn))
    }
}
