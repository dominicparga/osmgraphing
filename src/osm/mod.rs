use std::ffi::OsStr;
use std::path::Path;

use crate::err::FileError;

pub mod fmi;
pub mod pbf;

//--------------------------------------------------------------------------------------------------

pub enum Support {
    PBF, // XML, FMI,
}

impl Support {
    const EXT_PBF: &'static str = "pbf";
    // unsupported yet
    const _EXT_XML: &'static str = "osm";
    const _EXT_FMI: &'static str = "fmi";

    pub fn supported_exts() -> &'static [&'static str; 1] {
        &[Self::EXT_PBF]
    }

    pub fn from_path<S>(path: &S) -> Result<Self, FileError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        // check path
        let path = Path::new(&path);
        match path.extension() {
            // extension exists -> check if supported
            Some(os_str) => match os_str.to_str() {
                Some(Self::EXT_PBF) => Ok(Support::PBF),
                Some(unsupported_ext) => Err(FileError::unsupported_extension(
                    unsupported_ext,
                    Self::supported_exts(),
                )),
                // path is not valid unicode
                None => Err(FileError::invalid_unicode()),
            },
            // no extension
            None => Err(FileError::unsupported_extension("", Self::supported_exts())),
        }
    }
}
