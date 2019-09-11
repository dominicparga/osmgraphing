use std::ffi::OsString;

use log::error;

use super::{TestEdge, TestNode};
use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//

#[test]
fn parse() {
    let path = OsString::from("resources/maps/small.fmi");
    let _graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };
}

#[test]
fn graph_construction() {
    let path = OsString::from("resources/maps/small.fmi");
    let graph = match Parser::parse(&path) {
        Ok(graph) => graph,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

    //--------------------------------------------------------------------------------------------//
    // setup correct data

    // nodes sorted by id
    // name, id, decimicro_lat, decimicro_lon
    let node_opp = TestNode::from("Oppenweiler", 26_033_921, 48_9840100, 9_4589188, &graph);
    let node_bac = TestNode::from("Backnang", 26_160_028, 48_9416023, 9_4332023, &graph);
    let node_wai = TestNode::from("Waiblingen", 252_787_940, 48_8271096, 9_3098661, &graph);
    let node_end = TestNode::from("Endersbach", 298_249_467, 48_8108510, 9_3679493, &graph);
    let node_dea = TestNode::from("Dead-end", 1_621_605_361, 48_9396327, 9_4188680, &graph);
    let node_stu = TestNode::from("Stuttgart", 2_933_335_353, 48_7701757, 9_1565768, &graph);

    // Due to the offset-array, the edge-ids should match with sorting by src-id, then by dst-id.
    // -> testing offset-array
    // name, id, src, dst, meters, maxspeed
    let edge_opp_bac = TestEdge::from("Oppenweiler->Backnang", 0, &node_opp, &node_bac, 8_000, 50);
    let edge_bac_opp = TestEdge::from("Backnang->Oppenweiler", 1, &node_bac, &node_opp, 8_000, 50);
    let edge_bac_wai = TestEdge::from("Backnang->Waiblingen", 2, &node_bac, &node_wai, 23_000, 120);
    let edge_bac_end = TestEdge::from("Backnang->Endersbach", 3, &node_bac, &node_end, 22_000, 80);
    // 1_069 is the length of a straight line, since the file contains trash in there.
    let edge_bac_dea = TestEdge::from("Backnang->Dead-end", 4, &node_bac, &node_dea, 1_069, 30);
    let edge_wai_bac = TestEdge::from("Waiblingen->Backnang", 5, &node_wai, &node_bac, 23_000, 120);
    let edge_wai_end = TestEdge::from("Waiblingen->Endersbach", 6, &node_wai, &node_end, 8_000, 50);
    let edge_wai_stu = TestEdge::from(
        "Waiblingen->Stuttgart",
        7,
        &node_wai,
        &node_stu,
        17_000,
        100,
    );
    let edge_end_bac = TestEdge::from("Endersbach->Backnang", 8, &node_end, &node_bac, 22_000, 80);
    let edge_end_wai = TestEdge::from("Endersbach->Waiblingen", 9, &node_end, &node_wai, 8_000, 50);
    let edge_end_stu = TestEdge::from(
        "Endersbach->Stuttgart",
        10,
        &node_end,
        &node_stu,
        21_000,
        80,
    );
    let edge_stu_wai = TestEdge::from(
        "Stuttgart->Waiblingen",
        11,
        &node_stu,
        &node_wai,
        17_000,
        100,
    );
    let edge_stu_end = TestEdge::from(
        "Stuttgart->Endersbach",
        12,
        &node_stu,
        &node_end,
        21_000,
        80,
    );

    //--------------------------------------------------------------------------------------------//
    // testing graph

    assert_eq!(graph.node_count(), 6, "Wrong node-count");
    assert_eq!(graph.edge_count(), 13, "Wrong edge-count");
    assert!(
        graph.edge(24, 42).is_none(),
        "Edge doesn't exist, so graph should return None."
    );
    assert!(
        graph.leaving_edges(424).is_none(),
        "Node's idx is too high, thus the node should not have any leaving edges."
    );
    assert!(
        graph.leaving_edges(node_dea.idx).is_none(),
        "Node has no leaving edges, so the method should return None."
    );

    //--------------------------------------------------------------------------------------------//
    // testing nodes

    node_opp.assert(&graph);
    node_bac.assert(&graph);
    node_end.assert(&graph);
    node_wai.assert(&graph);
    node_dea.assert(&graph);
    node_stu.assert(&graph);

    //--------------------------------------------------------------------------------------------//
    // testing edges

    edge_opp_bac.assert(&graph);
    edge_bac_opp.assert(&graph);
    edge_bac_wai.assert(&graph);
    edge_bac_end.assert(&graph);
    edge_bac_dea.assert(&graph);
    edge_wai_bac.assert(&graph);
    edge_wai_end.assert(&graph);
    edge_wai_stu.assert(&graph);
    edge_end_bac.assert(&graph);
    edge_end_wai.assert(&graph);
    edge_end_stu.assert(&graph);
    edge_stu_wai.assert(&graph);
    edge_stu_end.assert(&graph);
}
