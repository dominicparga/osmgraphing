use std::ffi::OsString;

use osmgraphing::osm;
use osmgraphing::osm::geo;
use osmgraphing::routing;

//------------------------------------------------------------------------------------------------//
// helpers

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
        graph: &routing::Graph,
    ) -> TestNode {
        let idx = graph
            .node_idx_from(id)
            .expect(&format!("The node_id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            decimicro_lat,
            decimicro_lon,
        }
    }

    fn assert(&self, graph: &routing::Graph) {
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
        name: &str,
        id: i64,
        src: &TestNode,
        dst: &TestNode,
        meters: u32,
        maxspeed: u16,
    ) -> TestEdge {
        TestEdge {
            name: String::from(name),
            id,
            src_idx: src.idx,
            dst_idx: dst.idx,
            meters,
            maxspeed,
        }
    }

    fn assert(&self, graph: &routing::Graph) {
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
// tests

#[test]
fn parsing() {
    let path = OsString::from("resources/osm/small.fmi");

    let parser = osm::fmi::Parser;
    let graph = parser.parse(&path);

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

#[test]
fn file_support() {
    assert!(
        osm::Support::from_path(&OsString::from("foo.asdf")).is_err(),
        "File-extension 'asdf' should not be supported."
    );

    let support = osm::Support::from_path(&OsString::from("foo.fmi"));
    assert!(support.is_ok(), "File-extension 'fmi' is not supported.");
    let support = support.unwrap();

    assert_eq!(support, osm::Support::FMI);
    assert_ne!(support, osm::Support::PBF);
    assert_ne!(support, osm::Support::XML);
}
