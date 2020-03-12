use crate::{
    configs::parser,
    helpers,
    network::{GraphBuilder, ProtoEdge, ProtoNode},
};
use log::info;
use std::{io::BufRead, ops::Range};

pub struct Parser {
    node_lines: Range<usize>,
    edge_lines: Range<usize>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            node_lines: 1..0,
            edge_lines: 1..0,
        }
    }

    fn is_line_functional(line: &String) -> bool {
        line != "" && line.chars().next() != Some('#')
    }
}

impl super::Parsing for Parser {
    /// Remembers range of edge-lines and node-lines
    fn preprocess(&mut self, cfg: &parser::Config) -> Result<(), String> {
        info!("START Start preprocessing fmi-parser.");
        super::check_parser_config(cfg)?;

        // only functional-lines are counted
        let mut line_number = 0;
        let mut is_taking_counts = false;
        // counts are only metric-count, node-count, edge-count (in this order)
        let mut counts = vec![];
        let file = helpers::open_file(&cfg.map_file)?;
        for line in intern::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            // If there is a count, remember it.
            // The first occuring count let `is_taking_counts` getting true.
            // If all counts have been processed, `is_taking_counts` would change to false,
            // where the loop should stop and remember the line-number.
            let params: Vec<&str> = line.split_whitespace().collect();
            if params.len() == 1 {
                is_taking_counts = true;

                if let Ok(count) = params[0].parse::<usize>() {
                    counts.push(count);
                }
            } else if is_taking_counts {
                break;
            }

            line_number += 1;
        }

        // add counts
        if counts.len() < 2 {
            return Err(format!(
                "The provided fmi-map-file doesn't have enough (edge-, node-) counts."
            ));
        }

        // Current state: Last line-number is first node-line.
        // Further, the last two counts are the node- and edge-counts.
        let edge_count = counts.pop().unwrap();
        let node_count = counts.pop().unwrap();

        // nodes
        let start = line_number;
        let end = start + node_count;
        self.node_lines = start..end;

        // edges
        let start = end;
        let end = start + edge_count;
        self.edge_lines = start..end;

        info!("FINISHED");

        Ok(())
    }

    fn parse_ways(
        &self,
        cfg: &parser::Config,
        graph_builder: &mut GraphBuilder,
    ) -> Result<(), String> {
        info!("START Create edges from input-file.");
        let mut line_number = 0;
        let file = helpers::open_file(&cfg.map_file)?;
        for line in intern::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            // check if line contains edge
            if !self.edge_lines.contains(&line_number) {
                line_number += 1;
                continue;
            }
            line_number += 1;

            // create edge and add it
            let proto_edge = ProtoEdge::from_str(&line, &cfg.edges)?;
            graph_builder.push_edge(proto_edge);
        }
        info!("FINISHED");

        Ok(())
    }

    fn parse_nodes(
        &self,
        cfg: &parser::Config,
        graph_builder: &mut GraphBuilder,
    ) -> Result<(), String> {
        info!("START Create nodes from input-file.");
        let mut line_number = 0;
        let file = helpers::open_file(&cfg.map_file)?;
        for line in intern::Reader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            // check if line contains edge
            if !self.node_lines.contains(&line_number) {
                line_number += 1;
                continue;
            }
            line_number += 1;

            // create node and add it
            let proto_node = ProtoNode::from_str(&line, &cfg.nodes)?;
            if graph_builder.is_node_in_edge(proto_node.id) {
                graph_builder.push_node(proto_node);
            }
        }
        info!("FINISHED");

        Ok(())
    }
}

mod intern {
    use crate::{
        configs::{parser, EdgeCategory, NodeCategory},
        defaults::DimVec,
        network::{MetricIdx, ProtoEdge, ProtoNode},
        units::geo,
    };
    pub use std::{io::BufReader as Reader, str};

