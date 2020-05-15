use crate::{configs, io::SupportingFileExts, network::Graph};
use log::info;

mod workload;

trait Writing {
    fn write(
        &self,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> Result<(), String>;
}

pub struct Writer;

impl Writer {
    pub fn write(
        iteration: usize,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> Result<(), String> {
        info!(
            "START Write graph's route-workload with iteration {}",
            iteration
        );
        let result = workload::Writer::new(iteration).write(graph, balancing_cfg);
        info!("FINISHED");
        result
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}
