mod pbf {
    pub use osmpbfreader::{reader::OsmPbfReader as Reader, OsmObj};
}

use crate::{
    network::{GraphBuilder, StreetType},
    units::{geo::Coordinate, speed::KilometersPerHour, Metric, MetricU8},
};
use log::info;
use std::fs::File;

pub struct Parser;
impl super::Parsing for Parser {
    fn parse_ways(file: File, graph_builder: &mut GraphBuilder) {
        info!("START Create edges from input-file.");
        for mut way in pbf::Reader::new(file)
            .par_iter()
            .filter_map(Result::ok)
            .filter_map(|obj| match obj {
                pbf::OsmObj::Way(way) => Some(way),
                _ => None,
            })
        {
            if way.nodes.len() < 2 {
                continue;
            }

            // collect relevant data from file
            let highway_tag = match StreetType::from(&way) {
                Some(highway_tag) => highway_tag,
                None => continue,
            };
            // TODO: get via json
            if !highway_tag.is_for_vehicles(false) {
                continue;
            }
            let lane_count = highway_tag.parse_lane_count(&way);
            let maxspeed = KilometersPerHour::new(highway_tag.parse_maxspeed(&way));
            let (is_oneway, is_reverse) = highway_tag.parse_oneway(&way);

            // create (proto-)edges
            if is_reverse {
                way.nodes.reverse();
            }
            let iter_range = if is_oneway {
                0..0
            } else {
                // if not oneway
                // -> add node-IDs reversed to generate edges forwards and backwards
                0..way.nodes.len() - 1
            };
            let mut nodes_iter = way.nodes.iter().chain(way.nodes[iter_range].iter().rev());

            // add edges, one per node-pair in way.nodes
            let mut src_id = nodes_iter
                .next()
                .expect(format!("Way.nodes.len()={} but should be >1.", way.nodes.len()).as_ref());
            for dst_id in nodes_iter {
                graph_builder.push_edge(
                    // Some(way.id.0),
                    src_id.0,
                    dst_id.0,
                    None,
                    maxspeed,
                    Some(vec![MetricU8::new(lane_count)]),
                );
                src_id = dst_id;
            }
        }
        info!("FINISHED");
    }

    fn parse_nodes(file: File, graph_builder: &mut GraphBuilder) {
        info!("START Create nodes from input-file.");
        for node in pbf::Reader::new(file)
            .par_iter()
            .filter_map(Result::ok)
            .filter_map(|obj| match obj {
                pbf::OsmObj::Node(node) => Some(node),
                _ => None,
            })
        {
            // add node to graph if it's part of an edge
            if graph_builder.is_node_in_edge(node.id.0) {
                graph_builder.push_node(
                    node.id.0,
                    Coordinate::new(node.decimicro_lat, node.decimicro_lon),
                );
            }
        }
        info!("FINISHED");
    }
}
