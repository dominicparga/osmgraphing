pub mod edges;
pub mod graph;

use crate::{
    configs,
    defaults::{self, accuracy},
    helpers::err,
    network::Graph,
};
use log::info;
use progressing::Bar;
use std::io::Write;

fn write_edges_to_file<W: Write>(
    writer: &mut W,
    graph: &Graph,
    writing_cfg: &crate::configs::writing::network::edges::Config,
) -> err::Feedback {
    let fwd_edges = graph.fwd_edges();
    let bwd_edges = graph.bwd_edges();
    let nodes = graph.nodes();

    // write header

    if writing_cfg.is_writing_header {
        for (i, edge_info) in writing_cfg
            .ids
            .iter()
            .map(|id| match id {
                Some(id) => format!("{}", id.0),
                None => format!("{}", defaults::writing::IGNORE_STR),
            })
            .enumerate()
        {
            write!(writer, "{}", edge_info)?;

            // Sth has been written, so
            // write space if needed
            if i < writing_cfg.ids.len() - 1 {
                write!(writer, " ")?;
            }
        }

        // write end of line
        writeln!(writer, "")?;
    }

    // write edges to file

    let mut progress_bar = progressing::BernoulliBar::from_goal(fwd_edges.count());
    info!("{}", progress_bar);

    // for every edge
    for edge_idx in fwd_edges
        .iter()
        .filter(|&edge_idx| !fwd_edges.is_shortcut(edge_idx) || writing_cfg.is_writing_shortcuts)
    {
        // loop over graphs config
        // and print respective data
        // if id fits

        // for every writing-cfg-item
        for (i, next_id) in writing_cfg.ids.iter().enumerate() {
            if let Some(next_id) = next_id {
                let mut has_been_written = false;

                // Now: Ignore-case is already covered

                // look for category of same id
                // and write data to file
                for category in graph.cfg().edges.categories.iter() {
                    match category {
                        configs::parsing::edges::Category::Meta { info, id } => {
                            if id != next_id {
                                continue;
                            }

                            match info {
                                configs::parsing::edges::MetaInfo::EdgeId => {
                                    let edge_id = fwd_edges.id(edge_idx);
                                    write!(writer, "{}", edge_id)?;
                                }
                                configs::parsing::edges::MetaInfo::SrcId => {
                                    let src_idx = bwd_edges.dst_idx(edge_idx);
                                    let src_id = nodes.id(src_idx);
                                    write!(writer, "{}", src_id)?;
                                }
                                configs::parsing::edges::MetaInfo::SrcIdx => {
                                    let src_idx = bwd_edges.dst_idx(edge_idx);
                                    write!(writer, "{}", src_idx)?;
                                }
                                configs::parsing::edges::MetaInfo::DstId => {
                                    let dst_idx = fwd_edges.dst_idx(edge_idx);
                                    let dst_id = nodes.id(dst_idx);
                                    write!(writer, "{}", dst_id)?;
                                }
                                configs::parsing::edges::MetaInfo::DstIdx => {
                                    let dst_idx = fwd_edges.dst_idx(edge_idx);
                                    write!(writer, "{}", dst_idx)?;
                                }
                                configs::parsing::edges::MetaInfo::ShortcutIdx0 => {
                                    match fwd_edges.sc_edges(edge_idx) {
                                        Some(sc_edges) => write!(writer, "{}", sc_edges[0])?,
                                        None => write!(
                                            writer,
                                            "{}",
                                            defaults::writing::NO_SHORTCUT_IDX
                                        )?,
                                    }
                                }
                                configs::parsing::edges::MetaInfo::ShortcutIdx1 => {
                                    match fwd_edges.sc_edges(edge_idx) {
                                        Some(sc_edges) => write!(writer, "{}", sc_edges[1])?,
                                        None => write!(
                                            writer,
                                            "{}",
                                            defaults::writing::NO_SHORTCUT_IDX
                                        )?,
                                    }
                                }
                            }
                        }
                        configs::parsing::edges::Category::Metric {
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
                        configs::parsing::edges::Category::Ignored => continue, // covered in else-case
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
                        "Writing-config has id {} which is not part of graph's edge-data.",
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
            if i < writing_cfg.ids.len() - 1 {
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
