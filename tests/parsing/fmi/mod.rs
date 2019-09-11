mod small;

//------------------------------------------------------------------------------------------------//

use osmgraphing::network::{geo, Graph};

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
            .expect(&format!("The node_id {} is not in graph.", id));
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
