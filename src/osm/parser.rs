use std::ffi::OsStr;
use std::path::Path;

use super::Read;

pub struct Parser {

}

impl super::Parse for Parser {
    fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) {
        //----------------------------------------------------------------------------------------//
        // supported file extensions

        const EXT_PBF: &str = "pbf";
        // const EXT_XML: &str = "xml";

        fn unsupported_ext_found(unsupp_ext: Option<&str>) {
            if let Some(unsupp_ext) = unsupp_ext {
                println!("Unsupported extension '{}' were given!", unsupp_ext);
            };
            println!("Please insert a path to a valid osm-file. Supported extensions are: {}",
                EXT_PBF
            );
        }

        //----------------------------------------------------------------------------------------//
        // process given path

        // check path
        let path = Path::new(&path);
        let reader = match path.extension() {
            None => {
                unsupported_ext_found(None);
                None
            },
            Some(os_str) => {
                match os_str.to_str() {
                    Some(EXT_PBF) => {
                        Some(super::pbf::Reader::from_path(path))
                    },
                    unsupp_ext => {
                        unsupported_ext_found(unsupp_ext);
                        None
                    },
                }
            }
        };
        if let Some(mut reader) = reader {
            reader.stuff();
        }
    }
}
