//------------------------------------------------------------------------------------------------//
// other modules

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;

use osmgraphing::network::Graph;
use osmgraphing::routing;

//------------------------------------------------------------------------------------------------//
// own modules

use super::model::SmallEdgeInfo;

//------------------------------------------------------------------------------------------------//

pub struct WorkerSocket {
    data_tx: Sender<Vec<(i64, i64)>>,
    handle: thread::JoinHandle<Result<(), String>>,
}
impl<'a> WorkerSocket {
    pub fn spawn_some(count: u8, graph: &Arc<Graph>) -> (Vec<Self>, Receiver<usize>) {
        let mut workers = vec![];
        let (stats_tx, stats_rx) = mpsc::channel();

        for _ in 0..(count - 1) {
            let tx = mpsc::Sender::clone(&stats_tx);
            workers.push(Self::spawn(tx, Arc::clone(graph)));
        }
        workers.push(Self::spawn(stats_tx, Arc::clone(graph)));

        (workers, stats_rx)
    }

    pub fn spawn(stats_tx: Sender<usize>, graph: Arc<Graph>) -> Self {
        let (data_tx, data_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut astar = routing::factory::new_fastest_path_astar();
            let mut stats: Vec<Option<SmallEdgeInfo>> = vec![None; graph.edge_count()];

            loop {
                let proto_routes = match data_rx.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };

                super::work_off(proto_routes, &mut astar, &mut stats, &graph)?;

                stats_tx.send(3).unwrap();
            }

            Ok(())
        });

        WorkerSocket {
            data_tx: data_tx,
            handle: handle,
        }
    }
}
