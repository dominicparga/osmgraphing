use log::{debug, warn};
use osmgraphing::{
    configs,
    helpers::err,
    network::{EdgeIdx, Graph, RoutePair},
    routing::{self, ConvexHullExplorator, Dijkstra},
};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{
    ops::Deref,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

pub struct Master {
    outcome_rx: mpsc::Receiver<(WorkerIdx, Outcome)>,
    // route-pairs and random numbers
    worker_sockets: Vec<Option<WorkerSocket>>,
    work_size: usize,
    last_worker_idx: Option<WorkerIdx>,
}

impl Master {
    pub fn work_size(&self) -> usize {
        self.work_size
    }

    fn last_worker_idx(&self) -> WorkerIdx {
        *self
            .last_worker_idx
            .as_ref()
            .expect("Before sending work, an empty outcome has to be received initially.")
    }

    pub fn spawn_some(
        count: usize,
        graph: &Arc<Graph>,
        routing_cfg: &Arc<configs::routing::Config>,
    ) -> err::Result<Master> {
        let mut worker_sockets = Vec::with_capacity(count);

        let (outcome_tx, outcome_rx) = mpsc::channel();
        // mpsc::Sender::clone(outcome_tx)
        let mut outcome_txs = vec![outcome_tx; count];
        for idx in (0..count).map(WorkerIdx) {
            // create worker

            let (work_tx, work_rx) = mpsc::channel();
            let worker = Worker::new(WorkerContext {
                idx,
                graph: Arc::clone(graph),
                routing_cfg: Arc::clone(routing_cfg),
                work_rx,
                outcome_tx: outcome_txs
                    .pop()
                    .expect("There should be enough outcome_txs."),
            });

            // spawn worker and store socket

            let handle = worker.spawn()?;
            worker_sockets.push(Some(WorkerSocket { work_tx, handle }))
        }

        Ok(Master {
            outcome_rx,
            worker_sockets,
            work_size: 1,
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
        self.work_size += 1;

        let (worker_idx, outcome) = self
            .outcome_rx
            .recv()
            .map_err(|e| err::Msg::from(format!("Receiving outcome stucks due to {}", e)))?;
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

#[derive(Clone, Copy, Debug)]
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

pub struct Outcome {
    pub path_edges: Vec<EdgeIdx>,
    pub num_routes: usize,
}

struct WorkerContext {
    idx: WorkerIdx,
    graph: Arc<Graph>,
    routing_cfg: Arc<configs::routing::Config>,
    work_rx: mpsc::Receiver<Work>,
    outcome_tx: mpsc::Sender<(WorkerIdx, Outcome)>,
}

struct Worker {
    dijkstra: Dijkstra,
    explorator: ConvexHullExplorator,
    // context
    idx: WorkerIdx,
    graph: Arc<Graph>,
    routing_cfg: Arc<configs::routing::Config>,
    work_rx: mpsc::Receiver<Work>,
    outcome_tx: mpsc::Sender<(WorkerIdx, Outcome)>,
}

impl Worker {
    fn new(context: WorkerContext) -> Worker {
        Worker {
            dijkstra: Dijkstra::new(),
            explorator: ConvexHullExplorator::new(),
            idx: context.idx,
            graph: context.graph,
            routing_cfg: context.routing_cfg,
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
                    path_edges: Vec::new(),
                    num_routes: 0,
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
                let num_routes = work.route_pairs.len();

                // do work
                let path_edges = self.work_off(work);

                // return outcome
                self.outcome_tx
                    .send((
                        self.idx,
                        Outcome {
                            path_edges,
                            num_routes,
                        },
                    ))
                    .expect("Sending outcome should always work.")
            }
            Ok(())
        });

        Ok(handle)
    }

    fn work_off(&mut self, work: Work) -> Vec<EdgeIdx> {
        let mut path_edges = Vec::new();
        let mut rng = rand_pcg::Pcg32::seed_from_u64(work.seed);

        for (route_pair, count) in work.route_pairs {
            let RoutePair { src, dst } = route_pair.into_node(&self.graph);

            // find explorated routes

            let now = Instant::now();
            let found_paths = self.explorator.fully_explorate(
                routing::Query {
                    src_idx: src.idx(),
                    dst_idx: dst.idx(),
                    graph: &self.graph,
                    routing_cfg: &self.routing_cfg,
                },
                &mut self.dijkstra,
            );
            debug!(
                "Ran Explorator-query from src-id {} to dst-id {} in {} ms. Found {} path(s).",
                src.id(),
                dst.id(),
                now.elapsed().as_micros() as f64 / 1_000.0,
                found_paths.len()
            );

            // Update next workload by looping over all found routes
            // -> Routes have to be flattened,
            // -> or shortcuts will lead to wrong best-paths, because counts won't be cumulated.

            if found_paths.len() > 0 {
                let die = Uniform::from(0..found_paths.len());
                for _ in 0..count {
                    let p = found_paths[die.sample(&mut rng)]
                        .clone()
                        .flatten(&self.graph);

                    debug!("    {}", p);

                    for edge_idx in p {
                        path_edges.push(edge_idx);
                    }
                }
            } else {
                warn!("Didn't find any path when explorating.")
            }
        }

        path_edges.shrink_to_fit();
        path_edges
    }
}
