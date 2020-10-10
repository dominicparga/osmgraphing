use crate::helpers::err;
use std::path::Path;

mod parsing;
mod writing;

pub mod smarts {
    pub use crate::io::writing::smarts::Writer;
}
pub mod network {
    pub mod graph {
        pub use crate::io::parsing::network::graph::Parser;
        pub use crate::io::writing::network::graph::Writer;
    }
    pub mod edges {
        pub use crate::io::parsing::network::edges::Parser;
        pub use crate::io::writing::network::edges::Writer;
    }
}
pub mod routing {
    pub use crate::io::parsing::routing::Parser;
    pub use crate::io::writing::routing::Writer;
}
#[cfg(feature = "gpl-3.0")]
pub mod evaluating_balance {
    pub use crate::io::writing::evaluating_balance::Writer;
}

pub fn ext_from<P: AsRef<Path> + ?Sized>(path: &P) -> err::Result<&str> {
    let path = path.as_ref();

    // if file has extension
    if let Some(os_str) = path.extension() {
        // if filename is valid unicode
        if let Some(extension) = os_str.to_str() {
            Ok(extension)
        } else {
            Err(err::Msg::from("Filename is invalid Unicode."))
        }
    } else {
        Err(err::Msg::from(format!(
            "The file {:?} has no extension.",
            &path,
        )))
    }
}

pub trait SupportingFileExts {
    fn ext_from<P: AsRef<Path> + ?Sized>(path: &P) -> err::Result<&str> {
        ext_from(path)
    }

    fn supported_exts<'a>() -> &'a [&'a str];

    fn find_supported_ext<'a, P: AsRef<Path> + ?Sized>(path: &P) -> err::Result<&'a str> {
        let supported_exts = Self::supported_exts();

        match Self::ext_from(path) {
            Ok(extension) => {
                // check if extension is supported
                for supported_ext in supported_exts {
                    if supported_ext == &extension.to_ascii_lowercase() {
                        return Ok(supported_ext);
                    }
                }
                // extension is not supported
                Err(err::Msg::from(format!(
                    "Unsupported extension `{}` was given. Supported extensions are {:?}",
                    extension, supported_exts
                )))
            }
            Err(msg) => Err(err::Msg::from(format!(
                "{} Supported extensions are {:?}",
                msg, supported_exts
            ))),
        }
    }

    fn check_ext_support<P: AsRef<Path> + ?Sized>(path: &P) -> err::Feedback {
        Self::find_supported_ext(path).map(|_ext| ())
    }

    fn is_file_supported<P: AsRef<Path> + ?Sized>(path: &P) -> bool {
        Self::find_supported_ext(path).is_ok()
    }
}

pub enum MapFileExt {
    PBF,
    FMI,
}

impl SupportingMapFileExts for MapFileExt {}
impl SupportingFileExts for MapFileExt {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["osm.pbf", "pbf", "fmi"]
    }
}

pub trait SupportingMapFileExts: SupportingFileExts {
    fn from_path<P: AsRef<Path> + ?Sized>(path: &P) -> err::Result<MapFileExt> {
        match Self::find_supported_ext(path)? {
            "osm.pbf" | "pbf" => Ok(MapFileExt::PBF),
            "fmi" => Ok(MapFileExt::FMI),
            _ => Err(err::Msg::from(
                "Should not happen, since 'find_supported_ext(...)' should cover this.",
            )),
        }
    }
}
