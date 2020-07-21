pub mod approximating;
pub mod configs;
pub mod defaults;
pub mod helpers;
pub mod io;
pub mod multi_ch_constructor;
pub mod network;
pub mod routing;

pub mod compiler {
    include!(concat!(env!("OUT_DIR"), "/", "compiler.rs"));
}
