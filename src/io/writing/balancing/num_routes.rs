use crate::{configs, helpers::err};
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
        abs_workload: &Vec<usize>,
        balancing_cfg: &configs::balancing::Config,
    ) -> err::Feedback {
        // prepare

        // get writers

        let mut writer = {
            let path = balancing_cfg.results_dir.join("abs_workloads.csv");
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

        for num_routes in abs_workload {
            writeln!(writer, "{}", num_routes)?;
        }

        Ok(())
    }
}
