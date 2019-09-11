use std::fs::File;

use log::info;

use crate::network::{geo, GraphBuilder, StreetType};

mod pbf {
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::OsmObj;
}

//------------------------------------------------------------------------------------------------//

pub struct Parser;
impl super::Parsing for Parser {
    fn parse_ways(file: File, graph_builder: &mut GraphBuilder) {
        info!("Starting edge-creation ..");
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
            let maxspeed = highway_tag.parse_maxspeed(&way);
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
                graph_builder.push_edge(Some(way.id.0), src_id.0, dst_id.0, None, maxspeed);
                src_id = dst_id;
            }
        }
        info!("Finished edge-creation");
    }

    fn parse_nodes(file: File, graph_builder: &mut GraphBuilder) {
        info!("Starting node-creation ..");
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
                    geo::Coordinate::new(node.decimicro_lat, node.decimicro_lon),
                );
            }
        }
        info!("Finished node-creation");
    }
}
