use crate::{
    configs::writing,
    helpers::err,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;

pub mod fmi;

trait Writing {
    fn write(&self, graph: &Graph, writing_cfg: &writing::network::Config) -> err::Feedback;
}

pub struct Writer;

impl Writer {
    pub fn write(graph: &Graph, writing_cfg: &writing::network::Config) -> err::Feedback {
        info!("START Write file from graph");
        match Writer::from_path(&writing_cfg.map_file)? {
            MapFileExt::FMI => {
                fmi::Writer::new().write(graph, writing_cfg)?;
                info!("FINISHED");
                Ok(())
            }
            MapFileExt::PBF => Err("No support for writing pbf-files.".into()),
        }
    }
}

impl SupportingMapFileExts for Writer {}
impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["fmi"]
    }
}
