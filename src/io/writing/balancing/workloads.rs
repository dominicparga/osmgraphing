use crate::{configs, defaults, helpers::err, network::Graph};
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

impl Writer {
    pub fn write(
        &mut self,
        iter: usize,
        workloads: &Vec<usize>,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        // prepare

        // get writers

        let mut writer = {
            let path = balancing_cfg
                .results_dir
                .join(format!("{}", iter))
                .join(defaults::balancing::stats::DIR)
                .join(defaults::balancing::stats::files::ABS_WORKLOADS);
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

        writeln!(writer, "num_routes")?;

        // write data

        let fwd_edges = graph.fwd_edges();
        for edge_idx in fwd_edges
            .iter()
            .filter(|&edge_idx| !fwd_edges.is_shortcut(edge_idx))
        {
            writeln!(writer, "{}", workloads[*edge_idx])?;
        }

        Ok(())
    }
}
