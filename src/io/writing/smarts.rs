use crate::{
    configs::writing::smarts::Config as WritingConfig, defaults, helpers::err,
    io::SupportingFileExts, network::Graph, routing::paths::Path,
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Writer;

impl Writer {
    /// Route-File-Format from [SMARTS-homepage](https://projects.eng.unimelb.edu.au/smarts/documentation/)
    pub fn write(
        chosen_paths: &[Path],
        graph: &Graph,
        writing_cfg: &WritingConfig,
    ) -> err::Feedback {
        // prepare

        let output_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&writing_cfg.file)?;
        let mut writer = BufWriter::new(output_file);

        let fwd_edges = graph.fwd_edges();
        let bwd_edges = graph.bwd_edges();
        let nodes = graph.nodes();

        // write header

        writeln!(
            writer,
            "<?xml version=\"{}\" encoding=\"UTF-8\"?>",
            defaults::smarts::route_file_format::VERSION
        )?;
        writeln!(writer, "<data>")?;

        for path in chosen_paths.iter() {
            let id = format!("{}->{}", nodes.id(path.src_idx()), nodes.id(path.dst_idx()));

            // every path refers to a vehicle
            writeln!(
                writer,
                "<vehicle id=\"{}\" type=\"{}\" start_time=\"{}\" driverProfile=\"{}\">",
                id,
                defaults::smarts::route_file_format::VEHICLE_TYPE,
                defaults::smarts::route_file_format::START_TIME,
                defaults::smarts::route_file_format::DRIVER_PROFILE
            )?;

            let mut is_first = true;
            // write passing nodes to file
            for &edge_idx in path {
                if is_first {
                    let src_id = nodes.id(bwd_edges.dst_idx(edge_idx));
                    writeln!(writer, "<node id=\"{}\"/>", src_id)?;
                    is_first = false;
                }

                let dst_id = nodes.id(fwd_edges.dst_idx(edge_idx));
                writeln!(writer, "<node id=\"{}\"/>", dst_id)?;
            }

            writeln!(writer, "</vehicle>")?;
        }

        writeln!(writer, "</data>")?;

        Ok(())
    }
}

impl SupportingFileExts for Writer {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["csv"]
    }
}
