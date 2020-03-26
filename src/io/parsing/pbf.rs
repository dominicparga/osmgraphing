use crate::{
    configs::parser::{self, EdgeCategory},
    defaults::{self, capacity::DimVec},
    helpers,
    network::{EdgeBuilder, MetricIdx, NodeBuilder, ProtoEdge, ProtoNode, StreetCategory},
};
use kissunits::geo::Coordinate;
use log::info;
use osmpbfreader::{reader::OsmPbfReader, OsmObj};
use smallvec::smallvec;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
}

impl super::Parsing for Parser {
    fn preprocess(&mut self, cfg: &parser::Config) -> Result<(), String> {
        info!("START Start preprocessing pbf-parser.");
        super::check_parser_config(cfg)?;
        info!("FINISHED");
        Ok(())
    }

    fn parse_ways(&self, builder: &mut EdgeBuilder) -> Result<(), String> {
        info!("START Create edges from input-file.");
        let file = helpers::open_file(&builder.cfg().map_file)?;
        for mut way in OsmPbfReader::new(file)
            .par_iter()
            .filter_map(Result::ok)
            .filter_map(|obj| match obj {
                OsmObj::Way(way) => Some(way),
                _ => None,
            })
        {
            if way.nodes.len() < 2 {
                continue;
            }

            // collect relevant data from file, if way-type is as expected by user
            let highway_tag = match StreetCategory::from(&way) {
                Some(highway_tag) => highway_tag,
                None => continue,
            };
            if !highway_tag.is_for(
                &builder.cfg().vehicles.category,
                builder.cfg().vehicles.are_drivers_picky,
            ) {
                continue;
            }

            // get nodes of way to create proto-edges later
            let (is_oneway, is_reverse) = highway_tag.parse_oneway(&way);
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
            let nodes: Vec<i64> = way
                .nodes
                .iter()
                .chain(way.nodes[iter_range].iter().rev())
                .map(|id| id.0)
                .collect();

            // Collect metrics as expected by user-config
            // ATTENTION: A way contains multiple edges, thus be careful when adding new metrics.
            let mut metrics: DimVec<_> = smallvec![None; builder.cfg().edges.dim()];
            for metric_idx in (0..builder.cfg().edges.dim()).map(MetricIdx) {
                let category = builder.cfg().edges.metric_category(metric_idx);
                let is_provided = builder.cfg().edges.is_metric_provided(metric_idx);

                match category {
                    EdgeCategory::Meters | EdgeCategory::Seconds | EdgeCategory::Custom => {
                        if is_provided {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be calculated, \
                                 but is expected to be provided.",
                                category
                            ));
                        }
                    }
                    EdgeCategory::KilometersPerHour => {
                        if is_provided {
                            let maxspeed =
                                defaults::speed::TYPE::from(highway_tag.parse_maxspeed(&way));
                            metrics[*metric_idx] = Some(*maxspeed);
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be provided, \
                                 but is expected to be calculated.",
                                category
                            ));
                        }
                    }
                    EdgeCategory::LaneCount => {
                        if is_provided {
                            let lane_count = highway_tag.parse_lane_count(&way);
                            metrics[*metric_idx] = Some(lane_count as f64);
                        } else {
                            return Err(format!(
                                "The {} of an edge in a pbf-file has to be provided, \
                                 but is expected to be calculated.",
                                category
                            ));
                        }
                    }
                    EdgeCategory::SrcId
                    | EdgeCategory::IgnoredSrcIdx
                    | EdgeCategory::DstId
                    | EdgeCategory::IgnoredDstIdx
                    | EdgeCategory::ShortcutEdgeIdx
                    | EdgeCategory::Ignore => (),
                }
            }

            // for n nodes in a way, you can create (n-1) edges
            for node_idx in 0..(nodes.len() - 1) {
                // add proto-edge to graph
                builder.insert(ProtoEdge {
                    src_id: nodes[node_idx],
                    dst_id: nodes[node_idx + 1],
                    metrics: metrics.clone(),
                });
            }
        }
        info!("FINISHED");
        Ok(())
    }

    fn parse_nodes(&self, builder: &mut NodeBuilder) -> Result<(), String> {
        info!("START Create nodes from input-file.");
        let cfg = builder.cfg();

        let file = helpers::open_file(&cfg.map_file)?;
        for node in OsmPbfReader::new(file)
            .par_iter()
            .filter_map(Result::ok)
            .filter_map(|obj| match obj {
                OsmObj::Node(node) => Some(node),
                _ => None,
            })
        {
            // add node to graph if it's part of an edge
            builder.insert(ProtoNode {
                id: node.id.0,
                coord: Coordinate::from_decimicro(node.decimicro_lat, node.decimicro_lon),
                level: None,
            });
        }
        info!("FINISHED");
        Ok(())
    }
}
