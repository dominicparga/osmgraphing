use crate::{
    configs::writing::network::edges::Config as WritingConfig,
    defaults,
    helpers::err,
    io::{writing::network::write_edges_to_file, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;
use std::{
    fmt::Display,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Writer;

impl Writer {
    pub fn check(writing_cfg: &WritingConfig) -> err::Feedback {
        if writing_cfg.file.exists() {
            Err(err::Msg::from(
                "New map-file {} does already exist. Please remove it.",
            ))
        } else {
            Ok(())
        }
    }

    pub fn write(graph: &Graph, writing_cfg: &WritingConfig) -> err::Feedback {
        info!(
            "START Write the graph's edges with {}",
            writing_cfg.file.display()
        );

        if !Self::is_file_supported(&writing_cfg.file) {
            return Err(format!("No support for writing {}.", writing_cfg.file.display()).into());
        }

        // prepare

        let output_file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&writing_cfg.file)
        {
            Ok(file) => file,
            Err(e) => {
                return Err(err::Msg::from(format!(
                    "Couldn't open {} due to error: {}",
                    writing_cfg.file.display(),
                    e
                )))
            }
        };
        let mut writer = BufWriter::new(output_file);

        write_edges_to_file(&mut writer, graph, writing_cfg)?;

        info!("FINISHED");
        Ok(())
    }

    /// Ignores `writing_cfg.is_denormalizing`, because no mean is provided.
    pub fn write_external_values<T: Display>(
        values: &[T],
        graph: &Graph,
        writing_cfg: &WritingConfig,
    ) -> err::Feedback {
        if !Self::is_file_supported(&writing_cfg.file) {
            return Err(err::Msg::from(format!(
                "No support for writing {}.",
                writing_cfg.file.display()
            )));
        }

        let mut writer = {
            let output_file = match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&writing_cfg.file)
            {
                Ok(f) => f,
                Err(e) => {
                    return Err(err::Msg::from(format!(
                        "Couldn't open {} due to error: {}",
                        writing_cfg.file.display(),
                        e
                    )))
                }
            };
            BufWriter::new(output_file)
        };

        // write header

        if writing_cfg.is_writing_header {
            for (i, id) in writing_cfg.ids.iter().enumerate() {
                write!(
                    writer,
                    "{}",
                    id.as_ref()
                        .map(|id| id.as_ref())
                        .unwrap_or(defaults::writing::IGNORE_STR)
                )?;

                // if not last line -> enter space
                if i < writing_cfg.ids.len() - 1 {
                    write!(writer, " ")?;
                }
            }
            writeln!(writer, "")?;
        }

        // write values

        let fwd_edges = graph.fwd_edges();
        for edge_idx in fwd_edges.iter().filter(|&edge_idx| {
            writing_cfg.is_writing_shortcuts || !fwd_edges.is_shortcut(edge_idx)
        }) {
            writeln!(writer, "{} {}", fwd_edges.id(edge_idx), values[*edge_idx])?;
        }

        Ok(())
    }
}

impl SupportingMapFileExts for Writer {}
impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["csv"]
    }
}
