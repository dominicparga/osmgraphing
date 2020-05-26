use crate::{configs::writing, helpers::err, io::SupportingFileExts, network::Graph};
use log::info;

mod random_or_all;

trait Writing {
    fn write(&self, graph: &Graph, writing_cfg: &writing::routing::Config) -> err::Feedback;
}

pub struct Writer;

impl Writer {
    pub fn write(graph: &Graph, writing_cfg: &writing::routing::Config) -> err::Feedback {
        info!(
            "START Write routes {} from graph",
            writing_cfg.file.display()
        );
        let result = match writing_cfg.category {
            writing::routing::Category::RandomOrAll { seed, max_count } => {
                random_or_all::Writer::new(seed, max_count).write(graph, writing_cfg)
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
