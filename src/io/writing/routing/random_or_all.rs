use crate::{
    configs,
    helpers::err,
    network::{Graph, NodeIdx},
    routing,
};
use log::{info, warn};
use progressing::{bernoulli::Bar as BernoulliBar, Baring};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{
    cmp::min,
    collections::HashSet,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Writer {
    seed: u64,
    max_count: usize,
}

impl Writer {
    pub fn new(seed: u64, max_count: usize) -> Writer {
        Writer { seed, max_count }
    }
}

impl Writer {
    pub fn write(
        &self,
        graph: &Graph,
        routing_cfg: &configs::routing::Config,
        writing_cfg: &configs::writing::routing::Config,
    ) -> err::Feedback {
        // prepare

        let output_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&writing_cfg.file)?;
        let mut writer = BufWriter::new(output_file);

        let nodes = graph.nodes();
        let fwd_edges = graph.fwd_edges();

        // create routes

        let num_possible_routes = nodes.count() * nodes.count();
        let max_count = min(num_possible_routes, self.max_count);
        let mut found_route_pairs = Vec::with_capacity(max_count);

        let found_route_pairs = {
            let mut processed_indices = HashSet::new();

            let mut rng = rand_pcg::Pcg32::seed_from_u64(self.seed);
            let die = Uniform::from(0..nodes.count());

            let mut dijkstra = routing::Dijkstra::new();

            if num_possible_routes <= self.max_count {
                warn!(
                    "There are only {} nodes in the graph, resulting in {} possible routes, \
                    and {} are requested.",
                    nodes.count(),
                    num_possible_routes,
                    self.max_count
                );
            }

            let mut progress_bar = BernoulliBar::with_goal(max_count).timed();
            info!("{}", progress_bar);

            // Stop when enough existing routes have been found
            // or when all possible routes are processed.
            while progress_bar.progress().successes < max_count
                && progress_bar.progress().attempts < num_possible_routes
            {
                if progress_bar.has_progressed_significantly() {
                    progress_bar.remember_significant_progress();
                    info!("{}", progress_bar);
                }

                let (src_idx, dst_idx) = {
                    // if all possible routes are less than the preferred route-count
                    // -> just print all possible routes
                    // else: print random routes
                    if num_possible_routes <= self.max_count {
                        let i = progress_bar.progress().attempts;
                        let src_idx = NodeIdx(i / nodes.count());
                        let dst_idx = NodeIdx(i % nodes.count());
                        (src_idx, dst_idx)
                    } else {
                        let src_idx = NodeIdx(die.sample(&mut rng));
                        let dst_idx = NodeIdx(die.sample(&mut rng));
                        (src_idx, dst_idx)
                    }
                };

                let is_already_processed = !processed_indices.insert((src_idx, dst_idx));
                if !is_already_processed
                    && dijkstra
                        .compute_best_path(routing::Query {
                            src_idx,
                            dst_idx,
                            graph: &graph,
                            routing_cfg: &routing_cfg,
                        })
                        .is_some()
                {
                    found_route_pairs
                        .push((nodes.create(src_idx).id(), nodes.create(dst_idx).id()));
                    progress_bar.add(true);
                } else {
                    progress_bar.add(false);
                }
            }

            found_route_pairs.sort();

            found_route_pairs
        };

        // write header

        writeln!(writer, "# graph-file: {}", graph.cfg().map_file.display())?;
        writeln!(writer, "# node-count: {}", nodes.count(),)?;
        writeln!(writer, "# edge-count: {}", fwd_edges.count(),)?;
        writeln!(writer, "")?;

        // write route-count

        writeln!(writer, "# route-count")?;
        writeln!(writer, "{}", found_route_pairs.len())?;
        writeln!(writer, "")?;

        // write routes

        writeln!(
            writer,
            "# random routes: (src-id dst-id count) as (i64, i64, usize)"
        )?;
        writeln!(writer, "# seed: {}", self.seed)?;
        for (src_id, dst_id) in found_route_pairs {
            writeln!(writer, "{} {} {}", src_id, dst_id, 1)?;
        }

        Ok(())
    }
}
