pub mod astar {
    pub mod unidirectional {
        use crate::{
            network,
            network::{HalfEdge, MetricIdx, Node},
            routing::astar::{unidirectional::GenericAstar, Astar},
            units::{geo, length::Meters, speed::KilometersPerHour, time::Milliseconds},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar<Meters>> {
            let cost_fn = move |edge: &HalfEdge| edge.length(length_idx).unwrap();
            let estimate_fn =
                |from: &Node, to: &Node| geo::haversine_distance_m(&from.coord(), &to.coord());
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar<Milliseconds>> {
            let cost_fn = move |edge: &HalfEdge| edge.duration(duration_idx).unwrap();
            let estimate_fn = |from: &Node, to: &Node| {
                let meters = geo::haversine_distance_m(&from.coord(), &to.coord());
                let maxspeed = KilometersPerHour(network::defaults::speed::MAX_KMH as u32);
                meters / maxspeed
            };
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }
    }

    pub mod bidirectional {
        use crate::{
            network,
            network::{HalfEdge, MetricIdx, Node},
            routing::astar::{bidirectional::GenericAstar, Astar},
            units::{geo, length::Meters, speed::KilometersPerHour, time::Milliseconds},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar<Meters>> {
            let cost_fn = move |edge: &HalfEdge| edge.length(length_idx).unwrap();
            let estimate_fn =
                |from: &Node, to: &Node| geo::haversine_distance_m(&from.coord(), &to.coord());
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar<Milliseconds>> {
            let cost_fn = move |edge: &HalfEdge| edge.duration(duration_idx).unwrap();
            let estimate_fn = |from: &Node, to: &Node| {
                let meters = geo::haversine_distance_m(&from.coord(), &to.coord());
                let maxspeed = KilometersPerHour(network::defaults::speed::MAX_KMH as u32);
                meters / maxspeed
            };
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }
    }
}

pub mod dijkstra {
    pub mod unidirectional {
        use crate::{
            network::{HalfEdge, MetricIdx},
            routing::dijkstra::{unidirectional::GenericAstar, Astar},
            units::{length::Meters, time::Milliseconds},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar<Meters>> {
            let cost_fn = move |edge: &HalfEdge| edge.length(length_idx).unwrap();
            Box::new(GenericAstar::new(cost_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar<Milliseconds>> {
            let cost_fn = move |edge: &HalfEdge| edge.duration(duration_idx).unwrap();
            Box::new(GenericAstar::new(cost_fn))
        }
    }

    pub mod bidirectional {
        use crate::{
            network::{HalfEdge, MetricIdx},
            routing::dijkstra::{bidirectional::GenericAstar, Astar},
            units::{length::Meters, time::Milliseconds},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar<Meters>> {
            let cost_fn = move |edge: &HalfEdge| edge.length(length_idx).unwrap();
            Box::new(GenericAstar::new(cost_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar<Milliseconds>> {
            let cost_fn = move |edge: &HalfEdge| edge.duration(duration_idx).unwrap();
            Box::new(GenericAstar::new(cost_fn))
        }
    }
}
