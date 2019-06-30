pub mod err;

pub mod osm;
pub mod routing;
pub mod server;

//--------------------------------------------------------------------------------------------------
// logging
// from https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html#include-timestamp-in-log-messages
// and https://doc.servo.org/env_logger/struct.Builder.html#examples-2

#[macro_use]
extern crate log;

// use chrono::Local;
use env_logger::Builder;
// use std::io::Write;
// use log::LevelFilter;

pub struct Logging;
impl Logging {
    pub fn init() {
        // e.g. export RUST_LOG='warn,osmgraphing::osm::pbf=debug'
        // -> first is global default
        // -> comma-separated `path::to::module=level`
        // where `path::to::module` is rooted in the name of the crate it was compiled for.
        Builder::from_env("RUST_LOG")
            // .format(|buf, record| {
            //     writeln!(
            //         buf,
            //         "{} [{}] - {}",
            //         Local::now().format("%Y-%m-%dT%H:%M:%S"),
            //         record.level(),
            //         record.args()
            //     )
            // })
            // .filter(None, LevelFilter::Error)
            .init();

        debug!("Initializing Logger has finished.");
    }
}
