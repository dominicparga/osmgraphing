pub mod dijkstra {
    use crate::{
        network::{HalfEdge, MetricIdx},
        routing::dijkstra::{bidirectional, unidirectional, Astar},
    };

    pub fn unidirectional(metric_idx: MetricIdx) -> Box<dyn Astar> {
        let cost_fn = move |edge: &HalfEdge| edge.metric(metric_idx).unwrap();
        Box::new(unidirectional::GenericDijkstra::new(cost_fn))
    }

    pub fn bidirectional(metric_idx: MetricIdx) -> Box<dyn Astar> {
        let cost_fn = move |edge: &HalfEdge| edge.metric(metric_idx).unwrap();
        Box::new(bidirectional::GenericDijkstra::new(cost_fn))
    }
}

pub mod astar {
    pub mod shortest {
        use crate::{
            network::{HalfEdge, MetricIdx, Node},
            routing::astar::{bidirectional, unidirectional, Astar},
            units::geo,
        };

        pub fn unidirectional(length_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(length_idx).unwrap();
            let estimate_fn =
                |from: &Node, to: &Node| *geo::haversine_distance_km(&from.coord(), &to.coord());
            Box::new(unidirectional::GenericAstar::new(cost_fn, estimate_fn))
        }

        pub fn bidirectional(length_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(length_idx).unwrap();
            let estimate_fn =
                |from: &Node, to: &Node| *geo::haversine_distance_km(&from.coord(), &to.coord());
            Box::new(bidirectional::GenericAstar::new(cost_fn, estimate_fn))
        }
    }

    pub mod fastest {
        use crate::{
            defaults,
            network::{HalfEdge, MetricIdx, Node},
            routing::astar::{bidirectional, unidirectional, Astar},
            units::{geo, speed::KilometersPerHour},
        };

        pub fn unidirectional(duration_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(duration_idx).unwrap();
            let estimate_fn = |from: &Node, to: &Node| {
                let meters = geo::haversine_distance_km(&from.coord(), &to.coord());
                let maxspeed = KilometersPerHour(defaults::speed::MAX_KMH as f32);
                *(meters / maxspeed)
            };
            Box::new(unidirectional::GenericAstar::new(cost_fn, estimate_fn))
        }

        pub fn bidirectional(duration_idx: MetricIdx) -> Box<dyn Astar> {
            let cost_fn = move |edge: &HalfEdge| edge.metric(duration_idx).unwrap();
            let estimate_fn = |from: &Node, to: &Node| {
                let meters = geo::haversine_distance_km(&from.coord(), &to.coord());
                let maxspeed = KilometersPerHour(defaults::speed::MAX_KMH as f32);
                *(meters / maxspeed)
            };
            Box::new(bidirectional::GenericAstar::new(cost_fn, estimate_fn))
        }
    }
}
