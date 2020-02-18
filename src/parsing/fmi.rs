use crate::{configs::graph, network::GraphBuilder};
use log::info;
use std::{fs::File, io::BufRead, ops::Range};

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
    fn preprocess(&mut self, file: File) -> Result<(), String> {
        info!("START Start preprocessing for fmi-parser.");
        // only functional-lines are counted
        let mut line_number = 0;
        let mut is_taking_counts = false;
        // counts are only metric-count, node-count, edge-count (in this order)
        let mut counts = vec![];
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
        file: File,
        graph_builder: &mut GraphBuilder,
        cfg: &graph::Config,
    ) -> Result<(), String> {
        info!("START Create edges from input-file.");
        let mut line_number = 0;
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
            let proto_edge = intern::ProtoEdge::from_str(&line, &cfg.edges)?;
            graph_builder.push_edge(
                proto_edge
                    .src_id
                    .expect("Src-id should already have been tested."),
                proto_edge
                    .dst_id
                    .expect("Dst-id should already have been tested."),
                proto_edge.meters,
                proto_edge.maxspeed,
                proto_edge.duration,
                proto_edge.lane_count,
                proto_edge.metric_u32,
            );
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
        let mut line_number = 0;
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
            let proto_node = line.parse::<intern::ProtoNode>()?;
            graph_builder.push_node(proto_node.id, proto_node.coord);
        }
        info!("FINISHED");

        Ok(())
    }
}

mod intern {
    use crate::{
        configs::{edges, MetricType},
        units::{
            geo, length::Meters, speed::KilometersPerHour, time::Milliseconds, MetricU32, MetricU8,
        },
    };
    pub use std::{io::BufReader as Reader, str};

    pub struct ProtoNode {
        pub id: i64,
        pub coord: geo::Coordinate,
    }

    impl str::FromStr for ProtoNode {
        type Err = String;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let params: Vec<&str> = line.split_whitespace().collect();

            let n = 4;
            if params.len() < n {
                return Err(format!(
                    "Not enough params for a node ({}, but should be {}).",
                    params.len(),
                    n
                ));
            }

            let id = match params[0].parse::<i64>() {
                Ok(id) => id,
                Err(_) => {
                    return Err(format!(
                        "Parsing id '{:?}' from fmi-file, which is not i64.",
                        params[0]
                    ))
                }
            };
            let lat = match params[2].parse::<f64>() {
                Ok(lat) => lat,
                Err(_) => {
                    return Err(format!(
                        "Parsing lat '{:?}' from fmi-file, which is not f64.",
                        params[2]
                    ))
                }
            };
            let lon = match params[3].parse::<f64>() {
                Ok(lon) => lon,
                Err(_) => {
                    return Err(format!(
                        "Parsing lon '{:?}' from fmi-file, which is not f64.",
                        params[3]
                    ))
                }
            };

            Ok(ProtoNode {
                id,
                coord: geo::Coordinate::from_f64(lat, lon),
            })
        }
    }

    //--------------------------------------------------------------------------------------------//

    pub struct ProtoEdge {
        pub src_id: Option<i64>,
        pub dst_id: Option<i64>,
        pub meters: Option<Meters>,
        pub maxspeed: Option<KilometersPerHour>,
        pub duration: Option<Milliseconds>,
        pub lane_count: Option<MetricU8>,
        pub metric_u32: Option<MetricU32>,
    }

    impl ProtoEdge {
        fn new_empty() -> ProtoEdge {
            ProtoEdge {
                src_id: None,
                dst_id: None,
                meters: None,
                maxspeed: None,
                duration: None,
                lane_count: None,
                metric_u32: None,
            }
        }

        /// Parse a line of metrics into an edge.
        ///
        /// - When NodeIds are parsed, the first one is interpreted as src-id and the second one as dst-id.
        pub fn from_str(line: &str, cfg: &edges::Config) -> Result<ProtoEdge, String> {
            let params: Vec<&str> = line.split_whitespace().collect();

            let mut proto_edge = ProtoEdge::new_empty();

            // Loop over metric-types and parse params accordingly.
            // If a metric will be calculated, don't inc param-idx.
            let mut param_idx = 0;
            for idx in 0..cfg.metric_types.len() {
                let metric_type = cfg.metric_types[idx];
                let param = params.get(param_idx).ok_or(
                    "The fmi-map-file is expected to have more edge-params than actually has.",
                )?;

                match metric_type {
                    MetricType::Id => {
                        if proto_edge.src_id.is_none() {
                            proto_edge.src_id = Some(param.parse::<i64>().ok().ok_or(format!(
                                "Parsing {} (for edge-src) '{:?}' from fmi-file, which is not i64.",
                                MetricType::Id,
                                param
                            ))?);
                        } else if proto_edge.dst_id.is_none() {
                            proto_edge.dst_id = Some(param.parse::<i64>().ok().ok_or(format!(
                                "Parsing {} (for edge-dst) '{:?}' from fmi-file, which is not i64.",
                                MetricType::Id,
                                param
                            ))?);
                        } else {
                            return Err(format!(
                                "Both src-id and dst-id are already set, \
                                 but another {} should be parsed.",
                                MetricType::Id
                            ));
                        }

                        // param used
                        param_idx += 1;
                    }
                    MetricType::Length { provided } => {
                        if provided {
                            let meters = param.parse::<u32>().ok().ok_or(format!(
                                "Parsing {} '{}' of edge-param #{} didn't work.",
                                MetricType::Length { provided },
                                param,
                                param_idx
                            ))?;
                            proto_edge.meters = Some(Meters::new(meters));

                            // param used
                            param_idx += 1;
                        }
                    }
                    MetricType::Maxspeed { provided } => {
                        if provided {
                            let maxspeed = param.parse::<u16>().ok().ok_or(format!(
                                "Parsing {} '{}' of edge-param #{} didn't work.",
                                MetricType::Maxspeed { provided },
                                param,
                                param_idx
                            ))?;
                            proto_edge.maxspeed = Some(KilometersPerHour::new(maxspeed));

                            // param used
                            param_idx += 1;
                        }
                    }
                    MetricType::Duration { provided } => {
                        if provided {
                            let duration = param.parse::<u32>().ok().ok_or(format!(
                                "Parsing {} '{}' of edge-param #{} didn't work.",
                                MetricType::Duration { provided },
                                param,
                                param_idx
                            ))?;
                            proto_edge.duration = Some(Milliseconds::new(duration));

                            // param used
                            param_idx += 1;
                        }
                    }
                    MetricType::LaneCount => {
                        let lane_count = param.parse::<u8>().ok().ok_or(format!(
                            "Parsing {} '{}' of edge-param #{} didn't work.",
                            MetricType::LaneCount,
                            param,
                            param_idx
                        ))?;
                        proto_edge.lane_count = Some(MetricU8::new(lane_count));

                        // param used
                        param_idx += 1;
                    }
                    MetricType::Custom => {
                        let metric_u32 = param.parse::<u32>().ok().ok_or(format!(
                            "Parsing {} '{}' of edge-param #{} didn't work.",
                            MetricType::Custom,
                            param,
                            param_idx
                        ))?;
                        proto_edge.metric_u32 = Some(MetricU32::new(metric_u32));

                        // param used
                        param_idx += 1;
                    }
                    MetricType::Ignore => {
                        // param used
                        param_idx += 1;
                    }
                }
            }

            Ok(proto_edge)
        }
    }
}
