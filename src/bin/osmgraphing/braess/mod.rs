//------------------------------------------------------------------------------------------------//
// other modules

use log::{info, warn};
use osmgraphing::{routing, Parser};

//------------------------------------------------------------------------------------------------//
// own modules

pub mod routes;

//------------------------------------------------------------------------------------------------//
// config

pub struct Config<'a> {
    pub mapfile: &'a str,
}

//------------------------------------------------------------------------------------------------//
// simulation

pub fn run(cfg: Config) -> Result<(), String> {
    info!("Executing braess-optimization");

    //--------------------------------------------------------------------------------------------//
    // parsing

    let graph = Parser::parse_and_finalize(&cfg.mapfile)?;

    //--------------------------------------------------------------------------------------------//
    // read in src-dst-pairs

    let proto_routes = vec![(0, 1000)];

    //--------------------------------------------------------------------------------------------//
    // routing

    let mut astar = routing::factory::new_fastest_path_astar();

    // routes
    for (src_idx, dst_idx) in proto_routes {
        let src = graph.node(src_idx);
        let dst = graph.node(dst_idx);

        let option_path = astar.compute_best_path(src.id(), dst.id(), &graph);
        if let Some(path) = option_path {
            info!("Distance {} m from ({}) to ({}).", path.cost(), src, dst);
        } else {
            warn!("No path from ({}) to ({}).", src, dst);
        }
    }

    Ok(())
}
