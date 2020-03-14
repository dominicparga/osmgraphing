use crate::{
    configs::generator,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;

pub struct Generator;

impl Generator {
    pub fn generate(graph: &Graph, cfg_generator: &generator::Config) -> Result<(), String> {
        info!("START Generate file from graph");
        match Generator::from_path(&cfg_generator.map_file)? {
            MapFileExt::FMI => {
                fmi::Generator::new().generate(graph, cfg_generator)?;
                info!("FINISHED");
                Ok(())
            }
            MapFileExt::PBF => Err(String::from("No support for generating pbf-files.")),
        }
    }
}

impl SupportingMapFileExts for Generator {}
impl SupportingFileExts for Generator {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["fmi"]
    }
}

trait Generating {
    fn generate(&self, graph: &Graph, cfg_generator: &generator::Config) -> Result<(), String>;
}

pub mod fmi {
    use crate::{
        configs::{
            generator::{self, EdgeCategory, NodeCategory},
            parser,
        },
        defaults::accuracy,
        helpers::{self, Approx},
        network::{EdgeIdx, Graph, NodeIdx},
    };
    use log::info;
    use progressing::{self, Bar};
    use std::io::{BufWriter, Write};

    pub struct Generator;

    impl Generator {
        pub fn new() -> Generator {
            Generator {}
        }
    }

