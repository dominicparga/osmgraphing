mod pbf {
    pub use osmpbfreader::{reader::OsmPbfReader as Reader, OsmObj};
}

use crate::{
    configs::{graph, MetricType, VehicleType},
    network::{GraphBuilder, StreetType},
    units::{geo::Coordinate, speed::KilometersPerHour, MetricU8},
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

            // collect metrics as expected by user-config
            let mut proto_edge = intern::ProtoEdge::new_empty();
            for metric_type in cfg.edges.metric_types.iter() {
                match metric_type {
                    MetricType::Id => (), // taken later
                    &MetricType::Length { provided } => {
                        if !provided {
                            proto_edge.meters = None;
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be calculated, \
                                 but is expected to be provided.",
                                MetricType::Length { provided }
                            ));
                        }
                    }
                    &MetricType::Maxspeed { provided } => {
                        if provided {
                            proto_edge.maxspeed =
                                Some(KilometersPerHour::from(highway_tag.parse_maxspeed(&way)));
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be provided, \
                                 but is expected to be calculated.",
                                MetricType::Maxspeed { provided }
                            ));
                        }
                    }
                    &MetricType::Duration { provided } => {
                        if !provided {
                            proto_edge.duration = None;
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be calculated, \
                                 but is expected to be provided.",
                                MetricType::Duration { provided }
                            ));
                        }
                    }
                    MetricType::LaneCount => {
                        proto_edge.lane_count =
                            Some(MetricU8::new(highway_tag.parse_lane_count(&way)));
                    }
                    MetricType::Custom => {
                        return Err(format!("A pbf-file has no metric {}.", MetricType::Custom));
                    }
                    MetricType::Ignore => (),
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
                graph_builder.push_edge(
                    src_id.0,
                    dst_id.0,
                    proto_edge.meters,
                    proto_edge.maxspeed,
                    proto_edge.duration,
                    proto_edge.lane_count,
                    proto_edge.metric_u32,
                );
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

mod intern {
    use crate::units::{
        length::Meters, speed::KilometersPerHour, time::Milliseconds, MetricU32, MetricU8,
    };

    pub struct ProtoEdge {
        pub meters: Option<Meters>,
        pub maxspeed: Option<KilometersPerHour>,
        pub duration: Option<Milliseconds>,
        pub lane_count: Option<MetricU8>,
        pub metric_u32: Option<MetricU32>,
    }

    impl ProtoEdge {
        pub fn new_empty() -> ProtoEdge {
            ProtoEdge {
                meters: None,
                maxspeed: None,
                duration: None,
                lane_count: None,
                metric_u32: None,
            }
        }
    }
}
