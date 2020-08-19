use crate::{
    configs::{self, evaluating_balance::Config as WritingConfig, SimpleId},
    defaults,
    helpers::err,
    io::{self, SupportingFileExts},
    network::Graph,
};
use std::fmt::Display;

pub struct Writer;

impl Writer {
    pub fn check(writing_cfg: &WritingConfig) -> err::Feedback {
        for path in &[
            writing_cfg
                .results_dir
                .join(&writing_cfg.monitoring.edges_info.file),
            writing_cfg
                .results_dir
                .join(defaults::balancing::stats::files::ABS_WORKLOADS),
            writing_cfg
                .results_dir
                .join(defaults::smarts::XML_FILE_NAME),
        ] {
            if path.exists() {
                return Err(err::Msg::from(format!(
                    "New file {} does already exist. Please remove it.",
                    path.display()
                )));
            }
        }
        Ok(())
    }

    pub fn write<T>(values: &[T], graph: &Graph, writing_cfg: &WritingConfig) -> err::Feedback
    where
        T: Display,
    {
        // write edges-info

        let mut tmp_cfg = writing_cfg.monitoring.edges_info.clone();
        // path is relative to results-dir
        tmp_cfg.file = writing_cfg.results_dir.join(tmp_cfg.file);
        io::network::edges::Writer::write(&graph, &tmp_cfg)?;

        // write absolute workloads

        // look for name of edge-id (which occurs only once)
        let edge_id_name = {
            let id = graph
                .cfg()
                .edges
                .categories
                .iter()
                .find_map(|category| match category {
                    configs::parsing::edges::Category::Meta { info, id } => {
                        if info == &configs::parsing::edges::MetaInfo::EdgeId {
                            Some(id)
                        } else {
                            None
                        }
                    }
                    configs::parsing::edges::Category::Metric { unit: _, id: _ }
                    | configs::parsing::edges::Category::Ignored => None,
                });
            if let Some(id) = id {
                id.clone()
            } else {
                return Err(err::Msg::from(
                    "For writing absolute workloads to csv, an edge-id should be given.",
                ));
            }
        };
        let mut tmp_cfg = writing_cfg.monitoring.edges_info.clone();
        // path is relative to results-dir
        tmp_cfg.file = writing_cfg
            .results_dir
            .join(defaults::balancing::stats::files::ABS_WORKLOADS);
        // header-line
        tmp_cfg.ids = vec![
            Some(edge_id_name),
            Some(SimpleId::from(
                defaults::balancing::stats::csv_names::NUM_ROUTES,
            )),
        ];
        io::network::edges::Writer::write_external_values(values, &graph, &tmp_cfg)?;

        Ok(())
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["csv"]
    }
}
