use crate::{
    configs::parsing::{self, edges},
    defaults::capacity::DimVec,
    helpers::err,
    network::{EdgeBuilder, NodeBuilder, ProtoEdge, ProtoNode, StreetCategory},
};
use kissunits::geo::Coordinate;
use log::info;
use osmpbfreader::{reader::OsmPbfReader, OsmObj};
use smallvec::smallvec;
use std::fs::OpenOptions;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
}

impl super::Parsing for Parser {
    fn preprocess(&mut self, cfg: &parsing::Config) -> err::Feedback {
        info!("START Start preprocessing pbf-parser.");
        super::check_config(cfg)?;

        for category in cfg.edges.categories.iter() {
            match category {
                edges::Category::Meta { info, id: _ } => match info {
                    edges::MetaInfo::SrcId | edges::MetaInfo::DstId => {
                        // already checked in check_config(...)
                    }
                    edges::MetaInfo::EdgeId
                    | edges::MetaInfo::SrcIdx
                    | edges::MetaInfo::SrcLat
                    | edges::MetaInfo::SrcLon
                    | edges::MetaInfo::DstIdx
                    | edges::MetaInfo::DstLat
                    | edges::MetaInfo::DstLon
                    | edges::MetaInfo::ShortcutIdx0
                    | edges::MetaInfo::ShortcutIdx1 => {
                        return Err(format!("{:?} are not supported in pbf-files.", category).into())
                    }
                },
                edges::Category::Metric { unit, id: _ } => match unit {
                    edges::metrics::UnitInfo::Meters
                    | edges::metrics::UnitInfo::Kilometers
                    | edges::metrics::UnitInfo::Seconds
                    | edges::metrics::UnitInfo::Minutes
                    | edges::metrics::UnitInfo::Hours
                    | edges::metrics::UnitInfo::F64 => {
                        return Err(format!(
                            "The {:?} of an edge in a pbf-file has to be calculated, \
                             but is expected to be provided.",
                            category
                        )
                        .into());
                    }
                    edges::metrics::UnitInfo::KilometersPerHour
                    | edges::metrics::UnitInfo::LaneCount => {
                        // irrelevant
                    }
                },
                edges::Category::Ignored => (),
            }
        }

        info!("FINISHED");
        Ok(())
    }

    fn parse_ways(&self, builder: &mut EdgeBuilder) -> err::Feedback {
        info!("START Create edges from input-file.");
        let file = OpenOptions::new()
            .read(true)
            .open(&builder.cfg().map_file)
            .expect(&format!(
                "Couldn't open {}",
                builder.cfg().map_file.display()
            ));

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
                // -> don't use last one, to not use it twice (a->b->c->c->b->a)
                0..(way.nodes.len() - 1)
            };
            let nodes: Vec<i64> = way
                .nodes
                .iter()
                .chain(way.nodes[iter_range].iter().rev())
                .map(|id| id.0)
                .collect();

            // Collect metrics as expected by user-config
            // ATTENTION: A way contains multiple edges, thus be careful when adding new metrics.

            let mut metrics: DimVec<_> = smallvec![];

            for category in builder.cfg().edges.categories.iter() {
                match category {
                    edges::Category::Meta { info: _, id: _ } => {
                        // already checked in preprocessing
                    }
                    edges::Category::Metric { unit, id: _ } => match unit {
                        edges::metrics::UnitInfo::KilometersPerHour => {
                            let maxspeed = highway_tag.parse_maxspeed(&way);
                            metrics.push(*maxspeed);
                        }
                        edges::metrics::UnitInfo::LaneCount => {
                            let lane_count = highway_tag.parse_lane_count(&way);
                            metrics.push(lane_count as f64);
                        }
                        edges::metrics::UnitInfo::Meters
                        | edges::metrics::UnitInfo::Kilometers
                        | edges::metrics::UnitInfo::Seconds
                        | edges::metrics::UnitInfo::Minutes
                        | edges::metrics::UnitInfo::Hours
                        | edges::metrics::UnitInfo::F64 => {
                            // already checked in preprocessing
                        }
                    },
                    edges::Category::Ignored => {
                        // already checked in preprocessing
                    }
                }
            }

            // for n nodes in a way, you can create (n-1) edges
            for node_idx in 0..(nodes.len() - 1) {
                // add proto-edge to graph
                builder.insert(ProtoEdge {
                    id: None,
                    src_id: nodes[node_idx],
                    dst_id: nodes[node_idx + 1],
                    metrics: metrics.clone(),
                })?;
            }
        }
        info!("FINISHED");
        Ok(())
    }

    fn parse_nodes(&self, builder: &mut NodeBuilder) -> err::Feedback {
        info!("START Create nodes from input-file.");
        let cfg = builder.cfg();

        let file = OpenOptions::new()
            .read(true)
            .open(&cfg.map_file)
            .expect(&format!("Couldn't open {}", cfg.map_file.display()));
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
                ch_level: None,
            });
        }
        info!("FINISHED");
        Ok(())
    }
}
