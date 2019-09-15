use log::info;
use osmgraphing::{routing, Parser};

//------------------------------------------------------------------------------------------------//

pub struct Config<'a> {
    pub mapfile: &'a str,
}

//------------------------------------------------------------------------------------------------//

pub fn run(cfg: Config) -> Result<(), String> {
    info!("Executing braess-optimization");

    //--------------------------------------------------------------------------------------------//
    // parsing

    let mut graphbuilder = Parser::parse(&cfg.mapfile)?;
    graphbuilder.filter_edges(|_proto_edge| -> bool { true })?;
    let graph = graphbuilder.finalize()?;
    println!("{}", graph);

    //--------------------------------------------------------------------------------------------//
    // routing

    let mut astar = routing::factory::new_shortest_path_astar();

    // routes
    let src_idx = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();

    // calculate
    let src = graph.node(src_idx);
    for dst_idx in dsts {
        let dst = graph.node(dst_idx);

        let option_path = astar.compute_shortest_path(src.id(), dst.id(), &graph);
        if let Some(path) = option_path {
            info!("Distance {} m from ({}) to ({}).", path.cost(), src, dst);
        } else {
            info!("No path from ({}) to ({}).", src, dst);
        }
    }

    Ok(())
}
