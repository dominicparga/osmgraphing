pub mod dijkstra {
    pub mod unidirectional {
        use crate::{
            network::{HalfEdge, MetricIdx},
            routing::dijkstra::{unidirectional::GenericDijkstra, Astar},
            units::{length::Meters, time::Milliseconds},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar<Meters>> {
            let cost_fn = move |edge: &HalfEdge| edge.length(length_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar<Milliseconds>> {
            let cost_fn = move |edge: &HalfEdge| edge.duration(duration_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }

        pub fn multi(metric_indices: Vec<MetricIdx>) -> Box<dyn Astar<u32>> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(metric_indices[0]).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }
    }

    pub mod bidirectional {
        use crate::{
            network::{HalfEdge, MetricIdx},
            routing::dijkstra::{bidirectional::GenericDijkstra, Astar},
            units::{length::Meters, time::Milliseconds},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar<Meters>> {
            let cost_fn = move |edge: &HalfEdge| edge.length(length_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar<Milliseconds>> {
            let cost_fn = move |edge: &HalfEdge| edge.duration(duration_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }
    }
}

pub mod astar {
    pub mod unidirectional {
        use crate::{
            defaults,
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
                let maxspeed = KilometersPerHour(defaults::speed::MAX_KMH as u32);
                meters / maxspeed
            };
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }
    }

    pub mod bidirectional {
        use crate::{
            defaults,
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
                let maxspeed = KilometersPerHour(defaults::speed::MAX_KMH as u32);
                meters / maxspeed
            };
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }
    }
}
