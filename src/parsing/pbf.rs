mod pbf {
    pub use osmpbfreader::{reader::OsmPbfReader as Reader, OsmObj};
}

use crate::{
    configs::{graph, MetricCategory},
    network::{GraphBuilder, MetricIdx, ProtoEdge, StreetType},
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
            if !highway_tag.is_for(&cfg.vehicles.vehicle_type, cfg.vehicles.is_driver_picky) {
                continue;
            }

            // Collect metrics as expected by user-config
            // ATTENTION: A way contains multiple edges, thus be careful when adding new metrics.
            let cfg = &cfg.edges.metrics;
            let mut metric_values = vec![None; cfg.count()];
            for metric_idx in (0..cfg.count()).map(MetricIdx) {
                let metric_type = cfg.category(metric_idx);
                let is_provided = cfg.is_provided(metric_idx);

                match metric_type {
                    MetricCategory::Length | MetricCategory::Duration | MetricCategory::Custom => {
                        if is_provided {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be calculated, \
                                 but is expected to be provided.",
                                metric_type
                            ));
                        }
                    }
                    MetricCategory::Maxspeed => {
                        if is_provided {
                            let maxspeed = MetricU32::from(highway_tag.parse_maxspeed(&way));
                            metric_values[*metric_idx] = Some(maxspeed);
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be provided, \
                                 but is expected to be calculated.",
                                metric_type
                            ));
                        }
                    }
                    MetricCategory::LaneCount => {
                        if is_provided {
                            let lane_count = MetricU32::from(highway_tag.parse_lane_count(&way));
                            metric_values[*metric_idx] = Some(lane_count);
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be provided, \
                                 but is expected to be calculated.",
                                metric_type
                            ));
                        }
                    }
                    MetricCategory::Id | MetricCategory::Ignore => (),
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
            let mut src_id = nodes_iter
                .next()
                .ok_or(format!(
                    "Way.nodes.len()={} but should be >1.",
                    way.nodes.len()
                ))?
                .0;
            for dst_id in nodes_iter.map(|id| id.0) {
                // create proto-edge
                let proto_edge = ProtoEdge {
                    src_id,
                    dst_id,
                    metrics: metric_values.clone(),
                };
                // add proto-edge to graph
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
