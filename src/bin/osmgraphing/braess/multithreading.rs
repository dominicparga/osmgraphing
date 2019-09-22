//------------------------------------------------------------------------------------------------//
// other modules

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;

use log::{debug, trace};

use osmgraphing::network::Graph;
use osmgraphing::routing;

//------------------------------------------------------------------------------------------------//
// own modules

use super::model::{Packet, SmallEdgeInfo};

//------------------------------------------------------------------------------------------------//

pub struct WorkerSocket {
    data_tx: Sender<Vec<(i64, i64)>>,
    handle: thread::JoinHandle<Result<(), String>>,
}
impl<'a> WorkerSocket {
    pub fn data_tx(&self) -> &Sender<Vec<(i64, i64)>> {
        &self.data_tx
    }

    pub fn drop_and_join(self) -> Result<(), String> {
        drop(self.data_tx);
        match self.handle.join() {
            Ok(_) => Ok(()),
            Err(e) => return Err(format!("Could not join due to {:?}", e)),
        }
    }
}
impl<'a> WorkerSocket {
    pub fn spawn_some(
        count: u8,
        graph: &Arc<Graph>,
    ) -> Result<(Vec<Option<Self>>, Receiver<Packet>), String> {
        let mut workers = vec![];
        let (stats_tx, stats_rx) = mpsc::channel();

        for worker_idx in 0..(count - 1) {
            let tx = mpsc::Sender::clone(&stats_tx);
            workers.push(Some(Self::spawn(worker_idx, tx, Arc::clone(graph))?));
        }
        workers.push(Some(Self::spawn(count - 1, stats_tx, Arc::clone(graph))?));

        Ok((workers, stats_rx))
    }

    pub fn spawn(
        worker_idx: u8,
        stats_tx: Sender<Packet>,
        graph: Arc<Graph>,
    ) -> Result<Self, String> {
        let (data_tx, data_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            debug!("[Worker {}] spawned", worker_idx);

            let mut astar = routing::factory::new_fastest_path_astar();
            let mut stats: Vec<Option<SmallEdgeInfo>> = vec![None; graph.edge_count()];

            loop {
                trace!(
                    "[Worker {}][step 0] Waiting for proto-routes ..",
                    worker_idx
                );

                // recv new data
                let proto_routes: Vec<(i64, i64)> = match data_rx.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };

                trace!("[Worker {}][step 1] Received proto-routes", worker_idx);
                trace!("[Worker {}][step 2] Starting processing ..", worker_idx);

                // work data completely off
                let (k, n) =
                    super::work_off_all(&proto_routes[..], &mut astar, &mut stats, &graph)?;

                trace!(
                    "[Worker {}][step 3] Finished working-off proto-routes",
                    worker_idx
                );
                trace!("[Worker {}][step 4] Starting sending stats ..", worker_idx);

                // send results
                let packet = Packet {
                    worker_idx,
                    k,
                    n,
                    // drain clears stats
                    // drain + filter_map + collect -> capacity fits
                    stats: stats
                        .drain(..)
                        .filter_map(|s| if s.is_some() { Some(s) } else { None })
                        .collect(),
                };
                stats_tx.send(packet).unwrap();
                trace!("[Worker {}][step 5] stats sent", worker_idx);

                // refill cleared stats
                stats.resize(graph.edge_count(), None);
            }

            debug!("[Worker {}] terminated", worker_idx);
            Ok(())
        });

        // start thread such that it is already sending
        // -> terminating is easier for main-thread
        if let Err(e) = data_tx.send(vec![]) {
            Err(format!("Sending stucks due to {}", e))
        } else {
            Ok(WorkerSocket {
                data_tx: data_tx,
                handle: handle,
            })
        }
    }
}
