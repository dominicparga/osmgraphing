use std::ffi::OsString;

use log::error;

use osmgraphing::network::{geo, Graph};
use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------------------//

struct TestNode {
    name: String,
    id: i64,
    idx: usize,
    decimicro_lat: i32,
    decimicro_lon: i32,
}
impl TestNode {
    fn from(
        name: &str,
        id: i64,
        decimicro_lat: i32,
        decimicro_lon: i32,
        graph: &Graph,
    ) -> TestNode {
        let idx = graph
            .node_idx_from(id)
            .expect(&format!("The node-id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            decimicro_lat,
            decimicro_lon,
        }
    }

    fn assert(&self, graph: &Graph) {
        let node = graph.node(self.idx);
        let coord = geo::Coordinate::new(self.decimicro_lat, self.decimicro_lon);
        assert_eq!(
            node.id(),
            self.id,
            "Wrong node_id={} for {}",
            node.id(),
            self.name
        );
        assert_eq!(
            node.coord(),
            &coord,
            "Wrong coordinate {} for {}",
            node.coord(),
            self.name
        );
    }
}

//------------------------------------------------------------------------------------------------//

struct TestEdge {
    name: String,
    id: i64,
    src_idx: usize,
    dst_idx: usize,
    meters: u32,
    maxspeed: u16,
}
impl TestEdge {
    fn from(
        name: Option<&str>,
        id: i64,
        src: &TestNode,
        dst: &TestNode,
        meters: u32,
        maxspeed: u16,
    ) -> TestEdge {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            id,
            src_idx: src.idx,
            dst_idx: dst.idx,
            meters,
            maxspeed,
        }
    }

    fn assert(&self, graph: &Graph) {
        let edge = graph.edge(self.src_idx, self.dst_idx).expect(&format!(
            "Edge (src_idx, dst_idx): ({}, {}) does not exist.",
            self.src_idx, self.dst_idx
        ));

        assert_eq!(
            edge.id(),
            self.id,
            "Wrong edge_id={} for {}",
            edge.id(),
            self.name
        );
        assert_eq!(
            edge.src_idx(),
            self.src_idx,
            "Wrong src_idx={} for {}",
            edge.src_idx(),
            self.name
        );
        assert_eq!(
            edge.dst_idx(),
            self.dst_idx,
            "Wrong dst_idx={} for {}",
            edge.dst_idx(),
            self.name
        );
        assert_eq!(
            edge.meters(),
            self.meters,
            "Wrong meters={} for {}",
            edge.meters(),
            self.name
        );
        assert_eq!(
            edge.maxspeed(),
            self.maxspeed,
            "Wrong maxspeed={} for {}",
            edge.maxspeed(),
            self.name
        );
    }
}

//------------------------------------------------------------------------------------------------//

