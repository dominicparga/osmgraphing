use crate::{
    configs, defaults,
    helpers::err,
    network::{EdgeIdx, Graph},
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Writer {}

impl Writer {
    pub fn new() -> Writer {
        Writer {}
    }
}

impl super::Writing for Writer {
    fn write(
        &mut self,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        // prepare

        let fwd_edges = graph.fwd_edges();

        // get writers

        let mut writer = {
            let path = balancing_cfg
                .results_dir
                .join(defaults::explorating::files::EDGES_WRITER);
            let output_file = match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(f) => f,
                Err(e) => {
                    return Err(err::Msg::from(format!(
                        "Couldn't open {} due to error: {}",
                        path.display(),
                        e
                    )))
                }
            };
            BufWriter::new(output_file)
        };

        // write header

        writeln!(writer, "src-lat src-lon dst-lat dst-lon")?;

        // write data

        for _edge_idx in (0..fwd_edges.count()).map(EdgeIdx) {
            writeln!(writer, "{} {} {} {}", 1, 2, 3, 4)?; // TODO
        }

        Ok(())
    }
}
