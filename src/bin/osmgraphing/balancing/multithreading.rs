use log::{debug, info, trace, warn};
use osmgraphing::{
    configs::{
        self,
        routing::{ExploratorAlgo, RoutingAlgo},
    },
    defaults,
    helpers::err,
    network::{Graph, RoutePair},
    routing::{
        dijkstra::{self, Dijkstra},
        explorating::ConvexHullExplorator,
        paths::Path,
    },
};
use progressing::{mapping::Bar as MappingBar, Baring};
use rand::{
    distributions::{Distribution, Uniform},
    Rng, SeedableRng,
};
use std::{
    ops::Deref,
    sync::{mpsc, Arc},
    thread,
};

pub struct Master {
    outcome_rx: mpsc::Receiver<(WorkerIdx, Outcome)>,
    // route-pairs and random numbers
    worker_sockets: Vec<Option<WorkerSocket>>,
    work_size: usize,
    work_size_plus: usize,
    work_size_minus: usize,
    last_worker_idx: Option<WorkerIdx>,
}

impl Master {
    pub fn work_off(
        &mut self,
        mut route_pairs: Vec<(RoutePair<i64>, usize)>,
        arc_ch_graph: &Arc<Graph>,
        rng: &mut rand_pcg::Lcg64Xsh32,
        is_collecting_paths: bool,
    ) -> err::Result<(Vec<usize>, Option<Vec<Path>>)> {
        info!("Using {} threads working off", self.num_threads());

        route_pairs.reverse();
        // not routes, because progress can be shown without it (though it is less accurate)
        let num_of_route_pairs = route_pairs.len();

        let mut abs_workloads: Vec<usize> = vec![0; arc_ch_graph.fwd_edges().count()];
        let mut chosen_paths = if is_collecting_paths {
            // num_of_route_pairs is not accurate, but lower bound
            Some(Vec::with_capacity(num_of_route_pairs))
        } else {
            None
        };
        let mut avg_num_of_found_paths = 0.0;
        let mut var_num_of_found_paths = 0.0;

        let mut progress_bar = MappingBar::with_range(0, num_of_route_pairs).timed();

        info!("START Executing routes and analyzing workload",);
        loop {
            if let Ok(outcome) = self.recv() {
                // update counts from outcome

                for path in outcome
                    .chosen_paths
                    .into_iter()
                    .map(|path| path.flatten(&arc_ch_graph))
                {
                    for &edge_idx in &path {
                        abs_workloads[*edge_idx] += 1;
                    }

                    if let Some(chosen_paths) = chosen_paths.as_mut() {
                        chosen_paths.push(path);
                    }
                }
                // num_of_routes is ignored here
                progress_bar.add(outcome.num_of_route_pairs);

                // mean and variance
                if outcome.num_of_found_paths.len() > 0 {
                    let n = progress_bar.progress() as f64;
                    let k = outcome.num_of_found_paths.len() as f64;

                    // mean
                    // outcome
                    //     .num_of_found_paths
                    //     .iter()
                    //     .for_each(|k| avg_num_of_found_paths += k);
                    let old_mean = avg_num_of_found_paths;
                    let new_mean = old_mean
                        + (outcome
                            .num_of_found_paths
                            .iter()
                            .map(|x| *x as f64)
                            .sum::<f64>()
                            - k * old_mean)
                            / n;

                    // variance
                    let old_var = var_num_of_found_paths;
                    let new_var = old_var * ((n - k) / (n))
                        + outcome
                            .num_of_found_paths
                            .iter()
                            .map(|x| *x as f64)
                            .map(|x| (x - old_mean) * (x - new_mean))
                            .sum::<f64>()
                            / n;

                    // update
                    avg_num_of_found_paths = new_mean;
                    var_num_of_found_paths = new_var;
                }

                // print and update progress
                if progress_bar.has_progressed_significantly() {
                    progress_bar.remember_significant_progress();
                    info!("{}", progress_bar);
                    debug!(
                        "{}{:.1}{}{:.1}{}",
                        "On average over all route-pairs so far, ",
                        // (1 + 2 * avg_num_of_found_paths / progress_bar.progress()) / 2,
                        avg_num_of_found_paths,
                        "+-",
                        var_num_of_found_paths.sqrt(),
                        " path(s) per routing-query were found.",
                    );
                    debug!("Current work-size: {}", self.work_size);
                }

                // send new work

                if route_pairs.len() > 0 {
                    let chunk_len = std::cmp::min(route_pairs.len(), self.work_size());
                    let chunk: Vec<_> = route_pairs
                        .splice((route_pairs.len() - chunk_len).., vec![])
                        .rev()
                        .collect();
                    self.send(Work {
                        route_pairs: chunk,
                        seed: rng.gen(),
                    })?;
                } else {
                    self.drop_and_join_worker()?;
                }
            } else {
                // disconnected when all workers are dropped
                break;
            }
        }

        info!(
            "{}{:.1}{}{:.1}{}",
            "On average, ",
            // (1 + 2 * avg_num_of_found_paths / num_of_route_pairs) / 2,
            avg_num_of_found_paths,
            "+-",
            var_num_of_found_paths.sqrt(),
            " path(s) per exploration were found.",
        );

        Ok((abs_workloads, chosen_paths))
    }

