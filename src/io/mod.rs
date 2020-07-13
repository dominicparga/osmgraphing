mod parsing;
mod writing;

pub mod network {
    pub mod graph {
        pub use crate::io::parsing::network::graph::Parser;
        pub use crate::io::writing::network::graph::Writer;
    }
    pub mod edges {
        pub use crate::io::writing::network::edges::Writer;
    }
}
pub mod routing {
    pub use crate::io::parsing::routing::Parser;
    pub use crate::io::writing::routing::Writer;
}
pub mod balancing {
    pub use crate::io::writing::balancing::Writer;
}

use std::path::Path;

pub fn ext_from<P: AsRef<Path> + ?Sized>(path: &P) -> Result<&str, String> {
    let path = path.as_ref();

    // if file has extension
    if let Some(os_str) = path.extension() {
        // if filename is valid unicode
        if let Some(extension) = os_str.to_str() {
            Ok(extension)
        } else {
            Err(String::from("Filename is invalid Unicode."))
        }
    } else {
        Err(format!("The file {:?} has no extension.", &path,))
    }
}

pub trait SupportingFileExts {
    fn ext_from<P: AsRef<Path> + ?Sized>(path: &P) -> Result<&str, String> {
        ext_from(path)
    }

    fn supported_exts<'a>() -> &'a [&'a str];

    fn find_supported_ext<'a, P: AsRef<Path> + ?Sized>(path: &P) -> Result<&'a str, String> {
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
                Err(format!(
                    "Unsupported extension `{}` was given. Supported extensions are {:?}",
                    extension, supported_exts
                ))
            }
            Err(msg) => Err(format!(
                "{} Supported extensions are {:?}",
                msg, supported_exts
            )),
        }
    }

    fn is_file_supported<P: AsRef<Path> + ?Sized>(path: &P) -> bool {
        Self::find_supported_ext(path).is_ok()
    }

    fn is_ext_supported(ext: &str) -> bool {
        Self::find_supported_ext(ext).is_ok()
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
    fn from_path<P: AsRef<Path> + ?Sized>(path: &P) -> Result<MapFileExt, String> {
        match Self::find_supported_ext(path)? {
            "osm.pbf" | "pbf" => Ok(MapFileExt::PBF),
            "fmi" => Ok(MapFileExt::FMI),
            _ => Err(String::from(
                "Should not happen, since 'find_supported_ext(...)' should cover this.",
            )),
        }
    }
}
