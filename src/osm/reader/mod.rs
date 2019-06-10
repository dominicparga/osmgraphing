use std::ffi::OsStr;
use std::path::Path;

mod pbf;
use crate::err::Error;

pub enum Reader {
    Pbf(pbf::Reader),
    // Xml,
}

impl Reader {
    const EXT_PBF: &'static str = "pbf";
    // const EXT_XML: &'static str = "osm";

    pub fn from_path<S>(path: &S) -> Result<Self, Error> where
        S: AsRef<OsStr> + ?Sized,
    {
        let supp_exts = [Reader::EXT_PBF];

        // check path
        let path = Path::new(&path);
        match path.extension() {
            // extension exists -> check if supported
            Some(os_str) => match os_str.to_str() {
                // extension: pbf
                Some(Reader::EXT_PBF) => match pbf::Reader::from_path(&path) {
                    Ok(r)  => Ok(Reader::Pbf(r)),
                    Err(e) => Err(Error::Io(e)),
                },
                // unsupported extension: osm and others
                Some(unsupp_ext) => Err(Error::from_unsupported_extension(unsupp_ext, &supp_exts)),
                None             => Err(Error::from_invalid_unicode()),
            }
            // no extension
            None => Err(Error::from_unsupported_extension("", &supp_exts)),
        }
    }

    pub fn stuff(&mut self) {
        match self {
            Reader::Pbf(r) => r.stuff(),
            // Reader::Xml    => unimplemented!(),
        }
    }
}
