// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use crate::{
    configs,
    network::{Graph, NodeIdx},
    routing::{paths::Path, Dijkstra},
};
use log::debug;
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

        let mut routing_cfg = routing_cfg.clone();
        let dim = graph.metrics().dim();
        let mut triangulation = Triangulation::new(dim);
        let mut found_paths = Vec::new();

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
                let best_path = best_path.flatten(graph);

                // add path to triangulation (<-> convex hull)
                match triangulation.add_vertex(&best_path.costs()) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{}", e)),
                }

                // remember path
                found_paths.push(best_path);

                // } else {
                //     Err(format!(
                //         "No path exists from {} to {}",
                //         graph.nodes().create(src_idx),
                //         graph.nodes().create(dst_idx)
                //     ))
            }
        }

        // print convex-hull for debugging

        for cell in triangulation.convex_hull_cells() {
            debug!("New cell");
            for vertex in cell.vertices() {
                debug!("{:?}", vertex.coords());
            }
        }

        // algo
        //
        // Goal:
        // c(p1)

        let mut has_new_route = true;

        while has_new_route {
            has_new_route = false;

            // check every facet, if it is already sharp enough
            // loop
        }

        Ok(found_paths)
    }
}
