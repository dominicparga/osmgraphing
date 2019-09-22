use std::path;

use log::info;
use osmgraphing::{routing, Parser};
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

//------------------------------------------------------------------------------------------------//
// own modules

use super::io_kyle;
use super::progressing;

//------------------------------------------------------------------------------------------------//
// config

pub mod config {
    use std::path;

    pub struct Config<'a, P: AsRef<path::Path> + ?Sized> {
        pub paths: Paths<'a, P>,
        pub route_count: u32,
        pub seed: u64,
    }
    pub struct Paths<'a, P: AsRef<path::Path> + ?Sized> {
        pub input: InputPaths<'a, P>,
        pub output: OutputPaths<'a, P>,
    }

    //--------------------------------------------------------------------------------------------//
    // input-paths

    pub struct InputPaths<'a, P: AsRef<path::Path> + ?Sized> {
        pub files: InputFiles<'a, P>,
    }
    pub struct InputFiles<'a, P: AsRef<path::Path> + ?Sized> {
        pub map: &'a P,
    }

    //--------------------------------------------------------------------------------------------//
    // output-paths

    pub struct OutputPaths<'a, P: AsRef<path::Path> + ?Sized> {
        pub files: OutputFiles<'a, P>,
    }
    pub struct OutputFiles<'a, P: AsRef<path::Path> + ?Sized> {
        pub proto_routes: &'a P,
    }
}
pub use config as cfg;
use config::Config;

//------------------------------------------------------------------------------------------------//

pub fn search_and_export<P: AsRef<path::Path> + ?Sized>(cfg: Config<P>) -> Result<(), String> {
    info!("Executing proto-route-generator");

    //--------------------------------------------------------------------------------------------//
    // some default-params

    let writebuf_len = 100;
    let max_travel_time_ms = 3_600_000; // 1 h

    //--------------------------------------------------------------------------------------------//
    // prepare simulation

    // check path of io-files before expensive simulation
    io_kyle::create_file(cfg.paths.output.files.proto_routes)?;

    let graph = Parser::parse_and_finalize(&cfg.paths.input.files.map)?;
    let mut astar = routing::factory::new_fastest_path_astar();

    // random
    info!("Using seed {}", cfg.seed);
    let mut rng = rand_pcg::Pcg32::seed_from_u64(cfg.seed);
    let die = Uniform::from(0..graph.node_count());

    // proto_routes
    let mut proto_routes = Vec::with_capacity(writebuf_len);

    //--------------------------------------------------------------------------------------------//
    // routing

    // logging progress
    let mut progress_bar = progressing::Bar::from(cfg.route_count);

    // searching
    progress_bar.log();
    io_kyle::write_proto_routes(&vec![], cfg.paths.output.files.proto_routes, false)?;
    while progress_bar.k() < cfg.route_count {
        progress_bar.inc_n();

        let (src, dst) = {
            let src_idx: usize = die.sample(&mut rng);
            let dst_idx: usize = die.sample(&mut rng);
            let src = graph
                .node(src_idx)
                .expect("Range should be ok for src-node.");
            let dst = graph
                .node(dst_idx)
                .expect("Range should be ok for dst-node.");
            (src, dst)
        };
        if let Some(best_path) = astar.compute_best_path(src, dst, &graph) {
            progress_bar.inc_k().log();

            // if travel-time takes at most max_travel_time_ms (e.g. one hour)
            // -> accept
            // if more than one hour
            // -> shorten found path, then accept
            let mut cost = best_path.cost();
            let mut succ_idx = dst.idx();

            // shorten found best-path until it fits
            while cost > max_travel_time_ms {
                // get predecessor of last path-node
                let pred_idx = best_path
                    .pred_node_idx(succ_idx)
                    .expect("Path should be long enough to have a predecessor.");
                // update cost
                cost -= {
                    let (edge, _) = graph
                        .edge_from(pred_idx, succ_idx)
                        .expect("Path-edge should exist.");
                    edge.milliseconds()
                };
                succ_idx = pred_idx;
            }

            // add new path
            let new_dst = graph
                .node(succ_idx)
                .expect("Path should contain graph's nodes.");
            proto_routes.push((src.id(), new_dst.id()));

            // write to file
            if proto_routes.len() >= writebuf_len {
                io_kyle::write_proto_routes(
                    &proto_routes,
                    cfg.paths.output.files.proto_routes,
                    true,
                )?;
                proto_routes.clear();
            }
        }
    }
    info!("{}", progress_bar);

    Ok(())
}
