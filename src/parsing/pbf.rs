mod pbf {
    pub use osmpbfreader::{reader::OsmPbfReader as Reader, OsmObj};
}

use crate::{
    configs::{graph, MetricType, VehicleType},
    network::{GraphBuilder, ProtoEdge, StreetType},
    units::{geo::Coordinate, MetricU32},
};
use log::info;
use std::fs::File;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
}

impl super::Parsing for Parser {
    fn parse_ways(
        &self,
        file: File,
        graph_builder: &mut GraphBuilder,
        cfg: &graph::Config,
    ) -> Result<(), String> {
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

            // collect relevant data from file, if way-type is as expected by user
            let highway_tag = match StreetType::from(&way) {
                Some(highway_tag) => highway_tag,
                None => continue,
            };
            match cfg.vehicle_type {
                VehicleType::Car => {
                    if !highway_tag.is_for_vehicles(cfg.is_graph_suitable) {
                        continue;
                    }
                }
                VehicleType::Bicycle => {
                    if !highway_tag.is_for_bicycles(cfg.is_graph_suitable) {
                        continue;
                    }
                }
                VehicleType::Pedestrian => {
                    if !highway_tag.is_for_pedestrians(cfg.is_graph_suitable) {
                        continue;
                    }
                }
            }

            // Collect metrics as expected by user-config
            // ATTENTION: A way contains multiple edges, thus be careful when adding new metrics.
            let mut edge_metrics = vec![];
            for metric_type in cfg.edges.metric_types.iter() {
                match metric_type {
                    MetricType::Id { id: _ } => (), // taken later
                    &MetricType::Length { provided } => {
                        if provided {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be calculated, \
                                 but is expected to be provided.",
                                MetricType::Length { provided }
                            ));
                        }
                    }
                    &MetricType::Maxspeed { provided } => {
                        if provided {
                            let maxspeed = MetricU32::from(highway_tag.parse_maxspeed(&way));
                            edge_metrics.push((metric_type.id().to_owned(), maxspeed));
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be provided, \
                                 but is expected to be calculated.",
                                MetricType::Maxspeed { provided }
                            ));
                        }
                    }
                    &MetricType::Duration { provided } => {
                        if provided {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be calculated, \
                                 but is expected to be provided.",
                                MetricType::Duration { provided }
                            ));
                        }
                    }
                    MetricType::LaneCount => {
                        let lane_count = MetricU32::from(highway_tag.parse_lane_count(&way));
                        edge_metrics.push((metric_type.id().to_owned(), lane_count));
                    }
                    &MetricType::Custom { id: _ } => {
                        return Err(format!(
                            "A pbf-file has no metric {}.",
                            MetricType::Custom {
                                id: metric_type.id().to_owned()
                            }
                        ));
                    }
                    MetricType::Ignore { id: _ } => (),
                }
            }
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
            let mut src_id = nodes_iter.next().ok_or(format!(
                "Way.nodes.len()={} but should be >1.",
                way.nodes.len()
            ))?;
            for dst_id in nodes_iter {
                // create proto-edge, add metrics and add proto-edge to graph
                let mut proto_edge = ProtoEdge::new(src_id.0, dst_id.0);
                for (metric_id, metric_value) in edge_metrics.iter() {
                    proto_edge.add_metric(&metric_id, *metric_value);
                }
                graph_builder.push_edge(proto_edge);
                // update src for next edge (in the current way)
                src_id = dst_id;
            }
        }
        info!("FINISHED");
        Ok(())
    }

    fn parse_nodes(
        &self,
        file: File,
        graph_builder: &mut GraphBuilder,
        _cfg: &graph::Config,
    ) -> Result<(), String> {
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
                    Coordinate::from((node.decimicro_lat, node.decimicro_lon)),
                );
            }
        }
        info!("FINISHED");
        Ok(())
    }
}
