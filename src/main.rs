fn main() {
    env_logger::Builder::from_env("RUST_LOG").init();

    println!("For all supported example files, please use");
    println!("");
    println!("    'cargo run --example'");
    println!("");
    println!("");
    println!("You can set up the logger by setting RUST_LOG, e.g. to");
    println!("");
    println!("    export RUST_LOG='warn,osmgraphing=info,parser=info,dijkstra=info'");
    println!("");
    println!(
        "for getting warnings per default, but info about osmgraphing and the 'parser'-example."
    );
}
