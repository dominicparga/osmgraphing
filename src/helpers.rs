use std::{fs::File, path::Path, str::FromStr};

pub fn open_file<P: AsRef<Path> + ?Sized>(path: &P) -> Result<File, String> {
    let path = path.as_ref();
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("No such file {}", path.display())),
    }
}

pub enum MapFileExt {
    PBF,
    FMI,
}

impl MapFileExt {
    pub fn from_path<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Self, String> {
        let supported_exts = &["pbf", "fmi"];
        let path = path.as_ref();

        // if file has extension
        if let Some(os_str) = path.extension() {
            // if filename is valid unicode
            if let Some(extension) = os_str.to_str() {
                // check if parser supports extension
                match extension.to_ascii_lowercase().as_ref() {
                    "pbf" => Ok(MapFileExt::PBF),
                    "fmi" => Ok(MapFileExt::FMI),
                    // parser doesn't support this extension
                    unsupported_ext => Err(format!(
                        "Unsupported extension `{}` was given. Supported extensions are {:?}",
                        unsupported_ext, supported_exts
                    )),
                }
            } else {
                Err(String::from("Filename is invalid Unicode."))
            }
        } else {
            Err(format!(
                "The file {:?} has no extension. Supported extensions are {:?}",
                &path, supported_exts
            ))
        }
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