    fn work_size(&self) -> usize {
        // give one worker just 1 work-package, e.g. for monitoring

        // If only one worker exists
        // -> worker-idx is always 0 < 1
        // -> This condition guarantees, that
        //    (1) only one worker gets work of size 1
        //    (2) and only if this worker is not the only worker.
        //    (3) and it is not worker 0, which is used to calculate next work-size
        if self.last_worker_idx == Some(WorkerIdx(1)) {
            1
        } else {
            self.work_size
        }
    }

    fn num_threads(&self) -> usize {
        self.worker_sockets.len()
    }

    fn last_worker_idx(&self) -> WorkerIdx {
        *self
            .last_worker_idx
            .as_ref()
            .expect("Before sending work, an empty outcome has to be received initially.")
    }

    pub fn spawn_some(
        count: usize,
        arc_graph: &Arc<Graph>,
        arc_routing_cfg: &Arc<configs::routing::Config>,
    ) -> err::Result<Master> {
        info!("Using routing-algo: {:?}", arc_routing_cfg.routing_algo);
        let mut worker_sockets = Vec::with_capacity(count);

        let (outcome_tx, outcome_rx) = mpsc::channel();
        // mpsc::Sender::clone(outcome_tx)
        let mut outcome_txs = vec![outcome_tx; count];
        for idx in (0..count).map(WorkerIdx) {
            // create worker

            let (work_tx, work_rx) = mpsc::channel();
            let worker = Worker::new(WorkerContext {
                idx,
                arc_graph: Arc::clone(arc_graph),
                arc_routing_cfg: Arc::clone(arc_routing_cfg),
                work_rx,
                outcome_tx: outcome_txs
                    .pop()
                    .expect("There should be enough outcome_txs."),
            });

            // spawn worker and store socket

            let handle = worker.spawn()?;
            worker_sockets.push(Some(WorkerSocket { work_tx, handle }))
        }

        // ensure no underflow
        let init_work_size = {
            if defaults::balancing::INIT_WORK_SIZE < defaults::balancing::WORK_SIZE_MINUS {
                warn!(
                    "{}{}{}{}{}",
                    "INIT_WORK_SIZE=",
                    defaults::balancing::INIT_WORK_SIZE,
                    " but should be at least WORK_SIZE_MINUS=",
                    defaults::balancing::WORK_SIZE_MINUS,
                    " to prevent underflows."
                );
                defaults::balancing::WORK_SIZE_MINUS
            } else {
                defaults::balancing::INIT_WORK_SIZE
            }
        };
        // ensure no overflow
        let init_work_size = {
            if std::usize::MAX - defaults::balancing::WORK_SIZE_PLUS < init_work_size {
                warn!(
                    "{}{}{}{}{}",
                    "INIT_WORK_SIZE=",
                    defaults::balancing::INIT_WORK_SIZE,
                    " but should be lower than (std::usize::MAX - WORK_SIZE_PLUS)=",
                    std::usize::MAX - defaults::balancing::WORK_SIZE_PLUS,
                    " to prevent overflows."
                );
                std::usize::MAX - defaults::balancing::WORK_SIZE_PLUS
            } else {
                init_work_size
            }
        };
        Ok(Master {
            outcome_rx,
            worker_sockets,
            work_size: init_work_size,
            work_size_plus: defaults::balancing::WORK_SIZE_PLUS,
            work_size_minus: defaults::balancing::WORK_SIZE_MINUS,
            last_worker_idx: None,
        })
    }

