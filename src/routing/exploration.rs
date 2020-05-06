// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use crate::{
    configs,
    defaults::capacity::DimVec,
    helpers::{self, algebra},
    network::{Graph, NodeIdx},
    routing::{paths::Path, Dijkstra},
};
use log::debug;
use smallvec::smallvec;

pub struct ConvexHullExplorator {}

impl ConvexHullExplorator {
    pub fn new() -> ConvexHullExplorator {
        ConvexHullExplorator {}
    }

    // TODO cap exploration with epsilon for routing-costs (1 + eps) * costs[i]
    //
    // New paths of a facet are linear-combinations of its defining paths
    // -> could not be better than the best of already defined paths

    pub fn fully_explorate(
        &mut self,
        src_idx: NodeIdx,
        dst_idx: NodeIdx,
        dijkstra: &mut Dijkstra,
        graph: &Graph,
        routing_cfg: &configs::routing::Config,
    ) -> Vec<Path> {
        // init query

        // config and stuff
        let mut routing_cfg = routing_cfg.clone();
        let dim = graph.metrics().dim();
        // Every cost-value has to be below this value.
        let mut tolerated = smallvec![std::f64::INFINITY; dim];
        // don't consider ignored metrics
        let is_metric_considered: DimVec<_> = routing_cfg
            .alphas
            .iter()
            .map(|alpha| alpha > &0.0)
            .collect();
        let considered_dim = is_metric_considered
            .iter()
            .filter(|&&is_considered| is_considered)
            .count();
        debug!("is-metric-considered: {:?}", is_metric_considered);

        // return these paths in the end
        let mut found_paths = Vec::new();
        let mut candidates: Vec<DimVec<_>> = Vec::new();

        // find initial convex-hull
        // adding d dirac-points
        // and adding an average point as point number d+1

        for i in 0..dim {
            if !is_metric_considered[i] {
                continue;
            }

            // prepare dirac-paths
            // alphas = [0, ..., 0, 1, 0, ..., 0] (f64)
            routing_cfg.alphas = smallvec![0.0; dim];
            routing_cfg.alphas[i] = 1.0;

            debug!("use dirac-alpha {:?}", routing_cfg.alphas);

            // and if path exists
            // -> remember it as convex-hull-member

            if let Some(best_path) =
                dijkstra.compute_best_path(src_idx, dst_idx, graph, &routing_cfg)
            {
                let best_path = best_path.flatten(graph);
                // Remember tolerated costs for filtering in the end.
                // The costs have to be checked in the end, since this iterative algorithm could
                // find a tolerated path by using a unacceptable path.
                tolerated[i] = best_path.costs()[i] * routing_cfg.tolerated_scales[i];

                // if metric should be considered and path has not been added
                // -> remember path
                if !found_paths.contains(&best_path) {
                    found_paths.push(best_path);
                    debug!("pushed {}", found_paths.last().unwrap());
                }
            }
        }

        // If not enough different paths have been found
        // -> return already found paths by keeping candidates empty

        if considered_dim > 1 && found_paths.len() == considered_dim {
            candidates.push((0..found_paths.len()).collect());
        }

        // find new routes

        while let Some(candidate) = candidates.pop() {
            debug!("LOOP with {} possible candidate(s)", candidates.len() + 1);

            // check candidate, if it's shape already sharp enough

            // Solve LGS to get alpha, where all cell-vertex-costs (personalized with alpha)
            // are equal.
            // -> Determine rows of matrix

            let mut rows = DimVec::new();
            let mut b = DimVec::new();

            // one condition is missing, namely normalizing alpha
            // -> first row in matrix is 1.0
            rows.push(smallvec![1.0; dim]);
            b.push(1.0);

            // all lines describe the equality of each dot-product between cost-vector and alpha
            for i in 1..candidate.len() {
                rows.push(helpers::sub(
                    found_paths[candidate[0]].costs(),
                    found_paths[candidate[i]].costs(),
                ));
                b.push(0.0);
            }

            // but ignored metrics should lead to zero alpha
            for i in 0..is_metric_considered.len() {
                if !is_metric_considered[i] {
                    // set [0, ..., 0, 1, 0, ..., 0] to 0.0
                    let mut row = smallvec![0.0; dim];
                    row[i] = 1.0;
                    rows.push(row);
                    b.push(0.0);
                }
            }

            debug!("rows = {:?}", rows);
            debug!("b = {:?}", b);

            // calculate alphas
            // TODO check if points lay on a line
            // <-> matrix has rows of 0 and hence infinite solutions
            routing_cfg.alphas = if let Some(x) = algebra::Matrix::from_rows(rows).lu().solve(&b) {
                // x.iter_mut().for_each(|alpha| {
                //     if *alpha < 0.0 {
                //         *alpha = 0.0;
                //     }
                // });
                x
            } else {
                continue;
            };
            debug!("alphas = {:?}", routing_cfg.alphas);
            for i in 0..candidate.len() {
                debug!(
                    "alphas * costs[c{}] = {:?}",
                    i,
                    helpers::dot_product(&routing_cfg.alphas, found_paths[candidate[i]].costs())
                );
            }

            // find new path with new alpha

            if let Some(best_path) =
                dijkstra.compute_best_path(src_idx, dst_idx, graph, &routing_cfg)
            {
                let new_p = best_path.flatten(graph);
                debug!(
                    "alphas * new_costs = {:?}",
                    helpers::dot_product(&routing_cfg.alphas, new_p.costs())
                );

                // Check if path has already been found.
                // If so
                // -> Candidate's shape, meaning the respective partial convex-hull, is complete
                {
                    let mut is_already_found = false;

                    for &i in candidate.iter() {
                        if found_paths[i] == new_p {
                            is_already_found = true;
                            break;
                        }
                    }

                    if is_already_found {
                        debug!("already found path {}", new_p);
                        continue;
                    }
                }

                // Otherwise, add the new path and resulting new candidates.
                // Given is the cost-space and the goal is finding the extreme-points of the convex-hull of all paths in cost-space, which can be "seen" from the origin (since Dijkstra wants the shortest paths).
                // Given that the facet-paths (the initial dirac-paths from the beginning) are part of the convex-hull, a new path has to be part of the convex-hull as well, or Dijkstra would have found one of the already found paths in the first place.
                //
                //
                // ## Proof of correctness
                //
                // Note that imaginating a 2D-cost-space could help.
                //
                // Given a `d`-dimensional facet, whose `d` corners are part of the convex-hull, this facet is a hyperplane and thus has a normal-vector `n`.
                // By subtracting the `d-1` neighbors of an arbitrary chosen corner, an underdetermined linear-equation-system can be built and solved, resulting in a line orthogonal to the hyperplane of direction `n`.
                // Further, the dot-product is the orthogonal projection of a vector `v` onto another vector `w`, times the length of w.
                // When comparing two dot-products, where `w` is equal, it is just a comparison of the projections themselves.
                // This statement implies an equal dot-product for each corner of the `d`-dimensional facet with the normal-vector `n`.
                // Since the dot-product uses its vectors as position-vectors, starting from the origin, a shorter dot-product could only be found with a new position-vector, which is outside the facet towards the origin.
                // Interpreting `n` as alpha-vector, this new position-vector would be a cost-vector of a path found by Dijkstra, since Dijkstra optimizes the dot-product between these two vectors, and alpha is fixed.
                // This new position-vector points to a point in cost-space, which is the point of maximum distance to the current facet (towards origin).
                // Every new point in cost-space lays on the side of every previously found facet-hyperplane, which is not orientated towards the origin, or the point would have been found earlier.
                // Therefore, this procedure results in a set of cost-vectors, which are extreme-points of the convex-hull of all cost-vectors for this particular src-dst-pair.
                //
                //
                // ## Proof of completeness
                //
                // Remaining question is whether the procedure finds all cost-vectors.
                // Let there exist a point `q` in `d`-dimensional cost-space, a cost-vector, inside the `d` border-hyperplanes of an initial convex-hull (dirac-paths).
                // If the point `q` was outside the bounds, it would be part of the initial convex-hull as cost-vector of a dirac-path.
                // Further, let this point `q` be an extreme-point of the convex-hull.
                //
                // Every new alpha leads to a point of maximum distance to the current facet, towards origin due to the definition of the dot-product.
                // Hence, the point `q` has a smaller distance to the current facet and lays inside the yet found facet-hyperplanes, or it would have already been found.
                // This argument holds for every further step.
                // Every step is either the last step of this local convex-hull, if no new point of higher distance exists, or it shrinks the search-space due to new facet-hyperplanes.
                //
                // Thus, `q` will eventually be found.
                // The only exception is the case, where more than `d` paths lay on a facet.
                // Here, it is implementation- and graph-instance-dependent, which of multiple paths of same cost a typical Dijkstra would find.
                // Further, the linear-equation-system has to be aware of the case, where all `d` facet-paths lay on a hyperplane of lower dimension (e.g. on a line in three dimensions).
                //
                //
                // remember path
                found_paths.push(new_p);
                debug!("found path: {}", found_paths.last().unwrap());
                debug!("found paths: {}", found_paths.len());

                // Add new facets by replacing every cost with the new path's cost.
                for i in 0..candidate.len() {
                    let mut new_candidate = candidate.clone();
                    new_candidate[i] = found_paths.len() - 1;
                    candidates.push(new_candidate);
                }
            }
        }

        found_paths
            .into_iter()
            .filter(|p| p.costs() <= &tolerated)
            .collect()
    }
}
