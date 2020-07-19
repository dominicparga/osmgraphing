// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use crate::{
    configs,
    defaults::{self, capacity::DimVec},
    helpers::{self, algebra},
    network::{Graph, NodeIdx},
    routing::{
        dijkstra::{self, Dijkstra},
        paths::Path,
    },
};
use log::{debug, trace};
use smallvec::smallvec;
use std::ops::Deref;
mod wrong_alternative;
use wrong_alternative::{Config, ConvexHull};

// needed because convex-hull has dim+1 points per cell
pub type CHDimVec<T> = smallvec::SmallVec<[T; defaults::capacity::SMALL_VEC_INLINE_SIZE + 1]>;

struct Query<'a> {
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    graph: &'a Graph,
    routing_cfg: configs::routing::Config,
    graph_dim: usize,
    tolerances: DimVec<f64>,
    is_metric_considered: DimVec<bool>,
}

impl<'a> Query<'a> {
    fn with(query: dijkstra::Query<'a>) -> Query<'a> {
        // init query

        let src_idx = query.src_idx;
        let dst_idx = query.dst_idx;
        let graph = query.graph;
        let routing_cfg = query.routing_cfg.clone();

        // config and stuff
        let dim = graph.metrics().dim();
        // Every cost-value has to be below this value.
        let tolerances: DimVec<_> = smallvec![defaults::routing::TOLERATED_SCALE_INF; dim];
        // don't consider ignored metrics
        let is_metric_considered: DimVec<_> = routing_cfg
            .alphas
            .iter()
            .map(|alpha| alpha > &0.0)
            .collect();
        debug!("is_metric_considered: {:?}", is_metric_considered);

        Query {
            src_idx,
            dst_idx,
            graph,
            routing_cfg,
            graph_dim: dim,
            tolerances,
            is_metric_considered,
        }
    }
}

#[derive(Copy, Clone)]
struct CandidateId(usize);

impl Deref for CandidateId {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

#[derive(Clone)]
struct Candidate {
    ids: CHDimVec<CandidateId>,
}

impl Candidate {
    fn len(&self) -> usize {
        self.ids.len()
    }
}

pub struct ConvexHullExplorator {
    convex_hull: ConvexHull,
}

impl ConvexHullExplorator {
    pub fn new() -> ConvexHullExplorator {
        ConvexHullExplorator {
            convex_hull: ConvexHull::with(Config { dim: 0 }),
        }
    }

    fn init_query(&mut self, query: &Query) {
        self.convex_hull.init_query(Config {
            dim: query
                .is_metric_considered
                .iter()
                .filter(|&&is_considered| is_considered)
                .count(),
        })
    }

    fn explore_initial_paths(&mut self, query: &mut Query, dijkstra: &mut Dijkstra) {
        // find initial convex-hull
        // adding d initial points

        let mut init_alphas: CHDimVec<_> = CHDimVec::new();
        // and adding an average point as point number dim+1
        // -> remember for later
        let mut avg_alphas: DimVec<f64> = smallvec![0.0; query.graph_dim];

        // add peak-alphas ([0, ..., 0, 1, 0, ..., 0]) for considered metrics

        for metric_idx in
            (0..query.graph_dim).filter(|&metric_idx| query.is_metric_considered[metric_idx])
        {
            let mut alphas = smallvec![0.0; query.graph_dim];
            alphas[metric_idx] = 1.0;
            init_alphas.push((Some(metric_idx), alphas));

            avg_alphas[metric_idx] = 1.0;
        }

        // plus add avg-alpha, because convex-hull needs at least dim+1 points
        // init_alphas.push((None, avg_alphas)); // TODO uncomment for cyclops

        for (metric_idx, alphas) in init_alphas {
            debug!("use init-alpha {:?}", alphas);

            query.routing_cfg.alphas = alphas;
            if let Some(mut best_path) = dijkstra.compute_best_path(dijkstra::Query {
                src_idx: query.src_idx,
                dst_idx: query.dst_idx,
                graph: query.graph,
                routing_cfg: &query.routing_cfg,
            }) {
                best_path.calc_costs(query.graph);

                // Remember tolerated costs for filtering in the end.
                // The costs have to be checked in the end, since this iterative algorithm could
                // find a tolerated path by using an unacceptable path.

                if let Some(metric_idx) = metric_idx {
                    if query.routing_cfg.tolerated_scales[metric_idx] == std::f64::INFINITY {
                        query.tolerances[metric_idx] = std::f64::INFINITY;
                    } else {
                        // NaN when 0.0 * inf
                        query.tolerances[metric_idx] = best_path.costs()[metric_idx]
                            * query.routing_cfg.tolerated_scales[metric_idx];
                    }
                }

                if !self.convex_hull.contains(&best_path) {
                    self.convex_hull.push_path(best_path);
                }
            }
        }

        if self.convex_hull.has_volume() {
            self.convex_hull.init_candidates();
        }
    }

