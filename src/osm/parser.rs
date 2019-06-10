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

        let filter = |_| true;
        for obj in reader.iter().map(Result::unwrap) {
            if !filter(obj.tags()) {
                continue;
            }
            match obj {
                osmpbfreader::OsmObj::Node(node) => {
                    println!("{:?}", node)
                }
                osmpbfreader::OsmObj::Way(way) => {
                    println!("{:?}", way)
                }
                osmpbfreader::OsmObj::Relation(rel) => {
                    println!("{:?}", rel)
                }
            }
        }
    }
}
