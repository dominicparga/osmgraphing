use std::ffi::OsStr;

use super::Reader;

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) {
        match Reader::from_path(&path) {
            Ok(mut reader) => reader.stuff(),
            Err(e) => println!("{}", e),
        }
    }
}
