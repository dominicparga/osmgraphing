use crate::{
    configs::{
        parsing::{edges, nodes},
        writing,
    },
    defaults::{self, accuracy},
    helpers::approx::Approx,
    network::{EdgeIdx, Graph, NodeIdx},
};
use log::info;
use progressing::{self, Bar};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Writer;

impl Writer {
    pub fn new() -> Writer {
        Writer {}
    }
}

impl super::Writing for Writer {
    fn write(&self, graph: &Graph, writing_cfg: &writing::network::Config) -> Result<(), String> {
        fn inner_write(
            graph: &Graph,
            writing_cfg: &writing::network::Config,
        ) -> Result<(), Box<dyn std::error::Error>> {
            // prepare

            let output_file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&writing_cfg.map_file)?;
            let mut writer = BufWriter::new(output_file);

            let fwd_edges = graph.fwd_edges();
            let bwd_edges = graph.bwd_edges();
            let nodes = graph.nodes();

            // write header

            writeln!(writer, "# edge-metric-count")?;
            writeln!(writer, "# node-count")?;
            writeln!(writer, "# edge-count")?;
            writeln!(
                writer,
                "# nodes: {:?}",
                writing_cfg
                    .nodes
                    .ids
                    .iter()
                    .map(|id| match id {
                        Some(id) => format!("{}", id.0),
                        None => format!("{}", defaults::writing::IGNORE_STR),
                    })
                    .collect::<Vec<_>>()
            )?;
            writeln!(
                writer,
                "# edges: {:?}",
                writing_cfg
                    .edges
                    .ids
                    .iter()
                    .map(|id| match id {
                        Some(id) => format!("{}", id.0),
                        None => format!("{}", defaults::writing::IGNORE_STR),
                    })
                    .collect::<Vec<_>>()
            )?;

            writeln!(writer, "")?;

            // write counts

            let dim = writing_cfg
                .edges
                .ids
                .iter()
                .filter_map(|id| id.as_ref())
                .filter(|id| graph.cfg().edges.metrics.ids.contains(id))
                .count();
            let node_count = nodes.count();
            let edge_count = fwd_edges.count();
            writeln!(writer, "{}", dim)?;
            writeln!(writer, "{}", node_count)?;
            writeln!(writer, "{}", edge_count)?;

            // write graph-data to file

            let mut progress_bar = progressing::BernoulliBar::from_goal(node_count + edge_count);
            info!("{}", progress_bar);

            // write nodes

            // for every node
            for node_idx in (0..node_count).into_iter().map(NodeIdx) {
                // loop over graphs config
                // and print respective data
                // if id fits

                // for every writing-cfg-item
                for (i, next_id) in writing_cfg.nodes.ids.iter().enumerate() {
                    if let Some(next_id) = next_id {
                        let mut has_been_written = false;

                        // Now: Ignore-case is already covered

                        // look for category of same id
                        // and write data to file
                        for category in graph.cfg().nodes.categories.iter() {
                            match category {
                                nodes::Category::Meta { info, id } => {
                                    if id != next_id {
                                        continue;
                                    }

                                    let node = graph.nodes().create(node_idx);
                                    match info {
                                        nodes::MetaInfo::NodeId => write!(writer, "{}", node.id())?,
                                        nodes::MetaInfo::NodeIdx => {
                                            write!(writer, "{}", node.idx())?
                                        }
                                        nodes::MetaInfo::Level => {
                                            write!(writer, "{}", node.level())?
                                        }
                                    }
                                }
                                nodes::Category::Metric { unit, id } => {
                                    if id != next_id {
                                        continue;
                                    }

                                    let node = graph.nodes().create(node_idx);
                                    match unit {
                                        nodes::metrics::UnitInfo::Latitude => write!(
                                            writer,
                                            "{:.digits$}",
                                            node.coord().lat.approx(),
                                            digits = accuracy::F64_FMT_DIGITS,
                                        )?,
                                        nodes::metrics::UnitInfo::Longitude => write!(
                                            writer,
                                            "{:.digits$}",
                                            node.coord().lon.approx(),
                                            digits = accuracy::F64_FMT_DIGITS,
                                        )?,
                                        nodes::metrics::UnitInfo::Height => {
                                            unimplemented!("Nodes' height is not supported yet.")
                                        }
                                    }
                                }
                                nodes::Category::Ignored => continue, // covered in else-case
                            }

                            // When here, no 'continue' has been called
                            // so sth has been written.
                            has_been_written = true;
                            break;
                        }

                        // if nothing has been written
                        // -> id is not in config
                        if !has_been_written {
                            panic!(format!(
                                "Writing-config has id {} which is not part of graph's node-data.",
                                next_id
                            ));
                        }
                    } else {
                        // if id is None
                        // -> ignore column
                        write!(writer, "{}", defaults::writing::IGNORE_STR)?;
                    }

                    // Sth has been written, so
                    // write space if needed
                    if i < writing_cfg.nodes.ids.len() - 1 {
                        write!(writer, " ")?;
                    }
                }

                // write end of line
                writeln!(writer, "")?;

                // print progress
                progress_bar.add(true);
                if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                    info!("{}", progress_bar);
                }
            }

