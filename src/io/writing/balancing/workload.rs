use crate::{configs, defaults, helpers::err, network::Graph};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Writer {
    iteration: usize,
}

impl Writer {
    pub fn new(iteration: usize) -> Writer {
        Writer { iteration }
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
                .join(defaults::explorating::files::capacities(
                    self.iteration,
                    balancing_cfg.num_iterations,
                ));
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

        writeln!(writer, "capacity")?;

        // write data

        let metric_idx = graph.cfg().edges.metrics.idx_of(&balancing_cfg.metric_id);
        let metrics = graph.metrics();

        for edge_idx in fwd_edges {
            writeln!(writer, "{}", metrics[edge_idx][*metric_idx])?;
        }

        Ok(())
    }
}
