use log::info;

use osmgraphing::routing;

//------------------------------------------------------------------------------------------------//

#[test]
#[ignore]
fn dijkstra() {
    let graph = super::parse("resources/maps/small.fmi");

    // routing
    let mut dijkstra = routing::Dijkstra::new(&graph);
    let src_idx = 0;
    let dsts: Vec<usize> = (0..graph.node_count()).collect();
    // let dsts: Vec<usize> = vec![80]; problem on baden-wuerttemberg.osm.pbf

    let src = graph.node(src_idx);

    for dst_idx in dsts {
        let dst = graph.node(dst_idx);

        info!("");

        let path = dijkstra.compute_shortest_path(src_idx, dst_idx);
        info!(
            "Distance {} m from ({}) to ({}).",
            path.cost[dst_idx], src, dst
        );
    }
}
