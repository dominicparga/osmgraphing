// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use crate::{
    configs::routing::Config,
    network::{Graph, NodeIdx},
    routing::Dijkstra,
};
use log::debug;
use nd_triangulation::Triangulation;
use smallvec::smallvec;

pub struct ConvexHullExplorator {}

impl ConvexHullExplorator {
    pub fn new() -> ConvexHullExplorator {
        ConvexHullExplorator {}
    }

    fn compute_and_add_path(
        triangulation: &mut Triangulation,
        dijkstra: &mut Dijkstra,
        src_idx: NodeIdx,
        dst_idx: NodeIdx,
        graph: &Graph,
        cfg_routing: &Config,
    ) -> Result<(), String> {
        if let Some(best_path) = dijkstra.compute_best_path(src_idx, dst_idx, graph, cfg_routing) {
            let best_path = best_path.flatten(graph);
            match triangulation.add_vertex(&best_path.costs()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{}", e)),
            }
        } else {
            Err(format!(
                "No path exists from {} to {}",
                graph.nodes().create(src_idx),
                graph.nodes().create(dst_idx)
            ))
        }
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
        cfg_routing: &Config,
    ) -> Result<(), String> {
        //----------------------------------------------------------------------------------------//
        // init query

        let mut cfg_routing = cfg_routing.clone();
        let dim = graph.metrics().dim();

        //----------------------------------------------------------------------------------------//
        // find initial convex-hull
        // TODO what if not enough different paths have been found?
        // -> return already found paths

        let mut triangulation = Triangulation::new(dim);

        // prepare point d+1

        // if path exists -> add it to convex hull
        cfg_routing.alphas = smallvec![1.0 / (dim as f64); dim];
        Self::compute_and_add_path(
            &mut triangulation,
            dijkstra,
            src_idx,
            dst_idx,
            graph,
            &cfg_routing,
        )?;

        // prepare dirac-paths
        // alphas = [0, ..., 0, 1, 0, ..., 0] (f64)

        // path from src to dst does exist
        for i in 0..dim {
            cfg_routing.alphas = smallvec![0.0; dim];
            cfg_routing.alphas[i] = 1.0;
            Self::compute_and_add_path(
                &mut triangulation,
                dijkstra,
                src_idx,
                dst_idx,
                graph,
                &cfg_routing,
            )?;
        }

        //----------------------------------------------------------------------------------------//
        // print convex-hull for debugging

        // info!("metrics: {:?}", graph.cfg().edges.metric_categories());
        for cell in triangulation.convex_hull_cells() {
            debug!("New cell");
            for vertex in cell.vertices() {
                debug!("{:?}", vertex.coords());
            }
        }

        //----------------------------------------------------------------------------------------//
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

        Ok(())
    }
}
