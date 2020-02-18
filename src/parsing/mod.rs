pub mod fmi;
pub mod pbf;

use crate::{
    configs::graph,
    network::{Graph, GraphBuilder},
};
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
    pub fn parse(cfg: &graph::Config) -> Result<GraphBuilder, String> {
        match Type::from_path(&cfg.paths.map_file)? {
            Type::PBF => pbf::Parser::new().parse(cfg),
            Type::FMI => fmi::Parser::new().parse(cfg),
        }
    }

    pub fn parse_and_finalize(cfg: &graph::Config) -> Result<Graph, String> {
        match Type::from_path(&cfg.paths.map_file)? {
            Type::PBF => pbf::Parser::new().parse_and_finalize(cfg),
            Type::FMI => fmi::Parser::new().parse_and_finalize(cfg),
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

    fn preprocess(&mut self, _file: File) -> Result<(), String> {
        Ok(())
    }

    fn parse(&mut self, cfg: &graph::Config) -> Result<GraphBuilder, String> {
        let mut graph_builder = GraphBuilder::new();
        let path = &cfg.paths.map_file;

        info!("START Process given file");
        let file = Self::open_file(path)?;
        self.preprocess(file)?;
        let file = Self::open_file(path)?;
        self.parse_ways(file, &mut graph_builder, cfg)?;
        let file = Self::open_file(path)?;
        self.parse_nodes(file, &mut graph_builder, cfg)?;
        info!("FINISHED");

        Ok(graph_builder)
    }

    fn parse_ways(
        &self,
        file: File,
        graph_builder: &mut GraphBuilder,
        cfg: &graph::Config,
    ) -> Result<(), String>;

    fn parse_nodes(
        &self,
        file: File,
        graph_builder: &mut GraphBuilder,
        cfg: &graph::Config,
    ) -> Result<(), String>;

    fn parse_and_finalize(&mut self, cfg: &graph::Config) -> Result<Graph, String> {
        let path = Path::new(&cfg.paths.map_file);
        info!("START Parse given path {}", path.display());

        // TODO parse "cycleway" and others
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let result = self.parse(cfg)?.finalize(cfg);
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
