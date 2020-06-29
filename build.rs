use std::{env, fs, path::PathBuf};

mod defaults {
    pub const GRAPH_DIM: &str = "5";
}

fn main() {
    let out_dir = {
        let out_dir = env::var_os("OUT_DIR").expect("Env-var OUT_DIR is not set.");
        PathBuf::from(&out_dir)
    };

    // read in graph-dim from environment
    let graph_dim: usize = {
        let graph_dim = env::var_os("GRAPH_DIM").unwrap_or(defaults::GRAPH_DIM.into());
        let graph_dim = graph_dim.to_string_lossy();
        graph_dim.parse().expect(&format!(
            "The provided env-var GRAPH_DIM should be usize, but isn't (GRAPH_DIM={}).",
            graph_dim,
        ))
    };

    // https://stackoverflow.com/a/37528134
    //
    // write compiler-constants into file
    fs::write(
        out_dir.join("compiler.rs"),
        format!(
            "pub const GRAPH_DIM: usize = {};
            ",
            graph_dim
        ),
    )
    .expect("Writing compiler.rs didn't work.");

    // reruns
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=GRAPH_DIM");
}
