// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use crate::{
    configs,
    helpers::{self, Approx},
    network::{Graph, NodeIdx},
    routing::{paths::Path, Dijkstra},
};
use nd_triangulation::Triangulation;
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
        let mut found_paths_count = 0;
        let mut triangulation = Triangulation::new(dim);

        // find initial convex-hull
        // TODO what if not enough different paths have been found?
        // -> return already found paths
        //
        // prepare dirac-paths
        // alphas = [0, ..., 0, 1, 0, ..., 0] (f64)
        //
        // prepare avg paths

        for i in 0..(dim + 1) {
            // dirac vs average
            if i < dim {
                routing_cfg.alphas = smallvec![0.0; dim];
                routing_cfg.alphas[i] = 1.0;
            } else {
                routing_cfg.alphas = smallvec![1.0 / (dim as f64); dim];
            }

            // and if path exists
            // -> add its cost-vector to triangulation (<-> convex hull)
            // -> remember it for return

            if let Some(best_path) =
                dijkstra.compute_best_path(src_idx, dst_idx, graph, &routing_cfg)
            {
                found_paths.push(best_path.flatten(graph));
                log::debug!("pushed {}", found_paths.last().unwrap());
            } else {
                log::debug!("unpushed");
            }
        }

        // algo

        while found_paths_count < found_paths.len() && found_paths.len() < 10 {
            log::debug!(
                "loop with {} new found paths ({} total) and {} cells",
                found_paths.len() - found_paths_count,
                found_paths.len(),
                triangulation.convex_hull_cells().count()
            );

            // add new paths

            for i in found_paths_count..found_paths.len() {
                let p = &found_paths[i];
                match triangulation.add_vertex(p.costs()) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{}", e)),
                }
            }
            found_paths_count = found_paths.len();

            log::debug!(
                "after adding new paths to triangulation: {} cells",
                triangulation.convex_hull_cells().count()
            );
            for cell in triangulation.convex_hull_cells() {
                log::debug!("START cell");
                for vertex in cell.vertices() {
                    log::debug!("vertex-coords {:?}", vertex.coords());
                }
                log::debug!("FINISH cell");
            }

            // check every facet, if it is already sharp enough

            for cell in triangulation.convex_hull_cells() {
                if let Some(vertex) = cell.vertices().next() {
                    // Solve LGS to get alpha, where all cell-vertex-costs (personalized with alpha)
                    // are equal.

                    // TODO LGS
                    routing_cfg.alphas = routing_cfg.alphas;

                    // find best path

                    if let Some(best_path) =
                        dijkstra.compute_best_path(src_idx, dst_idx, graph, &routing_cfg)
                    {
                        let new_p = best_path.flatten(graph);

                        if new_p
                            .costs()
                            .iter()
                            .zip(vertex.coords())
                            .fold(true, |acc, (a, b)| acc && a == b)
                        {
                            if found_paths.contains(&new_p) {
                                continue;
                            }
                        }

                        // All already found paths have same cost with the used alpha.
                        // Hence, if the new costs are better than any of these paths,
                        // under this particular alpha, the new found path is part
                        // of the convex-hull and has to be added.
                        if helpers::dot_product(&routing_cfg.alphas, new_p.costs()).approx()
                            <= helpers::dot_product(&routing_cfg.alphas, vertex.coords()).approx()
                        {
                            // remember path
                            found_paths.push(new_p);
                            log::debug!("pushed {}", found_paths.last().unwrap());
                        } else {
                            log::debug!("unpushed because not good");
                        }
                    } else {
                        log::debug!("unpushed");
                    }
                }
            }
        }

        Ok(found_paths)
    }
}
