use crate::{
    configs::{self, writing},
    helpers,
    network::{Graph, NodeIdx},
    routing,
};
use log::{info, warn};
use progressing::{self, Bar};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{
    cmp::min,
    collections::HashSet,
    io::{BufWriter, Write},
};

pub struct Writer {
    seed: u64,
    count: usize,
}

impl Writer {
    pub fn new(seed: u64, count: usize) -> Writer {
        Writer { seed, count }
    }
}

impl super::Writing for Writer {
    fn write(&self, graph: &Graph, writing_cfg: &writing::routing::Config) -> Result<(), String> {
        fn inner_write(
            myself: &Writer,
            graph: &Graph,
            writing_cfg: &writing::routing::Config,
        ) -> Result<(), Box<dyn std::error::Error>> {
            // prepare

            let output_file = helpers::open_new_file(&writing_cfg.file)?;
            let mut writer = BufWriter::new(output_file);

            let nodes = graph.nodes();
            let fwd_edges = graph.fwd_edges();
            let mut dijkstra = routing::Dijkstra::new();
            let routing_cfg = configs::routing::Config::from_all_metrics(graph.cfg());

            let mut rng = rand_pcg::Pcg32::seed_from_u64(myself.seed);
            let die = Uniform::from(0..nodes.count());

            let num_possible_routes = nodes.count() * nodes.count();
            let max_count = min(num_possible_routes, myself.count);
            let mut processed_indices = HashSet::new();
            let mut found_route_pairs = Vec::with_capacity(max_count);

            // create routes

            if num_possible_routes <= myself.count {
                warn!(
                    "There are only {} nodes in the graph, resulting in {} possible routes, \
                     and {} are requested.",
                    nodes.count(),
                    nodes.count() * nodes.count(),
                    myself.count
                );
            }

            let mut progress_bar = progressing::BernoulliBar::from_goal(max_count);

            // Stop when enough existing routes have been found
            // or when all possible routes are processed.
            while progress_bar.progress().successes < max_count
                && progress_bar.progress().attempts < num_possible_routes
            {
                if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                    info!("{}", progress_bar);
                }

                let (src_idx, dst_idx) = {
                    // if all possible routes are less than the preferred route-count
                    // -> just print all possible routes
                    // else: print random routes
                    if num_possible_routes <= myself.count {
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
                        .compute_best_path(src_idx, dst_idx, &graph, &routing_cfg)
                        .is_some()
                {
                    found_route_pairs
                        .push((nodes.create(src_idx).id(), nodes.create(dst_idx).id()));
                    progress_bar.add(true);
                } else {
                    progress_bar.add(false);
                }
            }
            info!("{}", progress_bar);

            // write header

            writeln!(writer, "# graph-file: {}", graph.cfg().map_file.display())?;
            writeln!(writer, "# node-count: {}", nodes.count(),)?;
            writeln!(writer, "# edge-count: {}", fwd_edges.count(),)?;
            writeln!(writer, "")?;

            // write route-count

            let mut found_route_pairs: Vec<_> = found_route_pairs.into_iter().collect();
            found_route_pairs.sort();
            writeln!(writer, "# route-count")?;
            writeln!(writer, "{}", found_route_pairs.len())?;
            writeln!(writer, "")?;

            // write routes

            writeln!(
                writer,
                "# random routes: (src-id dst-id count) as (i64, i64, usize)"
            )?;
            writeln!(writer, "# seed: {}", myself.seed)?;
            for (src_id, dst_id) in found_route_pairs {
                writeln!(writer, "{} {} {}", src_id, dst_id, 1)?;
            }

            Ok(())
        }

        // return result

        if let Err(e) = inner_write(self, graph, writing_cfg) {
            Err(format!("{}", e))
        } else {
            Ok(())
        }
    }
}
