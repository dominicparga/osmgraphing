use log::info;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

//------------------------------------------------------------------------------------------------//

fn init_logging(quietly: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if quiet logging: doesn't log `info` for the server and this repo
    if !quietly {
        builder.filter(Some(env!("CARGO_PKG_NAME")), log::LevelFilter::Info);
        builder.filter(Some("playground_random"), log::LevelFilter::Info);
    }
    // overwrite default with environment-variables
    if let Ok(filters) = std::env::var("RUST_LOG") {
        builder.parse_filters(&filters);
    }
    if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&write_style);
    }
    // init
    builder.init();
}

fn main() {
    init_logging(false);
    info!("Executing example: print seeded random numbers");

    let seed = 42;
    info!("Using seed {}", seed);
    let mut rng = rand_pcg::Pcg32::seed_from_u64(seed);
    let die = Uniform::from(1..=6);
    for _ in 0..10 {
        info!("Throw the die: {}", die.sample(&mut rng))
    }
}
