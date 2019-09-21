use serde::{Deserialize, Serialize};

use osmgraphing::network::Graph;

#[derive(Serialize, Deserialize, Clone)]
pub struct EdgeInfo {
    pub src_id: i64,
    pub dst_id: i64,
    pub decimicro_lat: i32,
    pub decimicro_lon: i32,
    pub is_src: bool,
    pub is_dst: bool,
    pub lane_count: u8,
    pub length_m: u32,
    pub route_count: u16,
}
impl<'a> EdgeInfo {
    pub fn head_line() -> Vec<&'a str> {
        vec![
            "src-id",
            "dst-id",
            "decimicro-lat",
            "decimicro-lon",
            "is-src",
            "is-dst",
            "lane-count",
            "lenth-m",
            "route-count",
        ]
    }

    pub fn from(small_edge_info: SmallEdgeInfo, graph: &Graph) -> EdgeInfo {
        let edge = graph.edge(small_edge_info.edge_idx);

        let edge_src = graph.node(edge.src_idx());
        let edge_dst = graph.node(edge.dst_idx());

        EdgeInfo {
            src_id: edge_src.id(),
            dst_id: edge_dst.id(),
            decimicro_lat: {
                (edge_src.coord().decimicro_lat() + edge_dst.coord().decimicro_lat()) / 2
            },
            decimicro_lon: {
                (edge_src.coord().decimicro_lon() + edge_dst.coord().decimicro_lon()) / 2
            },
            is_src: small_edge_info.is_src,
            is_dst: small_edge_info.is_dst,
            lane_count: edge.lane_count(),
            length_m: edge.meters(),
            route_count: small_edge_info.route_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SmallEdgeInfo {
    pub edge_idx: usize,
    pub is_src: bool,
    pub is_dst: bool,
    pub route_count: u16,
}
