use std::ffi::OsStr;
use std::path::Path;

use crate::routing;
use routing::Graph;

mod xml {
    pub use quick_xml::Reader;
}

//------------------------------------------------------------------------------------------------//

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        //----------------------------------------------------------------------------------------//
        // get reader

        let path = Path::new(&path);
        let mut _reader = xml::Reader::from_file(&path);

        //----------------------------------------------------------------------------------------//
        // filter

        unimplemented!()
    }
}
