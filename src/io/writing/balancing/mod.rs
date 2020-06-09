use crate::{configs, helpers::err, io::SupportingFileExts, network::Graph};
use log::info;

mod edges;
mod new_metrics;
mod workloads;

pub struct Writer;

impl Writer {
    pub fn write(
        workloads: &Vec<usize>,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        info!("START Write graph's route-workload");
        edges::Writer::new().write(graph, balancing_cfg)?;
        new_metrics::Writer::new().write(graph, balancing_cfg)?;
        workloads::Writer::new().write(workloads, graph, balancing_cfg)?;
        info!("FINISHED");
        Ok(())
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}