    pub fn send(&self, work: Work) -> err::Feedback {
        let worker_idx = self.last_worker_idx();
        let worker_socket = self.worker_sockets[*worker_idx].as_ref().expect(&format!(
            "Worker {}'s sender should not be released yet.",
            *worker_idx
        ));

        worker_socket
            .work_tx
            .send(work)
            .map_err(|e| err::Msg::from(format!("Sending work stucks due to {}", e)))
    }

    pub fn recv(&mut self) -> err::Result<Outcome> {
        let (worker_idx, outcome) = {
            if let Ok((worker_idx, outcome)) = self.outcome_rx.try_recv() {
                if *worker_idx == 0 {
                    // the worker finished (much?) earlier and waited
                    // -> give workers more work
                    self.work_size = std::cmp::min(
                        std::usize::MAX - self.work_size_plus,
                        self.work_size + self.work_size_plus,
                    );
                }

                (worker_idx, outcome)
            } else {
                let (worker_idx, outcome) = self.outcome_rx.recv().map_err(|e| {
                    err::Msg::from(format!("Receiving outcome stucks due to {}", e))
                })?;

                if *worker_idx == 0 {
                    // the worker didn't finish yet and master waits
                    // -> give workers less work
                    self.work_size =
                        std::cmp::max(self.work_size_minus, self.work_size - self.work_size_minus);
                }

                (worker_idx, outcome)
            }
        };

        self.last_worker_idx = Some(worker_idx);
        Ok(outcome)
    }

