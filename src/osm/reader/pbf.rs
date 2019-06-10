use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::io;

use osmpbfreader::{OsmPbfReader,OsmObj,RelationId};

pub struct Reader {
    pbf: OsmPbfReader<File>,
}

impl Reader {
    pub fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> io::Result<Self> {
        let path = Path::new(&path);
        let file = File::open(&path)?;
        Ok(Reader { pbf: OsmPbfReader::new(file) })
    }

    // TODO: move out of this lib into example file
    pub fn stuff(&mut self) {
        fn wanted(obj: &OsmObj) -> bool {
            obj.id() == RelationId(7444).into() //id of relation for Paris
        }

        let objects = self.pbf.get_objs_and_deps(wanted).unwrap();
        // for _obj in pbf.iter().map(Result::unwrap) {
        println!(
            "The relation Paris is composed of {:?} items",
            objects.len()
        );
        for (id, _) in objects {
            println!("{:?}", id);
        }
    }
}
