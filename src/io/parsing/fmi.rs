use crate::{
    configs::{
        parsing::edges,
        parsing::{self, nodes},
    },
    defaults::{self, capacity::DimVec},
    helpers,
    network::{EdgeBuilder, EdgeIdx, NodeBuilder, ProtoEdge, ProtoNode, ProtoShortcut},
};
use kissunits::geo;
use log::info;
use std::{
    io::{BufRead, BufReader},
    ops::Range,
};

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
        line.len() > 0 && line.chars().next() != Some('#')
    }
}

impl super::Parsing for Parser {
    /// Remembers range of edge-lines and node-lines
    fn preprocess(&mut self, cfg: &parsing::Config) -> Result<(), String> {
        info!("START Start preprocessing fmi-parser.");
        super::check_config(cfg)?;

        // only functional-lines are counted
        let mut line_number = 0;
        let mut is_taking_counts = false;
        // counts are only metric-count, node-count, edge-count (in this order)
        let mut counts = vec![];
        let file = helpers::open_file(&cfg.map_file)?;
        for line in BufReader::new(file)
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

    fn parse_ways(&self, builder: &mut EdgeBuilder) -> Result<(), String> {
        info!("START Create edges from input-file.");
        let mut line_number = 0;
        let file = helpers::open_file(&builder.cfg().map_file)?;
        for line in BufReader::new(file)
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
            let proto_edge = ProtoShortcut::from_str(&line, &builder.cfg().edges)?;
            builder.insert(proto_edge);
        }
        info!("FINISHED");

        Ok(())
    }

    fn parse_nodes(&self, builder: &mut NodeBuilder) -> Result<(), String> {
        info!("START Create nodes from input-file.");
        let mut line_number = 0;
        let file = helpers::open_file(&builder.cfg().map_file)?;
        for line in BufReader::new(file)
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
            let proto_node = ProtoNode::from_str(&line, &builder.cfg().nodes)?;
            builder.insert(proto_node);
        }
        info!("FINISHED");

        Ok(())
    }
}

impl ProtoShortcut {
    /// Parse a line of metrics into an edge.
    ///
    /// - When NodeIds are parsed, the first one is interpreted as src-id and the second one as dst-id.
    pub fn from_str(line: &str, cfg: &parsing::edges::Config) -> Result<ProtoShortcut, String> {
        let mut metric_values = DimVec::new();
        let mut src_id = None;
        let mut dst_id = None;
        let mut sc_edge_0 = None;
        let mut sc_edge_1 = None;

        // Loop over edge-categories and parse params accordingly.
        let params: Vec<&str> = line.split_whitespace().collect();

        // Param-idx has to be counted separatedly because some metrics could be calculated.
        for (param_idx, category) in cfg.categories.iter().enumerate() {
            let param = *params.get(param_idx).ok_or(&format!(
                "The fmi-map-file is expected to have more edge-params (> {}) \
                 than actually has ({}).",
                param_idx,
                params.len()
            ))?;

            match category {
                edges::Category::Meta { info, id: _ } => match info {
                    edges::MetaInfo::SrcId => {
                        if src_id.is_none() {
                            src_id = Some(param.parse::<i64>().ok().ok_or(format!(
                                "Parsing {:?} (for edge-src) '{:?}' from fmi-file, which is not i64.",
                                category, param
                            ))?);
                        } else {
                            return Err(format!(
                                "Src-id is already set, but another src-id {} should be parsed.",
                                param
                            ));
                        }
                    }
                    edges::MetaInfo::DstId => {
                        if dst_id.is_none() {
                            dst_id = Some(param.parse::<i64>().ok().ok_or(format!(
                                "Parsing {:?} (for edge-dst) '{:?}' from fmi-file, which is not i64.",
                                category, param
                            ))?);
                        } else {
                            return Err(format!(
                                "Dst-id is already set, but another dst-id {} should be parsed.",
                                param
                            ));
                        }
                    }
                    edges::MetaInfo::ShortcutIdx0 => {
                        if param != defaults::parser::NO_SHORTCUT_IDX {
                            let sc_edge_idx = {
                                param.parse::<usize>().ok().ok_or(format!(
                                    "Parsing {:?} '{}' of edge-param #{} didn't work.",
                                    category, param, param_idx
                                ))?
                            };

                            if sc_edge_0.is_none() {
                                sc_edge_0 = Some(sc_edge_idx);
                            } else {
                                return Err(format!(
                                    "Too many {:?}: parsing '{}' of edge-param #{}",
                                    category, param, param_idx
                                ));
                            }
                        }
                    }
                    edges::MetaInfo::ShortcutIdx1 => {
                        if param != defaults::parser::NO_SHORTCUT_IDX {
                            let sc_edge_idx = {
                                param.parse::<usize>().ok().ok_or(format!(
                                    "Parsing {:?} '{}' of edge-param #{} didn't work.",
                                    category, param, param_idx
                                ))?
                            };

                            if sc_edge_1.is_none() {
                                sc_edge_1 = Some(sc_edge_idx);
                            } else {
                                return Err(format!(
                                    "Too many {:?}: parsing '{}' of edge-param #{}",
                                    category, param, param_idx
                                ));
                            }
                        }
                    }
                    edges::MetaInfo::SrcIdx | edges::MetaInfo::DstIdx => {
                        return Err(format!("Unsupported category {:?}", category))
                    }
                },
                edges::Category::Metric { unit: _, id: _ } => {
                    if let Ok(raw_value) = param.parse::<f64>() {
                        metric_values.push(raw_value);
                    } else {
                        return Err(format!(
                            "Parsing {:?} '{}' of edge-param #{} didn't work.",
                            category, param, param_idx
                        ));
                    };
                }
                edges::Category::Ignored => (),
            }
        }

        debug_assert_eq!(
            cfg.metrics.units.len(),
            metric_values.len(),
            "Metric-vec of proto-edge has {} elements, but should have {}.",
            metric_values.len(),
            cfg.metrics.units.len()
        );

        let sc_edges = {
            if sc_edge_0.is_none() && sc_edge_1.is_none() {
                None
            } else {
                Some([EdgeIdx(sc_edge_0.unwrap()), EdgeIdx(sc_edge_1.unwrap())])
            }
        };

        Ok(ProtoShortcut {
            proto_edge: ProtoEdge {
                src_id: src_id.ok_or("Proto-edge should have a src-id, but doesn't.".to_owned())?,
                dst_id: dst_id.ok_or("Proto-edge should have a dst-id, but doesn't.".to_owned())?,
                metrics: metric_values,
            },
            sc_edges,
        })
    }
}