    fn create_linear_system(
        &self,
        candidate: &Candidate,
        query: &Query,
    ) -> (DimVec<DimVec<f64>>, DimVec<f64>) {
        // Solve LGS to get alpha, where all cell-vertex-costs (personalized with alpha)
        // are equal.
        // -> Determine rows of matrix

        let mut rows = DimVec::new();
        let mut b = DimVec::new();

        // all lines describe the equality of each dot-product between cost-vector and alpha
        for i in 1..(candidate.len()) {
            rows.push(helpers::sub(
                self.convex_hull.path_from(candidate.ids[0]).costs(),
                self.convex_hull.path_from(candidate.ids[i]).costs(),
            ));
            b.push(0.0);
        }

        // but ignored metrics should lead to zero alpha
        for i in 0..query.is_metric_considered.len() {
            if !query.is_metric_considered[i] {
                // set [0, ..., 0, 1, 0, ..., 0] to 0.0
                let mut row = smallvec![0.0; query.graph_dim];
                row[i] = 1.0;
                rows.push(row);
                b.push(0.0);
            }
        }

        // if one condition is missing (depending on convex-hull-implementation),
        if rows.len() < query.graph_dim {
            // you could normalize alpha
            // -> one row in matrix is 1.0

            rows.push(smallvec![1.0; query.graph_dim]);
            b.push(1.0);
        }

        trace!("rows = {:?}", rows);
        trace!("b = {:?}", b);

        if rows.len() < self.convex_hull.dim() {
            panic!(
                "{}{}",
                "The linear system has less rows than the convex-hull has dimensions.",
                "This doesn't lead to a unique solution."
            )
        }

        (rows, b)
    }

    // TODO cap exploration with epsilon for routing-costs (1 + eps) * costs[i]
    //
    // New paths of a facet are linear-combinations of its defining paths
    // -> could not be better than the best of already defined paths

    pub fn fully_explorate(
        &mut self,
        query: dijkstra::Query,
        dijkstra: &mut Dijkstra,
    ) -> Vec<Path> {
        let mut query = Query::with(query);
        self.init_query(&query);
        self.explore_initial_paths(&mut query, dijkstra);

        // find new routes

        while let Some(candidate) = self.convex_hull.pop_candidate() {
            // check candidate, if it's shape already sharp enough

            let (rows, b) = self.create_linear_system(&candidate, &query);

            // calculate alphas
            query.routing_cfg.alphas =
                if let Some(x) = algebra::Matrix::from_rows(rows).lu().solve(&b) {
                    x
                } else {
                    continue;
                };
            trace!("alphas = {:?}", query.routing_cfg.alphas);
            for i in 0..candidate.len() {
                trace!(
                    "alphas * costs[c{}] = {:?}",
                    i,
                    helpers::dot_product(
                        &query.routing_cfg.alphas,
                        self.convex_hull.path_from(candidate.ids[i]).costs(),
                    )
                );
            }

            // find new path with new alpha

            if let Some(mut best_path) = dijkstra.compute_best_path(dijkstra::Query {
                src_idx: query.src_idx,
                dst_idx: query.dst_idx,
                graph: query.graph,
                routing_cfg: &query.routing_cfg,
            }) {
                best_path.calc_costs(query.graph);
                let new_p = best_path;
                trace!(
                    "alphas * new_costs = {:?}",
                    helpers::dot_product(&query.routing_cfg.alphas, new_p.costs())
                );

                self.convex_hull.update(new_p, candidate);
            }
        }

        self.convex_hull
            .found_paths()
            .iter()
            .filter_map(|p| {
                if helpers::le(p.costs(), &query.tolerances) {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
