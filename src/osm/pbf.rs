use std::ffi::{OsString,OsStr};
use std::fs::File;
use std::path::Path;

use osmpbfreader::{OsmPbfReader,OsmObj, RelationId};

pub struct Reader {
    pbf: OsmPbfReader<File>,
}

impl Reader {
    pub fn stuff(&mut self) {
        fn wanted(obj: &OsmObj) -> bool {
            obj.id() == RelationId(7444).into() //id of relation for Paris
        }

        let objects = self.pbf.get_objs_and_deps(wanted).unwrap();
        println!(
            "The relation Paris is composed of {:?} items",
            objects.len()
        );
        for (id, _) in objects {
            println!("{:?}", id);
        }
    }
}

impl super::Read for Reader {
    fn from_str(s: &str) -> Reader {
        return Self::from_os_str(&OsString::from(s));
    }

    fn from_os_str<S: AsRef<OsStr> + ?Sized>(path: &S) -> Reader {
        let path = Path::new(&path);
        let file = File::open(&path).unwrap();
        return Reader { pbf: OsmPbfReader::new(file) };
    }
}
