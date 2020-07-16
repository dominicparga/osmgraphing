use crate::{configs, helpers::err, io::SupportingFileExts, network::Graph};
use log::info;

mod random_or_all;

pub struct Writer;

impl Writer {
    pub fn write(
        graph: &Graph,
        routing_cfg: &configs::routing::Config,
        writing_cfg: &configs::writing::routing::Config,
    ) -> err::Feedback {
        info!(
            "START Write routes {} from graph with {:?}",
            writing_cfg.file.display(),
            writing_cfg.category
        );
        let result = match writing_cfg.category {
            configs::writing::routing::Category::RandomOrAll { seed, max_count } => {
                random_or_all::Writer::new(seed, max_count).write(graph, routing_cfg, writing_cfg)
            }
        };
        info!("FINISHED");
        result
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["route-pairs"]
    }
}