    pub fn drop_and_join_worker(&mut self) -> err::Feedback {
        let worker_idx = self.last_worker_idx();
        self.worker_sockets[*worker_idx]
            .take()
            .expect(&format!(
                "Worker {}'s socket shouldn't be dropped yet.",
                *worker_idx
            ))
            .drop_and_join()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorkerIdx(usize);

impl Deref for WorkerIdx {
    type Target = usize;

    fn deref(&self) -> &usize {
        &self.0
    }
}

struct WorkerSocket {
    work_tx: mpsc::Sender<Work>,
    handle: thread::JoinHandle<err::Feedback>,
}

impl WorkerSocket {
    fn drop_and_join(self) -> err::Feedback {
        drop(self.work_tx);
        self.handle
            .join()
            .map_err(|e| err::Msg::from(format!("Joining stucks due to {:?}", e)))?
    }
}

pub struct Work {
    pub route_pairs: Vec<(RoutePair<i64>, usize)>,
    pub seed: u64,
}

/// Chosen paths are not necessarily the same as found paths (e.g. when using explorator), for which reason the `num_of_found_paths` is provided separatedly.
pub struct Outcome {
    pub chosen_paths: Vec<Path>,
    pub num_of_found_paths: Vec<usize>,
    pub num_of_route_pairs: usize,
}

struct WorkerContext {
    idx: WorkerIdx,
    arc_graph: Arc<Graph>,
    arc_routing_cfg: Arc<configs::routing::Config>,
    work_rx: mpsc::Receiver<Work>,
    outcome_tx: mpsc::Sender<(WorkerIdx, Outcome)>,
}

struct Worker {
    dijkstra: Dijkstra,
    explorator: ConvexHullExplorator,
    // context
    idx: WorkerIdx,
    arc_graph: Arc<Graph>,
    arc_routing_cfg: Arc<configs::routing::Config>,
    work_rx: mpsc::Receiver<Work>,
    outcome_tx: mpsc::Sender<(WorkerIdx, Outcome)>,
}

impl Worker {
    fn new(context: WorkerContext) -> Worker {
        Worker {
            dijkstra: Dijkstra::new(),
            explorator: ConvexHullExplorator::new(),
            idx: context.idx,
            arc_graph: context.arc_graph,
            arc_routing_cfg: context.arc_routing_cfg,
            work_rx: context.work_rx,
            outcome_tx: context.outcome_tx,
        }
    }

    fn spawn(mut self) -> err::Result<thread::JoinHandle<err::Feedback>> {
        // start thread such that it is already sending
        // -> easier for main-thread
        self.outcome_tx
            .send((
                self.idx,
                Outcome {
                    chosen_paths: Vec::new(),
                    num_of_found_paths: Vec::new(),
                    num_of_route_pairs: 0,
                },
            ))
            .map_err(|e| format!("Sending initial outcome stucks due to {}", e))?;

        let handle = thread::spawn(move || {
            loop {
                // receive work until connection breaks
                let work = match self.work_rx.recv() {
                    Ok(work) => work,
                    Err(_) => {
                        // disconnected
                        break;
                    }
                };

                // do work
                let outcome = match self.arc_routing_cfg.routing_algo {
                    super::RoutingAlgo::Dijkstra => self.work_off_with_dijkstra(work),
                    super::RoutingAlgo::CHDijkstra => self.work_off_with_dijkstra(work),
                    super::RoutingAlgo::Explorator { algo } => {
                        self.work_off_with_explorator(work, algo)
                    }
                };

                // return outcome
                self.outcome_tx
                    .send((self.idx, outcome))
                    .expect("Sending outcome should always work.")
            }
            Ok(())
        });

        Ok(handle)
    }

    fn work_off_with_dijkstra(&mut self, work: Work) -> Outcome {
        let mut chosen_paths = Vec::new();
        let mut num_of_found_paths = Vec::new();
        let num_of_route_pairs = work.route_pairs.len();

        for (route_pair, route_count) in work.route_pairs {
            let RoutePair { src, dst } = route_pair.into_node(&self.arc_graph);

            // find explorated routes

            let best_path = self.dijkstra.compute_best_path(dijkstra::Query {
                src_idx: src.idx(),
                dst_idx: dst.idx(),
                graph: &self.arc_graph,
                routing_cfg: &self.arc_routing_cfg,
            });

            // Update next workload by looping over all found routes
            // -> Routes have to be flattened,
            // -> or future shortcuts using the resulting workload
            //    will lead to wrong best-paths, because counts won't be cumulated.

            if let Some(best_path) = best_path {
                num_of_found_paths.push(1);

                for _ in 0..(route_count - 1) {
                    chosen_paths.push(best_path.clone());
                }
                chosen_paths.push(best_path);
            } else {
                warn!("Didn't find any path when executing Dijkstra.")
            }
        }

        chosen_paths.shrink_to_fit();
        num_of_found_paths.shrink_to_fit();

        Outcome {
            chosen_paths,
            num_of_found_paths,
            num_of_route_pairs,
        }
    }

    fn work_off_with_explorator(&mut self, work: Work, explorator_algo: ExploratorAlgo) -> Outcome {
        let mut chosen_paths = Vec::new();
        let mut num_of_found_paths = Vec::new();
        let num_of_route_pairs = work.route_pairs.len();
        let mut rng = rand_pcg::Pcg32::seed_from_u64(work.seed);

        let mut routing_cfg = self.arc_routing_cfg.as_ref().clone();
        routing_cfg.routing_algo = RoutingAlgo::from(explorator_algo);

        for (route_pair, route_count) in work.route_pairs {
            let RoutePair { src, dst } = route_pair.into_node(&self.arc_graph);

            // find explorated routes

            let found_paths = self.explorator.fully_explorate(
                dijkstra::Query {
                    src_idx: src.idx(),
                    dst_idx: dst.idx(),
                    graph: &self.arc_graph,
                    routing_cfg: &routing_cfg,
                },
                &mut self.dijkstra,
            );

            num_of_found_paths.push(found_paths.len());

            // Update next workload by looping over all found routes
            // -> Routes have to be flattened,
            // -> or shortcuts will lead to wrong best-paths, because counts won't be cumulated.

            if found_paths.len() > 0 {
                let die = Uniform::from(0..found_paths.len());
                for _ in 0..route_count {
                    let chosen_path = found_paths[die.sample(&mut rng)].clone();
                    trace!("    {}", chosen_path);
                    chosen_paths.push(chosen_path);
                }
            } else {
                warn!("Didn't find any path when explorating.")
            }
        }

        chosen_paths.shrink_to_fit();
        num_of_found_paths.shrink_to_fit();

        Outcome {
            chosen_paths,
            num_of_found_paths,
            num_of_route_pairs,
        }
    }
}
