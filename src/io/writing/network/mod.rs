use crate::{
    configs::writing,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;

pub mod fmi;

trait Writing {
    fn write(&self, graph: &Graph, writing_cfg: &writing::network::Config) -> Result<(), String>;
}

pub struct Writer;

impl Writer {
    pub fn write(graph: &Graph, writing_cfg: &writing::network::Config) -> Result<(), String> {
        info!("START Write file from graph");
        match Writer::from_path(&writing_cfg.map_file)? {
            MapFileExt::FMI => {
                fmi::Writer::new().write(graph, writing_cfg)?;
                info!("FINISHED");
                Ok(())
            }
            MapFileExt::PBF => Err(String::from("No support for writing pbf-files.")),
        }
    }
}

impl SupportingMapFileExts for Writer {}
impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["fmi"]
    }
}
