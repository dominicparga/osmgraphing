pub mod dijkstra {
    pub mod unidirectional {
        use crate::{
            network::{HalfEdge, MetricIdx},
            routing::dijkstra::{unidirectional::GenericDijkstra, Astar},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(length_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(duration_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }
    }

    pub mod bidirectional {
        use crate::{
            network::{HalfEdge, MetricIdx},
            routing::dijkstra::{bidirectional::GenericDijkstra, Astar},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(length_idx).unwrap();
            Box::new(GenericDijkstra::new(cost_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(duration_idx).unwrap();
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
            units::{geo, speed::KilometersPerHour},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(length_idx).unwrap();
            let estimate_fn =
                |from: &Node, to: &Node| *geo::haversine_distance_km(&from.coord(), &to.coord());
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(duration_idx).unwrap();
            let estimate_fn = |from: &Node, to: &Node| {
                let meters = geo::haversine_distance_km(&from.coord(), &to.coord());
                let maxspeed = KilometersPerHour(defaults::speed::MAX_KMH as f32);
                *(meters / maxspeed)
            };
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }
    }

    pub mod bidirectional {
        use crate::{
            defaults,
            network::{HalfEdge, MetricIdx, Node},
            routing::astar::{bidirectional::GenericAstar, Astar},
            units::{geo, speed::KilometersPerHour},
        };

        pub fn shortest(length_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(length_idx).unwrap();
            let estimate_fn =
                |from: &Node, to: &Node| *geo::haversine_distance_km(&from.coord(), &to.coord());
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }

        pub fn fastest(duration_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(duration_idx).unwrap();
            let estimate_fn = |from: &Node, to: &Node| {
                let meters = geo::haversine_distance_km(&from.coord(), &to.coord());
                let maxspeed = KilometersPerHour(defaults::speed::MAX_KMH as f32);
                *(meters / maxspeed)
            };
            Box::new(GenericAstar::new(cost_fn, estimate_fn))
        }
    }
}
