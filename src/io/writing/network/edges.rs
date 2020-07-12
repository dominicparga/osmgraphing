use crate::{
    configs::writing::network::edges::Config as WritingConfig,
    helpers::err,
    io::{writing::network::write_edges_to_file, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;
use std::{fs::OpenOptions, io::BufWriter};

pub struct Writer;

impl Writer {
    pub fn write(graph: &Graph, writing_cfg: &WritingConfig) -> err::Feedback {
        info!(
            "START Write the graph's edges with {}",
            writing_cfg.file.display()
        );

        if !Self::is_file_supported(&writing_cfg.file) {
            return Err(format!("No support for writing {}.", writing_cfg.file.display()).into());
        }

        // prepare

        let output_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&writing_cfg.file)?;
        let mut writer = BufWriter::new(output_file);

        write_edges_to_file(&mut writer, graph, writing_cfg)?;

        info!("FINISHED");
        Ok(())
    }
}

impl SupportingMapFileExts for Writer {}
impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["csv"]
    }
}