    impl super::Generating for Generator {
        fn generate(&self, graph: &Graph, cfg_generator: &generator::Config) -> Result<(), String> {
            fn inner_generate(
                graph: &Graph,
                cfg_generator: &generator::Config,
            ) -> Result<(), Box<dyn std::error::Error>> {
                //--------------------------------------------------------------------------------//
                // prepare

                let output_file = helpers::open_new_file(&cfg_generator.map_file)?;
                let mut writer = BufWriter::new(output_file);
                let ignore_str = "_";

                //--------------------------------------------------------------------------------//
                // write header

                writeln!(writer, "# edge-metric-count")?;
                writeln!(writer, "# node-count")?;
                writeln!(writer, "# edge-count")?;
                writeln!(writer, "# nodes: {:?}", cfg_generator.nodes)?;
                let edge_component_ids: Vec<_> = cfg_generator
                    .edges
                    .iter()
                    // .map(|id| graph.cfg().edges.edge_category(id))
                    .map(|id| &id.0)
                    .collect();
                writeln!(writer, "# edges: {:?}", edge_component_ids)?;

                writeln!(writer, "")?;

                //--------------------------------------------------------------------------------//
                // write counts

                let dim = cfg_generator
                    .edges
                    .iter()
                    .map(|id| graph.cfg().edges.edge_category(id))
                    .filter(|parser_category| parser_category.is_metric())
                    .count();
                let node_count = graph.nodes().count();
                let edge_count = graph.fwd_edges().count();
                writeln!(writer, "{}", dim)?;
                writeln!(writer, "{}", node_count)?;
                writeln!(writer, "{}", edge_count)?;

                //--------------------------------------------------------------------------------//
                // write nodes

                let mut progress_bar =
                    progressing::BernoulliBar::from_goal((node_count + edge_count) as u32);
                info!("{}", progress_bar);
                for node_idx in (0..node_count).into_iter().map(NodeIdx) {
                    let node = graph.nodes().create(node_idx);

                    for (idx, category) in cfg_generator.nodes.iter().enumerate() {
                        // write node-info
                        match category {
                            NodeCategory::NodeId => write!(writer, "{}", node.id())?,
                            NodeCategory::NodeIdx => write!(writer, "{}", node.idx())?,
                            NodeCategory::Latitude => write!(
                                writer,
                                "{:.digits$}",
                                node.coord().lat.approx(),
                                digits = accuracy::F64_FMT_DIGITS,
                            )?,
                            NodeCategory::Longitude => write!(
                                writer,
                                "{:.digits$}",
                                node.coord().lon.approx(),
                                digits = accuracy::F64_FMT_DIGITS,
                            )?,
                            NodeCategory::Level => write!(writer, "{}", node.level())?,
                            NodeCategory::Ignore => write!(writer, "{}", ignore_str)?,
                        }
                        // write space if needed
                        if idx < cfg_generator.nodes.len() - 1 {
                            write!(writer, " ")?;
                        }
                    }
                    // write end of line
                    writeln!(writer, "")?;

                    // print progress
                    progress_bar.add((1, 1));
                    if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                        info!("{}", progress_bar);
                    }
                }

                //--------------------------------------------------------------------------------//
                // write edges

                for edge_idx in (0..edge_count).into_iter().map(EdgeIdx) {
                    for (idx, id) in cfg_generator.edges.iter().enumerate() {
                        // get category from id, needing a conversion
                        // from parser-category to generator-category
                        let category = {
                            let parser_category = graph.cfg().edges.edge_category(id);

                            match parser_category {
                                parser::EdgeCategory::Meters => EdgeCategory::Meters,
                                parser::EdgeCategory::KilometersPerHour => {
                                    EdgeCategory::KilometersPerHour
                                }
                                parser::EdgeCategory::Seconds => EdgeCategory::Seconds,
                                parser::EdgeCategory::LaneCount => EdgeCategory::LaneCount,
                                parser::EdgeCategory::Custom => EdgeCategory::Custom,
                                parser::EdgeCategory::SrcId => EdgeCategory::SrcId,
                                parser::EdgeCategory::IgnoredSrcIdx => EdgeCategory::SrcIdx,
                                parser::EdgeCategory::DstId => EdgeCategory::DstId,
                                parser::EdgeCategory::IgnoredDstIdx => EdgeCategory::DstIdx,
                                parser::EdgeCategory::Ignore => EdgeCategory::Ignore,
                            }
                        };

                        // write edge-info
                        match category {
                            EdgeCategory::Meters => {
                                let metric_idx = graph.cfg().edges.metric_idx(id);
                                let km = graph.metrics().get(metric_idx, edge_idx);
                                let m = km * 1_000.0;
                                write!(writer, "{:.digits$}", m, digits = accuracy::F64_FMT_DIGITS,)?
                            }
                            EdgeCategory::KilometersPerHour
                            | EdgeCategory::Seconds
                            | EdgeCategory::LaneCount
                            | EdgeCategory::Custom => {
                                let metric_idx = graph.cfg().edges.metric_idx(id);
                                write!(
                                    writer,
                                    "{:.digits$}",
                                    graph.metrics().get(metric_idx, edge_idx),
                                    digits = accuracy::F64_FMT_DIGITS,
                                )?
                            }
                            EdgeCategory::SrcId => {
                                let src_idx = graph.bwd_edges().dst_idx(edge_idx);
                                let src_id = graph.nodes().id(src_idx);
                                write!(writer, "{}", src_id)?;
                            }
                            EdgeCategory::SrcIdx => {
                                let src_idx = graph.bwd_edges().dst_idx(edge_idx);
                                write!(writer, "{}", src_idx)?;
                            }
                            EdgeCategory::DstId => {
                                let dst_idx = graph.fwd_edges().dst_idx(edge_idx);
                                let dst_id = graph.nodes().id(dst_idx);
                                write!(writer, "{}", dst_id)?;
                            }
                            EdgeCategory::DstIdx => {
                                let dst_idx = graph.fwd_edges().dst_idx(edge_idx);
                                write!(writer, "{}", dst_idx)?;
                            }
                            EdgeCategory::Ignore => write!(writer, "{}", ignore_str)?,
                        }
                        // write space if needed
                        if idx < cfg_generator.edges.len() - 1 {
                            write!(writer, " ")?;
                        }
                    }
                    // write end of line
                    writeln!(writer, "")?;

                    // print progress
                    progress_bar.add((1, 1));
                    if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                        info!("{}", progress_bar);
                    }
                }
                info!("{}", progress_bar);

                //--------------------------------------------------------------------------------//

                Ok(())
            }

            //------------------------------------------------------------------------------------//
            // return result

            if let Err(e) = inner_generate(graph, cfg_generator) {
                Err(format!("{}", e))
            } else {
                Ok(())
            }
        }
    }
}