            // write edges

            // for every edge
            for edge_idx in (0..edge_count).into_iter().map(EdgeIdx) {
                // loop over graphs config
                // and print respective data
                // if id fits

                // for every writing-cfg-item
                for (i, next_id) in writing_cfg.edges.ids.iter().enumerate() {
                    if let Some(next_id) = next_id {
                        let mut has_been_written = false;

                        // Now: Ignore-case is already covered

                        // look for category of same id
                        // and write data to file
                        for category in graph.cfg().edges.categories.iter() {
                            match category {
                                edges::Category::Meta { info, id } => {
                                    if id != next_id {
                                        continue;
                                    }

                                    match info {
                                        edges::MetaInfo::SrcId => {
                                            let src_idx = bwd_edges.dst_idx(edge_idx);
                                            let src_id = nodes.id(src_idx);
                                            write!(writer, "{}", src_id)?;
                                        }
                                        edges::MetaInfo::SrcIdx => {
                                            let src_idx = bwd_edges.dst_idx(edge_idx);
                                            write!(writer, "{}", src_idx)?;
                                        }
                                        edges::MetaInfo::DstId => {
                                            let dst_idx = fwd_edges.dst_idx(edge_idx);
                                            let dst_id = nodes.id(dst_idx);
                                            write!(writer, "{}", dst_id)?;
                                        }
                                        edges::MetaInfo::DstIdx => {
                                            let dst_idx = fwd_edges.dst_idx(edge_idx);
                                            write!(writer, "{}", dst_idx)?;
                                        }
                                        edges::MetaInfo::ShortcutIdx0 => {
                                            match fwd_edges.sc_edges(edge_idx) {
                                                Some(sc_edges) => {
                                                    write!(writer, "{}", sc_edges[0])?
                                                }
                                                None => write!(
                                                    writer,
                                                    "{}",
                                                    defaults::writing::NO_SHORTCUT_IDX
                                                )?,
                                            }
                                        }
                                        edges::MetaInfo::ShortcutIdx1 => {
                                            match fwd_edges.sc_edges(edge_idx) {
                                                Some(sc_edges) => {
                                                    write!(writer, "{}", sc_edges[1])?
                                                }
                                                None => write!(
                                                    writer,
                                                    "{}",
                                                    defaults::writing::NO_SHORTCUT_IDX
                                                )?,
                                            }
                                        }
                                    }
                                }
                                edges::Category::Metric {
                                    unit: _,
                                    id: metric_id,
                                } => {
                                    if metric_id != next_id {
                                        continue;
                                    }

                                    // get metric-idx from graph's config
                                    let metric_idx = graph
                                        .cfg()
                                        .edges
                                        .metrics
                                        .ids
                                        .iter()
                                        .position(|id| metric_id == id)
                                        .expect(&format!(
                                            "The metric-id {} doesn't exist in graph.",
                                            metric_id
                                        ));

                                    write!(
                                        writer,
                                        "{:.digits$}",
                                        graph.metrics()[edge_idx][metric_idx],
                                        digits = accuracy::F64_FMT_DIGITS
                                    )?;
                                }
                                edges::Category::Ignored => continue, // covered in else-case
                            }

                            // When here, no 'continue' has been called
                            // so sth has been written.
                            has_been_written = true;
                            break;
                        }

                        // if nothing has been written
                        // -> id is not in config
                        if !has_been_written {
                            panic!(format!(
                                "Writing-config has id {} which is not part of graph's edge-data.",
                                next_id
                            ))
                        }
                    } else {
                        // if id is None
                        // -> ignore column
                        write!(writer, "{}", defaults::writing::IGNORE_STR)?;
                    }

                    // Sth has been written, so
                    // write space if needed
                    if i < writing_cfg.edges.ids.len() - 1 {
                        write!(writer, " ")?;
                    }
                }

                // write end of line
                writeln!(writer, "")?;

                // print progress
                progress_bar.add(true);
                if progress_bar.progress().successes % (1 + (progress_bar.end() / 10)) == 0 {
                    info!("{}", progress_bar);
                }
            }
            info!("{}", progress_bar);

            Ok(())
        }

        // return result

        if let Err(e) = inner_write(graph, writing_cfg) {
            Err(format!("{}", e))
        } else {
            Ok(())
        }
    }
}
