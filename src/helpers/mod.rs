use crate::defaults::capacity::DimVec;
use std::str::FromStr;

pub mod algebra;
pub mod approx;
pub mod err;

pub fn is_line_functional(line: &String) -> bool {
    line.len() > 0 && line.chars().next() != Some('#')
}

pub fn add(a: &DimVec<f64>, b: &DimVec<f64>) -> DimVec<f64> {
    a.iter().zip(b).map(|(aa, bb)| aa + bb).collect()
}

pub fn add_assign(a: &mut DimVec<f64>, b: &DimVec<f64>) {
    a.iter_mut().zip(b).for_each(|(aa, bb)| *aa += bb);
}

pub fn sub(a: &DimVec<f64>, b: &DimVec<f64>) -> DimVec<f64> {
    a.iter().zip(b).map(|(aa, bb)| aa - bb).collect()
}

pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .fold(0.0, |start, (aa, &bb)| start + aa * bb)
}

pub fn le(a: &[f64], b: &[f64]) -> bool {
    a.iter()
        .zip(b)
        .fold(true, |start, (aa, bb)| start && aa.le(bb))
}

/// For example:
/// Work off proto-edges in chunks to keep memory-usage lower.
/// To keep additional memory-needs below 1 MB, the the maximum amount of four f64-values per
/// worked-off chunk has to be limited to 250_000.
pub trait MemSize {
    fn mem_size_b() -> usize;
}

/// Sets the logging-level of this repo.
///
/// max_log_level: None
/// => use default (Warn)
///
/// modules: in addition to default (`env!("CARGO_PKG_NAME")`)
///
/// Environment-variable RUST_LOG has precedence.
pub fn init_logging(max_log_level: &str, modules: &[&str]) -> err::Feedback {
    let mut builder = env_logger::Builder::new();

    // maximum filter-level for all components: `warn`
    builder.filter(None, log::LevelFilter::Warn);

    // if quiet logging: doesn't log `info` for this repo
    let max_log_level = log::LevelFilter::from_str(&max_log_level.to_ascii_uppercase())
        .ok()
        .ok_or(format!(
            "The provided max-log-level {} is not supported.",
            max_log_level
        ))?;
    for module in modules {
        builder.filter(Some(module), max_log_level);
    }
    builder.filter(Some(env!("CARGO_PKG_NAME")), max_log_level);

    // overwrite default with environment-variables
    if let Ok(filters) = std::env::var("RUST_LOG") {
        builder.parse_filters(&filters);
    }
    if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&write_style);
    }

    // init
    builder.init();

    // return
    Ok(())
}
