use std::path;

use log::info;
use osmgraphing::{routing, Parser};
use rand::distributions::{Distribution, Uniform};
use rand::{Rng, SeedableRng};

//------------------------------------------------------------------------------------------------//
// own modules

use super::io_kyle;

//------------------------------------------------------------------------------------------------//
// config

pub mod config {
    use std::path;

    pub struct Config<'a, P: AsRef<path::Path> + ?Sized> {
        pub paths: Paths<'a, P>,
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
    // prepare simulation

    // check path of io-files before expensive simulation
    io_kyle::create_file(cfg.paths.output.files.proto_routes)?;

    let graph = Parser::parse_and_finalize(&cfg.paths.input.files.map)?;
    println!("{}", graph);

    let mut rng = rand_pcg::Pcg32::seed_from_u64(123);
    let (rnd_src_idx, rnd_dst_idx): (usize, usize) = rng.gen();
    println!("({}, {})", rnd_src_idx, rnd_dst_idx);

    // //--------------------------------------------------------------------------------------------//
    // // routing

    // let mut astar = routing::factory::new_shortest_path_astar();

    // let seed = &[1, 2, 3, 4];
    // let mut rng = rand::thread_rng();
    // let die = Uniform::from(0..graph.node_count());
    // let throw = die.sample(&mut rng);
    // // routes
    // let src_idx = 0;
    // let dsts: Vec<usize> = (0..graph.node_count()).collect();

    // // calculate
    // let src = graph.node(src_idx);
    // for dst_idx in dsts {
    //     let dst = graph.node(dst_idx);

    //     let option_path = astar.compute_best_path(src.id(), dst.id(), &graph);
    //     if let Some(path) = option_path {
    //         info!("Distance {} m from ({}) to ({}).", path.cost(), src, dst);
    //     } else {
    //         info!("No path from ({}) to ({}).", src, dst);
    //     }
    // }

    Ok(())
}