impl ProtoNode {
    pub fn from_str(line: &str, cfg: &parsing::nodes::Config) -> Result<ProtoNode, String> {
        let mut node_id = None;
        let mut lat = None;
        let mut lon = None;
        let mut level = None;

        // Loop over node-categories and parse params accordingly.
        let params: Vec<&str> = line.split_whitespace().collect();

        for (param_idx, category) in cfg.categories.iter().enumerate() {
            let param = *params.get(param_idx).ok_or(
                "The fmi-map-file is expected to have more node-params \
                 than actually has.",
            )?;

            match category {
                nodes::Category::Meta { info, id: _ } => match info {
                    nodes::MetaInfo::NodeId => {
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
                    nodes::MetaInfo::Level => {
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
                    nodes::MetaInfo::NodeIdx => {
                        return Err(format!("Unsupported category {:?}", category))
                    }
                },
                nodes::Category::Metric { unit, id: _ } => match unit {
                    nodes::metrics::UnitInfo::Latitude => {
                        lat = match param.parse::<f64>() {
                            Ok(lat) => Some(lat),
                            Err(_) => {
                                return Err(format!(
                                    "Parsing lat '{:?}' from fmi-file, which is not f64.",
                                    params[2]
                                ))
                            }
                        };
                    }
                    nodes::metrics::UnitInfo::Longitude => {
                        lon = match param.parse::<f64>() {
                            Ok(lon) => Some(lon),
                            Err(_) => {
                                return Err(format!(
                                    "Parsing lon '{:?}' from fmi-file, which is not f64.",
                                    params[3]
                                ))
                            }
                        };
                    }
                    nodes::metrics::UnitInfo::Height => {
                        return Err(format!("Unsupported category {:?}", category))
                    }
                },
                nodes::Category::Ignored => (),
            }
        }

        let node_id = node_id.ok_or("Proto-node should have an id, but doesn't.".to_owned())?;
        let lat = lat.ok_or("Proto-node should have a coordinate, but latitude is misisng.")?;
        let lon = lon.ok_or("Proto-node should have a coordinate, but longitude is misisng.")?;
        Ok(ProtoNode {
            id: node_id,
            coord: geo::Coordinate { lat, lon },
            level,
        })
    }
}
