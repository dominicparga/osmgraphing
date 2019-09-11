pub mod fmi;
pub mod pbf;

//------------------------------------------------------------------------------------------------//

use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use log::info;

use crate::network::{Graph, GraphBuilder};

//------------------------------------------------------------------------------------------------//

trait Parsing {
    fn open_file<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<File, String> {
        let path = Path::new(&path);
        match File::open(&path) {
            Ok(file) => Ok(file),
            Err(_) => Err(format!("No such file {:?}", path)),
        }
    }

    fn parse_ways(file: File, graph_builder: &mut GraphBuilder);

    fn parse_nodes(file: File, graph_builder: &mut GraphBuilder);

    fn parse<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Graph, String> {
        info!("Starting parsing given path {:?} ..", &Path::new(&path));

        // TODO parse "cycleway" and others
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let mut graph_builder = GraphBuilder::new();

        info!("Starting processing given pbf-file ..");
        match Self::open_file(&path) {
            Ok(file) => Self::parse_ways(file, &mut graph_builder),
            Err(msg) => return Err(msg),
        }
        match Self::open_file(&path) {
            Ok(file) => Self::parse_nodes(file, &mut graph_builder),
            Err(msg) => return Err(msg),
        }
        info!("Finished processing given pbf-file");

        let result = graph_builder.finalize();
        info!("Finished parsing");
        result
    }
}

//------------------------------------------------------------------------------------------------//

enum Type {
    PBF,
    FMI,
}
impl Type {
    fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Self, String> {
        let supported_exts = &["pbf", "fmi"];
        let path = Path::new(&path);

        // if file has extension
        if let Some(os_str) = path.extension() {
            // if filename is valid unicode
            if let Some(extension) = os_str.to_str() {
                // check if parser supports extension
                match extension.to_ascii_lowercase().as_ref() {
                    "pbf" => Ok(Type::PBF),
                    "fmi" => Ok(Type::FMI),
                    // parser doesn't support this extension
                    unsupported_ext => Err(format!(
                        "Unsupported extension `{}` was given. Supported extensions are {:?}",
                        unsupported_ext, supported_exts
                    )),
                }
            } else {
                Err(String::from("Filename is invalid Unicode."))
            }
        } else {
            Err(format!(
                "The file {:?} has no extension. Supported extensions are {:?}",
                &path, supported_exts
            ))
        }
    }
}

pub struct Parser;
impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Graph, String> {
        match Type::from_path(path) {
            Ok(Type::PBF) => pbf::Parser::parse(path),
            Ok(Type::FMI) => fmi::Parser::parse(path),
            Err(msg) => Err(msg),
        }
    }
}
