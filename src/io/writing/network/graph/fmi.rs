use crate::{
    configs::{parsing::nodes, writing},
    defaults,
    helpers::err,
    io::writing::network::write_edges_to_file,
    network::Graph,
};
use log::info;
use progressing::{self, bernoulli::Bar as BernoulliBar, Baring};
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

impl Writer {
    pub fn write(
        &self,
        graph: &Graph,
        writing_cfg: &writing::network::graph::Config,
    ) -> err::Feedback {
        // prepare

        let output_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&writing_cfg.map_file)?;
        let mut writer = BufWriter::new(output_file);

        let fwd_edges = graph.fwd_edges();
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
        writeln!(
            writer,
            "# Edges' metrics are {} by their mean.",
            if writing_cfg.edges.is_denormalizing {
                "not normalized"
            } else {
                "normalized"
            }
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
        writeln!(writer, "{}", dim)?;
        writeln!(writer, "{}", nodes.count())?;
        // only write non-shortcuts
        writeln!(
            writer,
            "{}",
            fwd_edges
                .iter()
                .filter(|&edge_idx| !fwd_edges.is_shortcut(edge_idx)
                    || writing_cfg.edges.is_writing_shortcuts)
                .count()
        )?;

        // write nodes

        let mut progress_bar = BernoulliBar::with_goal(nodes.count()).timed();
        info!("{}", progress_bar);

        // for every node
        for node_idx in &nodes {
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
                                    nodes::MetaInfo::NodeIdx => write!(writer, "{}", node.idx())?,
                                    nodes::MetaInfo::CHLevel => {
                                        write!(writer, "{}", node.ch_level())?
                                    }
                                }
                            }
                            nodes::Category::Metric { unit, id } => {
                                if id != next_id {
                                    continue;
                                }

                                let node = graph.nodes().create(node_idx);
                                match unit {
                                    nodes::metrics::UnitInfo::Latitude => {
                                        write!(writer, "{}", node.coord().lat)?
                                    }
                                    nodes::metrics::UnitInfo::Longitude => {
                                        write!(writer, "{}", node.coord().lon)?
                                    }
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
                        return Err(format!(
                            "Writing-config has id {} which is not part of graph's node-data.",
                            next_id
                        )
                        .into());
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
            if progress_bar.has_progressed_significantly() {
                progress_bar.remember_significant_progress();
                info!("{}", progress_bar);
            }
        }

        // write edges
        write_edges_to_file(
            &mut writer,
            &graph,
            &writing::network::edges::Config::from(writing_cfg.clone()),
        )?;

        info!("FINISHED");
        Ok(())
    }
}
