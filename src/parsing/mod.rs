pub mod fmi;
pub mod pbf;

use crate::network::{Graph, GraphBuilder};
use log::info;
use std::{fs::File, path::Path};

//------------------------------------------------------------------------------------------------//

/// The parser parsing `*.osm.pbf`- and `*.fmi`-files into a graphbuilder or a graph.
///
///
/// ## The filter-pipeline
///
/// 1. Download raw osm-data (see [README](https://github.com/dominicparga/osmgraphing/blob/master/README.md))
/// 1. Read in this data
/// 1. Filter and process osm-components like nodes and edges, e.g. filtering via tags
/// 1. Create a memory- and runtime-efficient routing-graph.
///
///
/// ### Nodes
///
/// - Coordinates:
///   Nodes have coordinates given in `(latitude, longitude)`.
/// - Height: Nodes have a height, which is ignored right now.
///
///
/// ### Edges
///
/// Every edge will have a street-type with respective default speed-limit.
/// These defaults depend on the street-network and can be found in the respective module `network`.
///
///
/// ## Additional information
///
/// This `pbf`-parser uses [osmpbfreader-rs](https://crates.io/crates/osmpbfreader).
/// An own implementation would need [the pbf-impl of rust](https://github.com/stepancheg/rust-protobuf), but the previously mentioned osmpbfreader works well.
/// `*.osm`-xml-files are not supported, but could be read with [quick-xml](https://github.com/tafia/quick-xml).
///
/// Other libraries processing openstreetmap-data can be found [in the osm-wiki](https://wiki.openstreetmap.org/wiki/Frameworks#Data_Processing_or_Parsing_Libraries).
pub struct Parser;
impl Parser {
    pub fn parse<P: AsRef<Path> + ?Sized>(path: &P) -> Result<GraphBuilder, String> {
        match Type::from_path(path)? {
            Type::PBF => pbf::Parser::parse(path),
            Type::FMI => fmi::Parser::parse(path),
        }
    }

    pub fn parse_and_finalize<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Graph, String> {
        match Type::from_path(path)? {
            Type::PBF => pbf::Parser::parse_and_finalize(path),
            Type::FMI => fmi::Parser::parse_and_finalize(path),
        }
    }
}

trait Parsing {
    fn open_file<P: AsRef<Path> + ?Sized>(path: &P) -> Result<File, String> {
        let path = path.as_ref();
        match File::open(path) {
            Ok(file) => Ok(file),
            Err(_) => Err(format!("No such file {}", path.display())),
        }
    }

    fn parse_ways(file: File, graph_builder: &mut GraphBuilder);

    fn parse_nodes(file: File, graph_builder: &mut GraphBuilder);

    fn parse<P: AsRef<Path> + ?Sized>(path: &P) -> Result<GraphBuilder, String> {
        let mut graph_builder = GraphBuilder::new();

        info!("START Process given file");
        let file = Self::open_file(path)?;
        Self::parse_ways(file, &mut graph_builder);
        let file = Self::open_file(path)?;
        Self::parse_nodes(file, &mut graph_builder);
        info!("FINISHED");

        Ok(graph_builder)
    }

    fn parse_and_finalize<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Graph, String> {
        info!("START Parse given path {}", path.as_ref().display());

        // TODO parse "cycleway" and others
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let result = Self::parse(path)?.finalize();
        info!("FINISHED");
        result
    }
}

//------------------------------------------------------------------------------------------------//

enum Type {
    PBF,
    FMI,
}
impl Type {
    fn from_path<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Self, String> {
        let supported_exts = &["pbf", "fmi"];
        let path = path.as_ref();

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
