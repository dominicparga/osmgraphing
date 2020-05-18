use crate::{configs, helpers::err, io::SupportingFileExts, network::Graph};
use log::info;

mod edges;
mod workload;

trait Writing {
    fn write(&mut self, graph: &Graph, balancing_cfg: &configs::balancing::Config)
        -> err::Feedback;
}

pub struct Writer;

impl Writer {
    pub fn write(
        iteration: usize,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        info!(
            "START Write graph's route-workload with iteration {}",
            iteration
        );
        if iteration == 0 {
            edges::Writer::new().write(graph, balancing_cfg)?;
        }
        workload::Writer::new(iteration).write(graph, balancing_cfg)?;
        info!("FINISHED");
        Ok(())
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}
