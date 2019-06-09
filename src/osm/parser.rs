use std::ffi::OsStr;

use super::Reader;

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) {
        let mut reader = Reader::from_path(&path).unwrap();
        reader.stuff();
    }
}
