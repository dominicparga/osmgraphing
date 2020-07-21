// LU-decomposition because of
// https://math.stackexchange.com/questions/1720806/lu-decomposition-vs-qr-decomposition-for-similar-problems
//
// https://crates.io/crates/nalgebra

use crate::{
    configs,
    defaults::{self, capacity::DimVec},
    helpers::{
        self, algebra,
        approx::{Approx, ApproxEq},
    },
    network::{Graph, NodeIdx},
    routing::{
        dijkstra::{self, Dijkstra},
        paths::Path,
    },
};
use log::{debug, trace};
use nd_triangulation::Triangulation;
use smallvec::smallvec;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

// needed because convex-hull has dim+1 points per cell
type CHDimVec<T> = smallvec::SmallVec<[T; defaults::capacity::SMALL_VEC_INLINE_SIZE + 1]>;

struct Query<'a> {
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    graph: &'a Graph,
    routing_cfg: configs::routing::Config,
    graph_dim: usize,
    triangulation_dim: usize,
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
        let graph_dim = graph.metrics().dim();
        // Every cost-value has to be below this value.
        let tolerances: DimVec<_> = smallvec![defaults::routing::TOLERATED_SCALE_INF; graph_dim];
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
            graph_dim,
            triangulation_dim: is_metric_considered
                .iter()
                .filter(|&&is_considered| is_considered)
                .count(),
            tolerances,
            is_metric_considered,
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct VertexId(usize);

impl Deref for VertexId {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

#[derive(Clone)]
struct Vertex<'a> {
    pub id: VertexId,
    pub path: &'a Path,
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CellId(pub usize);

impl Deref for CellId {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

#[derive(Clone)]
struct Cell<'a> {
    id: CellId,
    vertices: CHDimVec<Vertex<'a>>,
}

impl<'a> Cell<'a> {
    pub fn id(&self) -> &CellId {
        &self.id
    }

    pub fn vertices(&self) -> &CHDimVec<Vertex<'a>> {
        &self.vertices
    }
}

pub struct ConvexHullExplorator {
    found_paths: HashMap<VertexId, Path>,
    visited_cells: HashSet<CellId>,
}

