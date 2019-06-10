use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use quick_xml::Reader as XmlReader;

use crate::err::Error;

pub struct Reader(XmlReader<BufReader<File>>);

impl Reader {
    pub fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Self, Error> {
        let path = Path::new(&path);
        Ok(Reader(XmlReader::from_file(&path)?))
    }
}
