use crate::{
    configs, defaults,
    helpers::{approx::Approx, err},
    network::Graph,
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

    pub fn write(
        &mut self,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        // prepare

        let nodes = graph.nodes();
        let fwd_edges = graph.fwd_edges();
        let bwd_edges = graph.bwd_edges();

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

        for edge_idx in fwd_edges
            .iter()
            .filter(|&edge_idx| !fwd_edges.is_shortcut(edge_idx))
        {
            let (src_lat, src_lon) = {
                let idx = bwd_edges.dst_idx(edge_idx);
                let coord = nodes.coord(idx);
                (coord.lat.approx(), coord.lon.approx())
            };
            let (dst_lat, dst_lon) = {
                let idx = fwd_edges.dst_idx(edge_idx);
                let coord = nodes.coord(idx);
                (coord.lat.approx(), coord.lon.approx())
            };
            writeln!(writer, "{} {} {} {}", src_lat, src_lon, dst_lat, dst_lon)?;
        }

        Ok(())
    }
}
