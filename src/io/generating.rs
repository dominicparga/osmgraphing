use crate::{
    configs::generator,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;

pub struct Generator;

impl Generator {
    pub fn generate(graph: &Graph, cfg: &generator::Config) -> Result<(), String> {
        info!("START Generate file from graph");
        match Generator::from_path(&cfg.map_file)? {
            MapFileExt::FMI => {
                fmi::Generator::new().generate(graph, cfg)?;
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
    fn generate(&self, graph: &Graph, cfg: &generator::Config) -> Result<(), String>;
}

pub mod fmi {
    use crate::{
        configs::{generator, EdgeCategory, NodeCategory},
        helpers,
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
        fn generate(&self, graph: &Graph, cfg: &generator::Config) -> Result<(), String> {
            fn inner_generate(
                graph: &Graph,
                cfg: &generator::Config,
            ) -> Result<(), Box<dyn std::error::Error>> {
                let output_file = helpers::open_new_file(&cfg.map_file)?;
                let mut writer = BufWriter::new(output_file);

                //--------------------------------------------------------------------------------//
                // write header

                writeln!(writer, "# node-count")?;
                writeln!(writer, "# edge-count")?;
                writeln!(writer, "# nodes: {:?}", cfg.nodes)?;
                let edge_component_ids: Vec<_> = cfg
                    .edges
                    .iter()
                    .map(|id| graph.cfg().edges.edge_category(id))
                    .collect();
                writeln!(writer, "# edges: {:?}", edge_component_ids)?;

                writeln!(writer, "")?;

                //--------------------------------------------------------------------------------//
                // write counts

                let node_count = graph.nodes().count();
                let edge_count = graph.fwd_edges().count();
                writeln!(writer, "{}", node_count)?;
                writeln!(writer, "{}", edge_count)?;

                //--------------------------------------------------------------------------------//
                // write nodes

                let mut progress_bar =
                    progressing::BernoulliBar::from_goal((node_count + edge_count) as u32);
                info!("{}", progress_bar);
                for node_idx in (0..node_count).into_iter().map(NodeIdx) {
                    let node = graph.nodes().create(node_idx);

                    for (idx, category) in cfg.nodes.iter().enumerate() {
                        // write node-info
                        match category {
                            NodeCategory::NodeId => write!(writer, "{}", node.id())?,
                            NodeCategory::NodeIdx => write!(writer, "{}", node.idx())?,
                            NodeCategory::Latitude => write!(writer, "{}", node.coord().lat)?,
                            NodeCategory::Longitude => write!(writer, "{}", node.coord().lon)?,
                            NodeCategory::Level => write!(writer, "{}", node.level())?,
                            NodeCategory::Ignore => write!(writer, "0")?,
                        }
                        // write space if needed
                        if idx < cfg.nodes.len() - 1 {
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
                    // src-id
                    let src_idx = graph.bwd_edges().dst_idx(edge_idx);
                    let src_id = graph.nodes().id(src_idx);
                    // dst-id
                    let dst_idx = graph.fwd_edges().dst_idx(edge_idx);
                    let dst_id = graph.nodes().id(dst_idx);

                    for (idx, id) in cfg.edges.iter().enumerate() {
                        let category = graph.cfg().edges.edge_category(id);

                        // write edge-info
                        match category {
                            EdgeCategory::Meters
                            | EdgeCategory::KilometersPerHour
                            | EdgeCategory::Seconds
                            | EdgeCategory::LaneCount
                            | EdgeCategory::Custom => {
                                let metric_idx = graph.cfg().edges.metric_idx(id);
                                write!(writer, "{}", graph.metrics().get(metric_idx, edge_idx))?
                            }
                            EdgeCategory::SrcId => write!(writer, "{}", src_id)?,
                            EdgeCategory::DstId => write!(writer, "{}", dst_id)?,
                            EdgeCategory::Ignore => write!(writer, "0")?,
                        }
                        // write space if needed
                        if idx < cfg.edges.len() - 1 {
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

            if let Err(e) = inner_generate(graph, cfg) {
                Err(format!("{}", e))
            } else {
                Ok(())
            }
        }
    }
}