    impl ProtoEdge {
        /// Parse a line of metrics into an edge.
        ///
        /// - When NodeIds are parsed, the first one is interpreted as src-id and the second one as dst-id.
        pub fn from_str(line: &str, cfg: &parser::edges::Config) -> Result<ProtoEdge, String> {
            let mut metric_values = DimVec::<_>::with_capacity(cfg.dim());
            let mut src_id = None;
            let mut dst_id = None;

            // Loop over edge-categories and parse params accordingly.
            let params: Vec<&str> = line.split_whitespace().collect();

            // Param-idx has to be counted separatedly because some metrics could be calculated.
            let mut param_idx = 0;
            for category in cfg.edge_categories().iter() {
                let param = *params.get(param_idx).ok_or(&format!(
                    "The fmi-map-file is expected to have more edge-params (> {}) \
                     than actually has ({}).",
                    param_idx,
                    params.len()
                ))?;

                match category {
                    EdgeCategory::SrcId => {
                        if src_id.is_none() {
                            src_id = Some(param.parse::<i64>().ok().ok_or(format!(
                                "Parsing {} (for edge-src) '{:?}' from fmi-file, which is not i64.",
                                category, param
                            ))?);
                            param_idx += 1;
                        } else {
                            return Err(format!(
                                "Src-id is already set, but another src-id {} should be parsed.",
                                param
                            ));
                        }
                    }
                    EdgeCategory::DstId => {
                        if dst_id.is_none() {
                            dst_id = Some(param.parse::<i64>().ok().ok_or(format!(
                                "Parsing {} (for edge-src) '{:?}' from fmi-file, which is not i64.",
                                category, param
                            ))?);
                            param_idx += 1;
                        } else {
                            return Err(format!(
                                "Dst-id is already set, but another dst-id {} should be parsed.",
                                param
                            ));
                        }
                    }
                    EdgeCategory::Meters => {
                        let metric_idx = MetricIdx(metric_values.len());

                        if cfg.is_metric_provided(metric_idx) {
                            if let Ok(meters) = param.parse::<f32>() {
                                metric_values.push(Some(meters / 1_000.0));
                            } else {
                                return Err(format!(
                                    "Parsing {} '{}' of edge-param #{} didn't work.",
                                    category, param, param_idx
                                ));
                            };
                            param_idx += 1;
                        } else {
                            metric_values.push(None);
                        }
                    }
                    EdgeCategory::KilometersPerHour
                    | EdgeCategory::Seconds
                    | EdgeCategory::LaneCount
                    | EdgeCategory::Custom => {
                        let metric_idx = MetricIdx(metric_values.len());

                        if cfg.is_metric_provided(metric_idx) {
                            if let Ok(value) = param.parse::<f32>() {
                                metric_values.push(Some(value));
                            } else {
                                return Err(format!(
                                    "Parsing {} '{}' of edge-param #{} didn't work.",
                                    category, param, param_idx
                                ));
                            };
                            param_idx += 1;
                        } else {
                            metric_values.push(None);
                        }
                    }
                    EdgeCategory::Ignore => param_idx += 1,
                }
            }

            debug_assert_eq!(
                cfg.dim(),
                metric_values.len(),
                "Metric-vec of proto-edge has {} elements, but should have {}.",
                metric_values.len(),
                cfg.dim()
            );
            Ok(ProtoEdge {
                src_id: src_id.ok_or("Proto-edge should have a src-id, but doesn't.".to_owned())?,
                dst_id: dst_id.ok_or("Proto-edge should have a dst-id, but doesn't.".to_owned())?,
                metrics: metric_values,
            })
        }
    }

    impl ProtoNode {
        pub fn from_str(line: &str, cfg: &parser::nodes::Config) -> Result<ProtoNode, String> {
            let mut node_id = None;
            let mut lat = None;
            let mut lon = None;
            let mut level = None;

            // Loop over node-categories and parse params accordingly.
            let params: Vec<&str> = line.split_whitespace().collect();

            for (param_idx, category) in cfg.categories().iter().enumerate() {
                let param = *params.get(param_idx).ok_or(
                    "The fmi-map-file is expected to have more node-params \
                     than actually has.",
                )?;

                match category {
                    NodeCategory::NodeId => {
                        node_id = match param.parse::<i64>() {
                            Ok(id) => Some(id),
                            Err(_) => {
                                return Err(format!(
                                    "Parsing id '{:?}' from fmi-file, which is not i64.",
                                    param
                                ))
                            }
                        };
                    }
                    NodeCategory::Latitude => {
                        lat = match param.parse::<f32>() {
                            Ok(lat) => Some(lat),
                            Err(_) => {
                                return Err(format!(
                                    "Parsing lat '{:?}' from fmi-file, which is not f32.",
                                    params[2]
                                ))
                            }
                        };
                    }
                    NodeCategory::Longitude => {
                        lon = match param.parse::<f32>() {
                            Ok(lon) => Some(lon),
                            Err(_) => {
                                return Err(format!(
                                    "Parsing lon '{:?}' from fmi-file, which is not f32.",
                                    params[3]
                                ))
                            }
                        };
                    }
                    NodeCategory::Level => {
                        level = match param.parse::<usize>() {
                            Ok(level) => Some(level),
                            Err(_) => {
                                return Err(format!(
                                    "Parsing level '{:?}' from fmi-file, which is not usize.",
                                    param
                                ))
                            }
                        };
                    }
                    NodeCategory::NodeIdx | NodeCategory::Ignore => (),
                }
            }

            let node_id = node_id.ok_or("Proto-node should have an id, but doesn't.".to_owned())?;
            let lat = lat.ok_or("Proto-node should have a coordinate, but latitude is misisng.")?;
            let lon =
                lon.ok_or("Proto-node should have a coordinate, but longitude is misisng.")?;
            Ok(ProtoNode::new(
                node_id,
                Some(geo::Coordinate { lat, lon }),
                level,
            ))
        }
    }
}
