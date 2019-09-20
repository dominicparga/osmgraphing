use serde::{Deserialize, Serialize};

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
}
