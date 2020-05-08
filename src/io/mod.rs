mod parsing;
mod writing;

pub mod network {
    pub use crate::io::parsing::network::Parser;
    pub use crate::io::writing::network::Writer;
}
pub mod routing {
    pub use crate::io::parsing::routing::Parser;
    pub use crate::io::writing::routing::Writer;
}

use std::path::Path;

pub trait SupportingFileExts {
    fn supported_exts<'a>() -> &'a [&'a str];

    fn find_supported_ext<'a, P: AsRef<Path> + ?Sized>(path: &P) -> Result<&'a str, String> {
        let supported_exts = Self::supported_exts();
        let path = path.as_ref();

        // if file has extension
        if let Some(os_str) = path.extension() {
            // if filename is valid unicode
            if let Some(extension) = os_str.to_str() {
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

pub trait SupportingMapFileExts: SupportingFileExts {
    fn from_path<P: AsRef<Path> + ?Sized>(path: &P) -> Result<MapFileExt, String> {
        match Self::find_supported_ext(path)? {
            "pbf" => Ok(MapFileExt::PBF),
            "fmi" => Ok(MapFileExt::FMI),
            _ => Err(String::from(
                "Should not happen, since 'find_supported_ext(...)' should cover this.",
            )),
        }
    }
}
