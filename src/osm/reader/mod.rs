use std::ffi::OsStr;
use std::path::Path;

mod pbf;

pub enum Reader {
    Pbf(pbf::Reader),
    Xml,
}

impl Reader {
    const EXT_PBF: &'static str = "pbf";
    const EXT_XML: &'static str = "osm";

    pub fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Self, String> {
        fn unsupported_ext_found(unsupp_ext: Option<&str>) {
            if let Some(unsupp_ext) = unsupp_ext {
                println!("Unsupported extension '{}' were given!", unsupp_ext);
            };
            println!("Please insert a path to a valid osm-file. Supported extensions are: {}",
                Reader::EXT_PBF
            );
        }

        // check path
        let path = Path::new(&path);
        match path.extension() {
            None => Err(String::from("nothing")),
            // if file has extension => compare if it's supported
            Some(os_str) => match os_str.to_str() {
                Some(Reader::EXT_PBF) => match pbf::Reader::from_path(&path) {
                    Ok(r)  => Ok(Reader::Pbf(r)),
                    Err(_) => Err(String::from("fu")),
                },
                Some(Reader::EXT_XML) => unimplemented!("Unimplemented, yuck! TODO"),
                unsupp_ext            => {
                    unsupported_ext_found(unsupp_ext);
                    Err(String::from("Oh no some error"))
                },
            }
        }
    }

    pub fn stuff(&mut self) {
        match self {
            Reader::Pbf(r) => r.stuff(),
            Reader::Xml    => unimplemented!(),
        }
    }
}
