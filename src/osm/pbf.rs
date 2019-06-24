use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use crate::routing;
use routing::Graph;

mod pbf {
    pub use osmpbfreader::reader::Iter;
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::{OsmObj, OsmPbfReader, RelationId};
}

//--------------------------------------------------------------------------------------------------

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        //------------------------------------------------------------------------------------------
        // get reader

        let path = Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        let mut reader = pbf::Reader::new(file);

        //------------------------------------------------------------------------------------------
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
                osmpbfreader::OsmObj::Node(node) => println!("{:?}", node),
                osmpbfreader::OsmObj::Way(way) => println!("{:?}", way),
                osmpbfreader::OsmObj::Relation(rel) => println!("{:?}", rel),
            }
        }

        unimplemented!()
    }
}