impl ConvexHullExplorator {
    pub fn new() -> ConvexHullExplorator {
        ConvexHullExplorator {
            found_paths: HashMap::new(),
            visited_cells: HashSet::new(),
        }
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
        // init query

        let mut query = Query::with(query);

        let mut triangulation = Triangulation::new(query.triangulation_dim);
        let mut is_triangulation_dirty = false;

        self.found_paths.clear();
        self.visited_cells.clear();
        let mut new_found_paths = Vec::new();
        ConvexHullExplorator::explore_initial_paths(&mut new_found_paths, &mut query, dijkstra);
        ConvexHullExplorator::update(
            &query,
            &mut is_triangulation_dirty,
            &mut self.found_paths,
            &mut new_found_paths,
            &mut triangulation,
        );

        // explore

        // +1 because a convex-hull (volume) needs dim+1 points
        // For imagination:
        // - line vs triangle in 2D
        // - triangle vs tetrahedron in 3D
        if query.triangulation_dim > 1
            && self.found_paths.len() + new_found_paths.len() > query.triangulation_dim
        {
            // find new routes

            trace!(
                "Start exploring new alternative routes, because triangulation of dim {} is ready.",
                query.triangulation_dim
            );
            while is_triangulation_dirty {
                trace!("Found {} paths yet.", self.found_paths.len());
                for raw_cell in triangulation.convex_hull_cells() {
                    // don't look at cells twice
                    if self.visited_cells.contains(&CellId(raw_cell.id())) {
                        trace!(
                            "Jump over already explored cell of cell-id {}",
                            raw_cell.id()
                        );
                        continue;
                    }
                    trace!("Explore cell of cell-id {}", raw_cell.id());

                    let cell = ConvexHullExplorator::cell_from(&self.found_paths, raw_cell);
                    self.visited_cells.insert(*cell.id());

                    // Check candidate, whether it's shape is already sharp enough.
                    // This is done by computing the normal-vector for facets of the convex hull,
                    // which is the alpha-vector resulting from the linear system below.
                    // If Dijkstra finds a better path for this alpha-vector,
                    // the path's cost is part of the convex-hull.

                    let (rows, b) = ConvexHullExplorator::create_linear_system(&cell, &query);

                    // calculate alphas
                    query.routing_cfg.alphas =
                        if let Some(x) = algebra::Matrix::from_rows(rows).lu().solve(&b) {
                            x
                        } else {
                            continue;
                        };
                    trace!("alphas = {:?}", query.routing_cfg.alphas);
                    for (i, vertex) in cell.vertices().iter().enumerate() {
                        // for i in 0..candidate.len() {
                        trace!(
                            "alphas * path_{}.costs() = {:?}",
                            i,
                            helpers::dot_product(&query.routing_cfg.alphas, vertex.path.costs(),)
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
                        let new_path = best_path;

                        let new_alpha_cost =
                            helpers::dot_product(&query.routing_cfg.alphas, new_path.costs());
                        trace!("alphas * new_path.costs() = {:?}", new_alpha_cost);
                        // take any vertex, since alpha is chosen s.t. all dot-products are equal
                        let any_alpha_cost = helpers::dot_product(
                            &query.routing_cfg.alphas,
                            cell.vertices()[0].path.costs(),
                        );

                        // Add new path if it's cost-vector's projection onto the alpha-vector
                        // is smaller.

                        // TODO improve <
                        let is_path_new = new_alpha_cost.approx() < any_alpha_cost.approx()
                            && !new_found_paths.contains(&new_path);
                        if is_path_new {
                            debug!("Push {}", new_path);
                            new_found_paths.push(new_path);
                        } else {
                            trace!("Already found path {}", new_path);
                        }
                    }
                }

                ConvexHullExplorator::update(
                    &query,
                    &mut is_triangulation_dirty,
                    &mut self.found_paths,
                    &mut new_found_paths,
                    &mut triangulation,
                );
            }
        }

        self.found_paths
            .values()
            .filter_map(|path| {
                if helpers::le(path.costs(), &query.tolerances) {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn explore_initial_paths(
        new_found_paths: &mut Vec<Path>,
        query: &mut Query,
        dijkstra: &mut Dijkstra,
    ) {
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

        // plus add avg-alpha, because cyclops-convex-hull needs at least dim+1 points
        // (another pro: this can be seen as addition, when not enough paths are found)
        init_alphas.push((None, avg_alphas));

        let mut found_paths = CHDimVec::new();
        for (metric_idx, alphas) in init_alphas {
            debug!("Trying init-alpha {:?}", alphas);

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

                if !found_paths
                    .iter()
                    .map(|path: &Path| path.costs())
                    .any(|costs| costs.approx_eq(best_path.costs()))
                {
                    debug!("Found and pushing init-path {}", best_path);
                    found_paths.push(best_path);
                }
            }
        }

        for path in found_paths {
            new_found_paths.push(path);
        }
    }

    fn cell_from<'a>(
        found_paths: &'a HashMap<VertexId, Path>,
        cell: nd_triangulation::Cell,
    ) -> Cell<'a> {
        Cell {
            id: CellId(cell.id()),
            vertices: cell
                .vertices()
                .into_iter()
                .map(|vertex| VertexId(vertex.id()))
                .map(|vertex_id| Vertex {
                    id: vertex_id,
                    path: found_paths.get(&vertex_id).expect(
                        "For every vertex in the triangulation, a path should be registered.",
                    ),
                })
                .collect(),
        }
    }

    fn create_linear_system(cell: &Cell, query: &Query) -> (DimVec<DimVec<f64>>, DimVec<f64>) {
        trace!("Create linear system with paths:");
        for vertex in cell.vertices() {
            trace!("  {}", vertex.path);
        }

        // Solve LGS to get alpha, where all cell-vertex-costs (personalized with alpha)
        // are equal.
        // -> Determine rows of matrix

        let mut rows = DimVec::new();
        let mut b = DimVec::new();

        // all lines describe the equality of each dot-product between cost-vector and alpha
        let vertex_0 = &cell.vertices()[0];
        for vertex_i in &cell.vertices()[1..] {
            rows.push(helpers::sub(vertex_0.path.costs(), vertex_i.path.costs()));
            b.push(0.0);
        }

        // but ignored metrics should lead to zero alpha
        for (i, _) in query
            .is_metric_considered
            .iter()
            .enumerate()
            .filter(|&(_, imc)| !imc)
        {
            // set [0, ..., 0, 1, 0, ..., 0] to 0.0
            let mut row = smallvec![0.0; query.graph_dim];
            row[i] = 1.0;
            rows.push(row);
            b.push(0.0);
        }

        // if one condition is missing (depending on convex-hull-implementation),
        match query.graph_dim - rows.len() {
            0 => (),
            1 => {
                // you could normalize alpha
                // -> one row in matrix is 1.0

                rows.push(smallvec![1.0; query.graph_dim]);
                b.push(1.0);
            }
            _ => panic!(
                "{}{}",
                "The linear system has less rows than the convex-hull has dimensions.",
                "This doesn't lead to a unique solution."
            ),
        }

        trace!("rows = {:?}", rows);
        trace!("b = {:?}", b);
        (rows, b)
    }

    fn update(
        query: &Query,
        is_triangulation_dirty: &mut bool,
        found_paths: &mut HashMap<VertexId, Path>,
        new_found_paths: &mut Vec<Path>,
        triangulation: &mut Triangulation,
    ) {
        trace!("Updating triangulation");
        *is_triangulation_dirty = new_found_paths.len() > 0;

        // add new paths to triangulation

        for path in new_found_paths.drain(..) {
            let new_raw_id = triangulation
                .add_vertex(
                    &path
                        .costs()
                        .iter()
                        .enumerate()
                        .filter_map(|(i, c)| {
                            if query.is_metric_considered[i] {
                                Some(*c)
                            } else {
                                None
                            }
                        })
                        .collect::<DimVec<_>>(),
                )
                .expect("Path's cost TODO filter by alpha...");
            let new_id = VertexId(new_raw_id);
            found_paths.insert(new_id, path);
        }
        debug_assert!(
            new_found_paths.is_empty(),
            "All new found paths should be added by now."
        );
        trace!(
            "Triangulation is {}dirty.",
            if *is_triangulation_dirty { "" } else { "not " }
        );
    }
}
