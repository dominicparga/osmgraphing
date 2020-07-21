use crate::{approximating::Approx, configs, defaults, helpers::err, network::Graph};
use kissunits::distance::Kilometers;
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
        iter: usize,
        graph: &Graph,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        // prepare

        let nodes = graph.nodes();
        let fwd_edges = graph.fwd_edges();
        let bwd_edges = graph.bwd_edges();
        let metrics = graph.metrics();

        // get writers

        let mut writer = {
            let path = balancing_cfg
                .results_dir
                .join(format!("{}", iter))
                .join(defaults::balancing::stats::DIR)
                .join(defaults::balancing::stats::files::EDGES_WRITER);
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

        writeln!(
            writer,
            "edge-id src_lat src_lon dst_lat dst_lon kilometers lane_count"
        )?;

        // write data

        let distance_idx = graph.cfg().edges.metrics.idx_of(&balancing_cfg.distance_id);
        let lane_count_idx = graph
            .cfg()
            .edges
            .metrics
            .idx_of(&balancing_cfg.lane_count_id);
        let distance_unit = graph.cfg().edges.metrics.units[*distance_idx];

        for edge_idx in fwd_edges
            .iter()
            .filter(|&edge_idx| !fwd_edges.is_shortcut(edge_idx))
        {
            let edge_id = fwd_edges.id(edge_idx);

            let (src_lat, src_lon) = {
                let idx = bwd_edges.dst_idx(edge_idx);
                let coord = Approx(nodes.coord(idx)).approx();
                (coord.lat, coord.lon)
            };
            let (dst_lat, dst_lon) = {
                let idx = fwd_edges.dst_idx(edge_idx);
                let coord = Approx(nodes.coord(idx)).approx();
                (coord.lat, coord.lon)
            };

            let (raw_distance, lane_count) = {
                let tmp = &metrics[edge_idx];
                (tmp[*distance_idx], tmp[*lane_count_idx] as u64)
            };
            // use correct unit for distance
            let distance = {
                // convert value to meters
                let raw_value = distance_unit.convert(
                    &configs::parsing::edges::metrics::UnitInfo::Kilometers,
                    raw_distance,
                );
                Kilometers(raw_value)
            };

            writeln!(
                writer,
                "{} {} {} {} {} {} {}",
                edge_id, src_lat, src_lon, dst_lat, dst_lon, *distance, lane_count
            )?;
        }

        Ok(())
    }
}
