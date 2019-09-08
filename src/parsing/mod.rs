pub mod fmi;
pub mod pbf;

//------------------------------------------------------------------------------------------------//

use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use log::{error, info};

use crate::network::{Graph, GraphBuilder};

//------------------------------------------------------------------------------------------------//

pub trait Parsing {
    fn open_file<S: AsRef<OsStr> + ?Sized>(path: &S) -> File {
        let path = Path::new(&path);
        File::open(&path).expect(&format!("File {:?} not found.", &path))
    }

    fn parse_ways<S: AsRef<OsStr> + ?Sized>(path: &S, graph_builder: &mut GraphBuilder);

    fn parse_nodes<S: AsRef<OsStr> + ?Sized>(path: &S, graph_builder: &mut GraphBuilder);

    fn parse<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Graph, String> {
        info!("Starting parsing ..");

        // TODO parse "cycleway" and others
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let mut graph_builder = GraphBuilder::new();

        info!("Starting processing given pbf-file ..");
        Self::parse_ways(&path, &mut graph_builder);
        Self::parse_nodes(&path, &mut graph_builder);
        info!("Finished processing given pbf-file");

        let graph = graph_builder.finalize();
        info!("Finished parsing");
        Ok(graph)
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
            Err(format!("The file `{:?}` has no extension.", &path))
        }
    }
}

pub struct Parser;
impl Parsing for Parser {
    fn parse_ways<S: AsRef<OsStr> + ?Sized>(path: &S, graph_builder: &mut GraphBuilder) {
        match Type::from_path(path) {
            Ok(Type::PBF) => pbf::Parser::parse_ways(path, graph_builder),
            Ok(Type::FMI) => fmi::Parser::parse_ways(path, graph_builder),
            Err(msg) => error!("{}", msg),
        }
    }

    fn parse_nodes<S: AsRef<OsStr> + ?Sized>(path: &S, graph_builder: &mut GraphBuilder) {
        match Type::from_path(path) {
            Ok(Type::PBF) => pbf::Parser::parse_nodes(path, graph_builder),
            Ok(Type::FMI) => fmi::Parser::parse_nodes(path, graph_builder),
            Err(msg) => error!("{}", msg),
        }
    }

    fn parse<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<Graph, String> {
        match Type::from_path(path) {
            Ok(Type::PBF) => pbf::Parser::parse(path),
            Ok(Type::FMI) => fmi::Parser::parse(path),
            Err(msg) => Err(msg),
        }
    }
}
