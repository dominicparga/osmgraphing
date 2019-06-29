// from https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html#include-timestamp-in-log-messages
// and https://doc.servo.org/env_logger/struct.Builder.html#examples-2

#[macro_use]
extern crate log;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
// use log::LevelFilter;

fn main() {
    // e.g. export RUST_LOG='warn,logging=info'
    Builder::from_env("RUST_LOG")
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        // .filter(None, LevelFilter::Error)
        .init();

    warn!("warn");
    info!("info");
    debug!("debug");
}
