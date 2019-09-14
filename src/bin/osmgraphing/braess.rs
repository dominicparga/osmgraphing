use std::ffi::OsString;
use std::time::Instant;

use log::{error, info};
use osmgraphing::{routing, Parser};

pub fn run() {
    info!("Executing braess-optimization");

    //----------------------------------------------------------------------------------------------
    // parsing

    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/maps/simple_stuttgart.fmi"),
    };

    let now = Instant::now();
    let graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
    info!(
        "Finished parsing in {} seconds ({} µs).",
        now.elapsed().as_secs(),
        now.elapsed().as_micros(),
    );
    info!("");
    info!("{}", graph);

    //----------------------------------------------------------------------------------------------
    // astar

    let mut astar = routing::factory::new_shortest_path_astar();

    let src_idx = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();
    // let dsts: Vec<usize> = vec![80]; problem on baden-wuerttemberg.osm.pbf

    let src = graph.node(src_idx);

    for dst_idx in dsts {
        let dst = graph.node(dst_idx);

        info!("");

        let now = Instant::now();
        let option_path = astar.compute_shortest_path(src.id(), dst.id(), &graph);
        info!(
            "Ran A* in {} µs a.k.a {} seconds",
            now.elapsed().as_micros(),
            now.elapsed().as_secs()
        );
        if let Some(path) = option_path {
            info!("Distance {} m from ({}) to ({}).", path.cost(), src, dst);
        } else {
            info!("No path from ({}) to ({}).", src, dst);
        }
    }
}
