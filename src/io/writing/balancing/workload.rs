use crate::{
    configs, defaults,
    network::{EdgeIdx, Graph},
};
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
        &self,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> Result<(), String> {
        fn inner_write(
            myself: &Writer,
            graph: &Graph,
            balancing_cfg: &configs::balancing::Config,
        ) -> Result<(), Box<dyn std::error::Error>> {
            // prepare

            let nodes = graph.nodes();
            let fwd_edges = graph.fwd_edges();

            // get writer to file and, if file is new, add header

            let mut writer = {
                if balancing_cfg.results_file.exists() {
                    let output_file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&balancing_cfg.results_file)?;
                    BufWriter::new(output_file)
                } else {
                    let output_file = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&balancing_cfg.results_file)?;
                    let mut writer = BufWriter::new(output_file);

                    // write header

                    writeln!(writer, "# graph-file: {}", graph.cfg().map_file.display())?;
                    writeln!(writer, "# node-count: {}", nodes.count(),)?;
                    writeln!(writer, "# edge-count: {}", fwd_edges.count(),)?;
                    writeln!(writer, "")?;
                    writeln!(writer, "# [ [ src-id, dst-id ], [ ... ] ]")?;
                    write!(writer, "edge-nodes-ids: [ ")?;
                    let bwd_edges = graph.bwd_edges();
                    for edge_idx in (0..(fwd_edges.count() - 1)).map(EdgeIdx) {
                        let src_idx = bwd_edges.dst_idx(edge_idx);
                        let dst_idx = fwd_edges.dst_idx(edge_idx);
                        let src_id = nodes.id(src_idx);
                        let dst_id = nodes.id(dst_idx);
                        write!(writer, "[ {}, {} ], ", src_id, dst_id)?;
                    }
                    let edge_idx = EdgeIdx(fwd_edges.count() - 1);
                    let src_idx = bwd_edges.dst_idx(edge_idx);
                    let dst_idx = fwd_edges.dst_idx(edge_idx);
                    let src_id = nodes.id(src_idx);
                    let dst_id = nodes.id(dst_idx);
                    writeln!(writer, "[ {}, {} ] ]", src_id, dst_id)?;

                    writeln!(writer, "")?;
                    writeln!(writer, "# [ workload-for-edge-idx-0, wfei-1, wfei-2, ... ]")?;
                    writeln!(writer, "route-counts-per-edge-idx:")?;

                    writer
                }
            };

            // write edges

            let metric_id = defaults::balancing::ROUTE_COUNT_ID;
            let metric_idx = graph.cfg().edges.metrics.idx_of(metric_id).expect(&format!(
                "Metric-id {} should be existent in graph, but isn't.",
                metric_id
            ));
            let metrics = graph.metrics();

            write!(
                writer,
                "  iteration-{:0digits$}: [ ",
                myself.iteration,
                digits = format!("{}", balancing_cfg.num_iterations).len()
            )?;
            for edge_idx in (0..(fwd_edges.count() - 1)).map(EdgeIdx) {
                write!(writer, "{}, ", metrics[edge_idx][*metric_idx])?;
            }
            writeln!(
                writer,
                "{} ]",
                metrics[EdgeIdx(fwd_edges.count() - 1)][*metric_idx]
            )?;

            Ok(())
        }

        // return result

        if let Err(e) = inner_write(self, graph, balancing_cfg) {
            Err(format!("{}", e))
        } else {
            Ok(())
        }
    }
}
