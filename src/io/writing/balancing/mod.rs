use crate::{configs, helpers::err, io::SupportingFileExts, network::Graph};
use log::info;

mod edges;
mod num_routes;
mod workload;

pub struct Writer;

impl Writer {
    pub fn write(
        abs_workloads: &Vec<usize>,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        info!("START Write graph's route-workload");
        edges::Writer::new().write(graph, balancing_cfg)?;
        workload::Writer::new().write(graph, balancing_cfg)?;
        num_routes::Writer::new().write(abs_workloads, graph, balancing_cfg)?;
        info!("FINISHED");
        Ok(())
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["yaml"]
    }
}
