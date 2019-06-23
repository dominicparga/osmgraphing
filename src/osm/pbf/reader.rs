use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use crate::err::Error;

//--------------------------------------------------------------------------------------------------

mod pbf {
    pub use osmpbfreader::reader::Iter;
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::{OsmObj, OsmPbfReader, RelationId};
}
mod xml {
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use quick_xml::Reader as XmlReader;

    use crate::err::Error;

    pub struct Reader(XmlReader<BufReader<File>>);

    impl Reader {
        pub fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Self, Error> {
            let path = Path::new(&path);
            Ok(Reader(XmlReader::from_file(&path)?))
        }
    }
}

//--------------------------------------------------------------------------------------------------

pub enum Reader<R> {
    Pbf(pbf::Reader<R>),
    Xml(xml::Reader),
}

impl<R> Reader<R> {
    const EXT_PBF: &'static str = "pbf";
    const _EXT_XML: &'static str = "osm";

    pub fn supported_exts() -> &'static [&'static str; 1] {
        &[Reader::<R>::EXT_PBF]
    }
}

impl Reader<File> {
    pub fn from_path<S>(path: &S) -> Result<Self, Error>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        // check path
        let path = Path::new(&path);
        match path.extension() {
            // extension exists -> check if supported
            Some(os_str) => match os_str.to_str() {
                // extension: pbf
                Some(Self::EXT_PBF) => {
                    let file = File::open(&path)?;
                    Ok(Reader::Pbf(pbf::Reader::new(file)))
                }
                // unsupported extension
                Some(unsupported_ext) => Err(Error::unsupported_extension(
                    unsupported_ext,
                    Self::supported_exts(),
                )),
                // path is not valid unicode
                None => Err(Error::invalid_unicode()),
            },
            // no extension
            None => Err(Error::unsupported_extension("", Self::supported_exts())),
        }
    }

    pub fn iter(&mut self) -> pbf::Iter<File> {
        if let Reader::Pbf(reader) = self {
            reader.iter()
        } else {
            panic!()
        }
    }
}
