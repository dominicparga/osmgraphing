use crate::{configs, defaults, helpers::err, network::Graph};
use kissunits::distance::Kilometers;
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

        let metrics = graph.metrics();

        for edge_idx in fwd_edges {
            // read metrics-data from graph
            let (distance, route_count, lane_count) = {
                let tmp = &metrics[edge_idx];
                (
                    tmp[*balancing_cfg.distance_idx],
                    tmp[*balancing_cfg.route_count_idx],
                    tmp[*balancing_cfg.lane_count_idx] as u64,
                )
            };
            // calculate
            let distance = {
                let unit = graph.cfg().edges.metrics.units[*balancing_cfg.distance_idx];
                // convert value to meters
                let raw_value = unit.convert(
                    &configs::parsing::edges::metrics::UnitInfo::Kilometers,
                    distance,
                );
                Kilometers(raw_value)
            };
            let num_vehicles = defaults::vehicles::calc_num_vehicles(distance);
            let capacity = lane_count * num_vehicles;
            let workload = route_count / (capacity as f64);
            writeln!(writer, "{}", workload)?;
        }

        Ok(())
    }
}
