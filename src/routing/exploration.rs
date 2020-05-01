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
    ) -> Result<Vec<Path>, String> {
        // init query

        // config and stuff
        let mut routing_cfg = routing_cfg.clone();
        let dim = graph.metrics().dim();

        // return these paths in the end
        let mut found_paths = Vec::new();
        let mut candidates: Vec<DimVec<_>> = Vec::new();

        // find initial convex-hull

        for i in 0..dim {
            // prepare dirac-paths
            // alphas = [0, ..., 0, 1, 0, ..., 0] (f64)
            routing_cfg.alphas = smallvec![0.0; dim];
            routing_cfg.alphas[i] = 1.0;

            // and if path exists
            // -> remember it as convex-hull-member

            if let Some(best_path) =
                dijkstra.compute_best_path(src_idx, dst_idx, graph, &routing_cfg)
            {
                found_paths.push(best_path.flatten(graph));
                debug!("pushed {}", found_paths.last().unwrap());
            } else {
                debug!("didn't push anything");
            }
        }

        // If not enough different paths have been found
        // -> return already found paths by keeping candidates empty

        found_paths.dedup();
        if dim > 1 && found_paths.len() == dim {
            candidates.push((0..dim).collect());
        }

        // find new routes

        while let Some(candidate) = candidates.pop() {
            debug!("LOOP with {} possible candidate(s)", candidates.len() + 1);

            // check candidate, if it's shape already sharp enough

            // Solve LGS to get alpha, where all cell-vertex-costs (personalized with alpha)
            // are equal.
            // -> Determine rows of matrix

            let mut rows = DimVec::new();
            for i in 1..dim {
                rows.push(helpers::sub(
                    found_paths[candidate[0]].costs(),
                    found_paths[candidate[i]].costs(),
                ));
            }
            // one condition is missing, namely normalizing alpha
            // -> last row in matrix is 1.0
            rows.push(smallvec![1.0; dim]);

            // solution-vector
            let mut b = smallvec![0.0; dim];
            // normalizing sum of alphas to 1.0
            // -> last row is 1.0
            b[dim - 1] = 1.0;

            debug!("rows = {:?}", rows);
            debug!("b = {:?}", b);

            // calculate alphas
            routing_cfg.alphas = if let Some(x) = algebra::Matrix::from_rows(rows).lu().solve(&b) {
                x
            } else {
                continue;
            };
            debug!("alphas = {:?}", routing_cfg.alphas);
            for i in 0..dim {
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
                // Given is the cost-space and the goal is finding a convex-hull of all paths in cost-space.
                // Given that the facet-paths (at the beginning, the initial dirac-paths) are part of the convex-hull, a new path has to be part of the convex-hull as well, or Dijkstra would have found one of the already found paths in the first place.
                //
                // Proof:
                //
                // Note that imaginating a 2D-cost-space could help.
                //
                // Dijkstra determines the cost-value by calculating the scalar-product of the alpha-vector with a path's cost-vector.
                // Further, the scalar-product is the orthogonal projection of a vector v onto the other vector w, times the length of w.
                // When comparing two scalar-products, where w is equal, it is just a comparison of of the projections themselves.
                //
                // Let the alpha-vector be w from above description.
                // Since the alpha-vector is only relevant in its direction, assume it to be infinitely long for better understanding.
                // The initially found paths are part of the convex-hull, because they have been found with dirac-alphas and hence have the shortest orthogonal projection on the alpha-vector.
                // Further, new paths' cost-vectors have to be between facet-paths in the cost-space, or mathematically speaking, be a linear combination of the facet-paths with positive coefficients, if the facet-paths are part of the convex-hull as well and there are no negative weights in the graph.
                //
                // Looking at a facet of the current convex-hull, every found path is the best one for a specific subset of all alpha-vectors.
                // A new path, found by Dijkstra, can't be worse than the already found ones.
                // Being a better path implies having a lower scalar-product with the alpha-vector and the new path's cost-vector.
                // The scalar-product can only be better, if the projection of the new cost-vector onto the respective alpha-vector is shorter than the projection of the facet-paths.
                // Since the new path has to be between the facet-paths, as the alpha-vector does, this can only be achieved by being closer to the origin, leading to a new convex-hull.

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

        // TODO no paths found -> error?
        Ok(found_paths)
    }
}
