use std::ffi::OsStr;
use std::path::Path;

mod pbf;
mod xml;
use crate::err::Error;

pub enum Reader {
    Pbf(pbf::Reader),
    Xml(xml::Reader),
}

impl Reader {
    const EXT_PBF: &'static str = "pbf";
    const EXT_XML: &'static str = "osm";

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
                Some(Reader::EXT_PBF) => Ok(Reader::Pbf(pbf::Reader::from_path(&path)?)),
                // extension: osm
                Some(Reader::EXT_XML) => Ok(Reader::Xml(xml::Reader::from_path(&path)?)),
                // unsupported extension: others/unknown
                Some(unsupp_ext) => Err(Error::unsupported_extension(unsupp_ext, &supp_exts)),
                // path is not valid unicode
                None => Err(Error::invalid_unicode()),
            }
            // no extension
            None => Err(Error::unsupported_extension("", &supp_exts)),
        }
    }

    pub fn next() {
        unimplemented!("Iterator functionality is not implemented yet")
    }

    pub fn stuff(&mut self) {
        match self {
            Reader::Pbf(r) => r.stuff(),
            Reader::Xml(_) => unimplemented!(),
        }
    }
}
