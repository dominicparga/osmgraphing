use std::ffi::OsStr;

use super::Reader;

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) {
        //----------------------------------------------------------------------------------------//
        // get reader

        let mut reader;
        match Reader::from_path(&path) {
            Err(e) => {
                println!("{}", e);
                return ()
            },
            Ok(r) => reader = r,
        }

        //----------------------------------------------------------------------------------------//
        // filter

        reader.stuff();
    }
}
