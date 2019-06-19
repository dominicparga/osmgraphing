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

        fn filter(obj: &osmpbfreader::OsmObj) -> bool {
            obj.tags();
            obj.id();
            obj.id() != osmpbfreader::RelationId(7444).into() // id of relation for Paris
        }

        // let filter = |_obj: &osmpbfreader::OsmObj| {
        //     _obj.tags();
        //     true
        // };
        for obj in reader.iter().map(Result::unwrap) {
            if !filter(&obj) {
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
