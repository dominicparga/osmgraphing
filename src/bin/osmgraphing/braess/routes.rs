// use log::info;
// use osmgraphing::{routing, Parser};
// use rand;
// use rand::distributions::{Distribution, Uniform};

// // ------------------------------------------------------------------------------------------------//
// // config

// pub struct Config<'a> {
//     pub out: &'a str,
//     pub mapfile: &'a str,
//     pub route_count: usize,
// }

// //------------------------------------------------------------------------------------------------//

// pub fn search_and_export(cfg: Config) -> Result<(), String> {
//     info!("Executing route-generator");

//     //--------------------------------------------------------------------------------------------//
//     // parsing

//     let graph = Parser::parse_and_finalize(&cfg.mapfile)?;
//     println!("{}", graph);

//     //--------------------------------------------------------------------------------------------//
//     // routing

//     let mut astar = routing::factory::new_shortest_path_astar();

//     let seed = &[1, 2, 3, 4];
//     let mut rng = rand::thread_rng();
//     let die = Uniform::from(0..graph.node_count());
//     let throw = die.sample(&mut rng);
//     // routes
//     let src_idx = 0;
//     let dsts: Vec<usize> = (0..graph.node_count()).collect();

//     // calculate
//     let src = graph.node(src_idx);
//     for dst_idx in dsts {
//         let dst = graph.node(dst_idx);

//         let option_path = astar.compute_shortest_path(src.id(), dst.id(), &graph);
//         if let Some(path) = option_path {
//             info!("Distance {} m from ({}) to ({}).", path.cost(), src, dst);
//         } else {
//             info!("No path from ({}) to ({}).", src, dst);
//         }
//     }

//     Ok(())
// }