#[test]
fn small() {
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
    let node_a = TestNode::from("a", 0, 0_1234567, 0_1234567, &graph);
    let node_b = TestNode::from("b", 1, 1_2345678, 1_2345678, &graph);
    let node_c = TestNode::from("c", 2, 2_3456789, 2_3456789, &graph);
    let node_d = TestNode::from("d", 3, 3_4567890, 3_4567890, &graph);
    let node_e = TestNode::from("e", 4, 4_5678901, 4_5678901, &graph);
    let node_f = TestNode::from("f", 5, 5_6789012, 5_6789012, &graph);
    let node_g = TestNode::from("g", 6, 6_7890123, 6_7890123, &graph);
    let node_h = TestNode::from("h", 7, 7_8901234, 7_8901234, &graph);

    // Due to the offset-array, the edge-ids should match with sorting by src-id, then by dst-id.
    // -> testing offset-array
    // name, id, src, dst, meters, maxspeed
    let edge_b_a = TestEdge::from(None, 0, &node_b, &node_a, 1_000, 30);
    let edge_b_c = TestEdge::from(None, 1, &node_b, &node_a, 1_000, 30);
    let edge_c_a = TestEdge::from(None, 2, &node_b, &node_a, 1_000, 30);
    let edge_c_b = TestEdge::from(None, 3, &node_b, &node_a, 1_000, 30);
    let edge_d_b = TestEdge::from(None, 4, &node_b, &node_a, 1_000, 30);
    let edge_d_e = TestEdge::from(None, 5, &node_b, &node_a, 2_000, 30);
    let edge_d_h = TestEdge::from(None, 6, &node_b, &node_a, 1_000, 30);
    let edge_e_d = TestEdge::from(None, 7, &node_b, &node_a, 2_000, 30);
    let edge_e_f = TestEdge::from(None, 8, &node_b, &node_a, 1_000, 30);
    let edge_f_e = TestEdge::from(None, 9, &node_b, &node_a, 1_000, 30);
    let edge_f_h = TestEdge::from(None, 10, &node_b, &node_a, 1_000, 30);
    let edge_g_e = TestEdge::from(None, 11, &node_b, &node_a, 1_000, 30);
    let edge_g_f = TestEdge::from(None, 12, &node_b, &node_a, 1_000, 30);
    let edge_h_c = TestEdge::from(None, 13, &node_b, &node_a, 4_000, 30);
    let edge_h_d = TestEdge::from(None, 14, &node_b, &node_a, 1_000, 30);
    let edge_h_f = TestEdge::from(None, 15, &node_b, &node_a, 1_000, 30);

    //--------------------------------------------------------------------------------------------//
    // testing graph

    assert_eq!(graph.node_count(), 8, "Wrong node-count");
    assert_eq!(graph.edge_count(), 16, "Wrong edge-count");
    assert!(
        graph.edge(24, 42).is_none(),
        "Edge doesn't exist, so graph should return None."
    );
    assert!(
        graph.leaving_edges(424).is_none(),
        "Node's idx is too high, thus the node should not have any leaving edges."
    );
    assert!(
        graph.leaving_edges(node_a.idx).is_none(),
        "Node has no leaving edges, so the method should return None."
    );

    //--------------------------------------------------------------------------------------------//
    // testing nodes

    node_a.assert(&graph);
    node_b.assert(&graph);
    node_c.assert(&graph);
    node_d.assert(&graph);
    node_e.assert(&graph);
    node_f.assert(&graph);
    node_g.assert(&graph);
    node_h.assert(&graph);

    //--------------------------------------------------------------------------------------------//
    // testing edges

    edge_b_a.assert(&graph);
    edge_b_c.assert(&graph);
    edge_c_a.assert(&graph);
    edge_c_b.assert(&graph);
    edge_d_b.assert(&graph);
    edge_d_e.assert(&graph);
    edge_d_h.assert(&graph);
    edge_e_d.assert(&graph);
    edge_e_f.assert(&graph);
    edge_f_e.assert(&graph);
    edge_f_h.assert(&graph);
    edge_g_e.assert(&graph);
    edge_g_f.assert(&graph);
    edge_h_c.assert(&graph);
    edge_h_d.assert(&graph);
    edge_h_f.assert(&graph);
}

#[test]
fn simple_stuttgart() {
    let path = OsString::from("resources/maps/simple_stuttgart.fmi");
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
    let edge_opp_bac = TestEdge::from(None, 0, &node_opp, &node_bac, 8_000, 50);
    let edge_bac_opp = TestEdge::from(None, 1, &node_bac, &node_opp, 8_000, 50);
    let edge_bac_wai = TestEdge::from(None, 2, &node_bac, &node_wai, 23_000, 120);
    let edge_bac_end = TestEdge::from(None, 3, &node_bac, &node_end, 22_000, 80);
    // 1_069 is the length of a straight line, since the file contains trash in there.
    let edge_bac_dea = TestEdge::from(None, 4, &node_bac, &node_dea, 1_069, 30);
    let edge_wai_bac = TestEdge::from(None, 5, &node_wai, &node_bac, 23_000, 120);
    let edge_wai_end = TestEdge::from(None, 6, &node_wai, &node_end, 8_000, 50);
    let edge_wai_stu = TestEdge::from(None, 7, &node_wai, &node_stu, 17_000, 100);
    let edge_end_bac = TestEdge::from(None, 8, &node_end, &node_bac, 22_000, 80);
    let edge_end_wai = TestEdge::from(None, 9, &node_end, &node_wai, 8_000, 50);
    let edge_end_stu = TestEdge::from(None, 10, &node_end, &node_stu, 21_000, 80);
    let edge_stu_wai = TestEdge::from(None, 11, &node_stu, &node_wai, 17_000, 100);
    let edge_stu_end = TestEdge::from(None, 12, &node_stu, &node_end, 21_000, 80);

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
