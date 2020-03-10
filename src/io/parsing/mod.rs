pub mod fmi;
pub mod pbf;

use crate::{
    configs::parser,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::{Graph, GraphBuilder},
};
use log::info;
use std::path::Path;

/// The parser parsing `*.osm.pbf`- and `*.fmi`-files into a graphbuilder or a graph.
///
///
/// ## The filter-pipeline
///
/// 1. Download raw osm-data (see [README](https://github.com/dominicparga/osmgraphing/blob/nightly/README.md))
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
    pub fn parse(cfg: &parser::Config) -> Result<GraphBuilder, String> {
        match Parser::from_path(&cfg.map_file)? {
            MapFileExt::PBF => pbf::Parser::new().parse(cfg),
            MapFileExt::FMI => fmi::Parser::new().parse(cfg),
        }
    }

    pub fn parse_and_finalize(cfg: parser::Config) -> Result<Graph, String> {
        match Parser::from_path(&cfg.map_file)? {
            MapFileExt::PBF => pbf::Parser::new().parse_and_finalize(cfg),
            MapFileExt::FMI => fmi::Parser::new().parse_and_finalize(cfg),
        }
    }
}

impl SupportingMapFileExts for Parser {}
impl SupportingFileExts for Parser {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["pbf", "fmi"]
    }
}

trait Parsing {
    fn preprocess(&mut self, _cfg: &parser::Config) -> Result<(), String> {
        Ok(())
    }

    fn parse(&mut self, cfg: &parser::Config) -> Result<GraphBuilder, String> {
        let mut graph_builder = GraphBuilder::new();

        info!("START Process given file");
        self.preprocess(cfg)?;
        self.parse_ways(cfg, &mut graph_builder)?;
        self.parse_nodes(cfg, &mut graph_builder)?;
        info!("FINISHED");

        Ok(graph_builder)
    }

    fn parse_ways(
        &self,
        cfg: &parser::Config,
        graph_builder: &mut GraphBuilder,
    ) -> Result<(), String>;

    fn parse_nodes(
        &self,
        cfg: &parser::Config,
        graph_builder: &mut GraphBuilder,
    ) -> Result<(), String>;

    fn parse_and_finalize(&mut self, cfg: parser::Config) -> Result<Graph, String> {
        let path = Path::new(&cfg.map_file);
        info!("START Parse from given path {}", path.display());

        // TODO parse "cycleway" and others
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let result = self.parse(&cfg)?.finalize(cfg);
        info!("FINISHED");
        result
    }
}
