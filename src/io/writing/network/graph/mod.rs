use crate::{
    configs::writing::network::graph::Config as WritingConfig,
    helpers::err,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;

pub mod fmi;

pub struct Writer;

impl Writer {
    pub fn write(graph: &Graph, writing_cfg: &WritingConfig) -> err::Feedback {
        info!(
            "START Write the graph with {}",
            writing_cfg.map_file.display()
        );

        match Writer::from_path(&writing_cfg.map_file)? {
            MapFileExt::FMI => fmi::Writer::new().write(graph, writing_cfg)?,
            MapFileExt::PBF => {
                return Err(format!("No support for writing pbf-files.").into());
            }
        }

        info!("FINISHED");
        Ok(())
    }
}

impl SupportingMapFileExts for Writer {}
impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["fmi"]
    }
}
