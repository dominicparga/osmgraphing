pub mod fmi;
pub mod general;
pub mod pbf;

use super::{create_config, parse, TestType};
use osmgraphing::{
    configs::MetricCategory,
    helpers::Approx,
    network::{EdgeIdx, Graph, MetricIdx, Node, NodeContainer, NodeIdx},
    units::geo,
};
use std::{fmt, fmt::Display};

fn assert_nodes(test_nodes: &Vec<TestNode>, nodes: &NodeContainer) {
    for (expected, original) in test_nodes
        .iter()
        .map(|expected| (expected, TestNode::from(nodes.create(expected.idx))))
    {
        assert_eq!(
            expected, &original,
            "Expected node {} but graph-node is {}.",
            expected, original
        );
    }
}

#[derive(Debug, Eq)]
struct TestNode {
    name: String,
    id: i64,
    idx: NodeIdx,
    coord: geo::Coordinate,
}

impl From<Node> for TestNode {
    fn from(node: Node) -> TestNode {
        TestNode {
            name: "node-from-graph".to_owned(),
            id: node.id(),
            idx: node.idx(),
            coord: node.coord(),
        }
    }
}

impl Display for TestNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, idx: {}, coord: {} }}",
            self.id, self.idx, self.coord,
        )
    }
}

impl PartialEq for TestNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.idx == other.idx && self.coord == other.coord
    }
}

impl TestNode {
    fn new(name: &str, id: i64, lat: f64, lon: f64, graph: &Graph) -> TestNode {
        let idx = graph
            .nodes()
            .idx_from(id)
            .expect(&format!("The node-id {} is not in graph.", id));
        TestNode {
            name: String::from(name),
            id,
            idx,
            coord: geo::Coordinate::from((lat, lon)),
        }
    }
}

struct TestEdge {
    name: String,
    edge_idx: EdgeIdx,
    is_fwd: bool,
    src_idx: NodeIdx,
    dst_idx: NodeIdx,
    metrics: Vec<f32>,
}

impl TestEdge {
    fn new_fwd(
        name: Option<&str>,
        edge_idx: EdgeIdx,
        src: &TestNode,
        dst: &TestNode,
        length: f32,
        maxspeed: f32,
        duration: f32,
    ) -> TestEdge {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            edge_idx,
            is_fwd: true,
            src_idx: src.idx.into(),
            dst_idx: dst.idx.into(),
            metrics: vec![length, maxspeed, duration],
        }
    }

    fn new_bwd(
        name: Option<&str>,
        edge_idx: EdgeIdx,
        src: &TestNode,
        dst: &TestNode,
        length: f32,
        maxspeed: f32,
        duration: f32,
    ) -> TestEdge {
        TestEdge {
            name: (name.unwrap_or(&format!("{}->{}", src.name, dst.name))).to_owned(),
            edge_idx,
            is_fwd: false,
            src_idx: src.idx.into(),
            dst_idx: dst.idx.into(),
            metrics: vec![length, maxspeed, duration],
        }
    }

    fn assert_correct(&self, graph: &Graph) {
        // get graph-components dependent on own direction
        let fwd_edges = graph.fwd_edges();
        let bwd_edges = graph.bwd_edges();
        let (edge, edge_idx) = {
            if self.is_fwd {
                fwd_edges
                    .between(self.src_idx, self.dst_idx)
                    .expect(&format!(
                        "Fwd-edge (src_idx, dst_idx): ({}, {}) does not exist.",
                        self.src_idx, self.dst_idx
                    ))
            } else {
                bwd_edges
                    .between(self.src_idx, self.dst_idx)
                    .expect(&format!(
                        "Bwd-edge (src_idx, dst_idx): ({}, {}) does not exist.",
                        self.src_idx, self.dst_idx
                    ))
            }
        };
        let prefix = {
            if self.is_fwd {
                "fwd-"
            } else {
                "bwd-"
            }
        };

        assert_eq!(
            edge_idx, self.edge_idx,
            "Wrong {}edge-idx={} for {}",
            prefix, edge_idx, self.name
        );
        assert_eq!(
            edge.dst_idx(),
            self.dst_idx,
            "Wrong dst_idx={} for {}edge {}",
            edge.dst_idx(),
            prefix,
            self.name
        );

        let access_stuff = vec![
            (MetricIdx(0), MetricCategory::Length),
            (MetricIdx(1), MetricCategory::Maxspeed),
            (MetricIdx(2), MetricCategory::Duration),
        ];
        for (metric_idx, metric_category) in access_stuff {
            let value = edge
                .metric(metric_idx)
                .expect(&format!("Edge should have a {}.", metric_category));
            assert!(
                value.approx_eq(&self.metrics[*metric_idx]),
                "Wrong {}={} for {}edge {}",
                metric_category,
                value,
                prefix,
                self.name
            );
        }
    }
}
