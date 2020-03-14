use crate::defaults::{accuracy, capacity::DimVec};
use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    fs::File,
    path::Path,
    str::FromStr,
};

pub fn add(a: &DimVec<f64>, b: &DimVec<f64>) -> DimVec<f64> {
    a.iter().zip(b).map(|(aa, bb)| aa + bb).collect()
}

pub fn add_to(a: &mut DimVec<f64>, b: &DimVec<f64>) {
    a.iter_mut().zip(b).for_each(|(aa, bb)| *aa += bb);
}

pub fn dot_product(a: &DimVec<f64>, b: &DimVec<f64>) -> f64 {
    a.iter()
        .zip(b)
        .fold(0.0, |start, (aa, &bb)| start + aa * bb)
}

pub trait Approx {
    fn approx(self) -> f64;
}

pub trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
}

pub trait ApproxCmp {
    fn approx_partial_cmp(&self, other: &Self) -> Option<Ordering>;
    fn approx_cmp(&self, other: &Self) -> Ordering;
}

impl Approx for f64 {
    fn approx(self) -> f64 {
        (self / accuracy::F64_ABS).round() * accuracy::F64_ABS
    }
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &f64) -> bool {
        (self - other).abs() <= accuracy::F64_ABS
    }
}

impl ApproxCmp for f64 {
    fn approx_partial_cmp(&self, other: &f64) -> Option<Ordering> {
        match (self < other, self > other, self.approx_eq(other)) {
            (false, false, false) => None,
            (false, true, false) => Some(Greater),
            (true, false, false) => Some(Less),
            (true, true, false) | (_, _, true) => Some(Equal),
        }
    }

    fn approx_cmp(&self, other: &f64) -> Ordering {
        self.approx_partial_cmp(other).expect(&format!(
            "No f64-comparison for {} and {} possible.",
            self, other
        ))
    }
}

impl ApproxEq for DimVec<f64> {
    fn approx_eq(&self, other: &DimVec<f64>) -> bool {
        self.iter()
            .zip(other)
            .fold(true, |acc, (aa, bb)| acc && aa.approx_eq(bb))
    }
}

pub fn open_file<P: AsRef<Path> + ?Sized>(path: &P) -> Result<File, String> {
    let path = path.as_ref();
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("No such file {}", path.display())),
    }
}

pub fn open_new_file<P: AsRef<Path> + ?Sized>(path: &P) -> Result<File, String> {
    let path = path.as_ref();
    if path.exists() {
        return Err(format!(
            "Provided file {} does already exist. Please remove it.",
            path.display()
        ));
    }

    match File::create(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Sets the logging-level of this repo.
///
/// max_log_level: None
/// => use default (Warn)
///
/// modules: in addition to default (`env!("CARGO_PKG_NAME")`)
///
/// Environment-variable RUST_LOG has precedence.
pub fn init_logging(max_log_level: &str, mut modules: Vec<&str>) -> Result<(), String> {
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
    modules.push(env!("CARGO_PKG_NAME"));
    for module in modules {
        builder.filter(Some(module), max_log_level);
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

    // return
    Ok(())
}
