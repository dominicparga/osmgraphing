pub mod fmi;
pub mod pbf;

use crate::{
    configs::parsing::{self, generating},
    defaults::capacity,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::{EdgeBuilder, Graph, GraphBuilder, NodeBuilder},
};
use log::{info, warn};
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
    pub fn parse(cfg: parsing::Config) -> Result<GraphBuilder, String> {
        match Parser::from_path(&cfg.map_file)? {
            MapFileExt::PBF => pbf::Parser::new().parse(cfg),
            MapFileExt::FMI => fmi::Parser::new().parse(cfg),
        }
    }

    pub fn parse_and_finalize(cfg: parsing::Config) -> Result<Graph, String> {
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
    fn preprocess(&mut self, cfg: &parsing::Config) -> Result<(), String> {
        check_config(cfg)
    }

    fn parse(&mut self, cfg: parsing::Config) -> Result<GraphBuilder, String> {
        let mut builder = GraphBuilder::new(cfg);

        info!("START Process given file");
        self.preprocess(builder.cfg())?;
        self.parse_ways(&mut builder)?;
        let mut builder = builder.next();
        self.parse_nodes(&mut builder)?;
        let builder = builder.next();
        info!("FINISHED");

        builder
    }

    fn parse_ways(&self, builder: &mut EdgeBuilder) -> Result<(), String>;

    fn parse_nodes(&self, builder: &mut NodeBuilder) -> Result<(), String>;

    fn parse_and_finalize(&mut self, cfg: parsing::Config) -> Result<Graph, String> {
        let path = Path::new(&cfg.map_file);
        info!("START Parse from given path {}", path.display());

        // TODO parse "cycleway" and other tags
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let result = self.parse(cfg)?.finalize();
        info!("FINISHED");
        result
    }
}

/// check if yaml-config is correct
fn check_config(cfg: &parsing::Config) -> Result<(), String> {
    // check nodes

    // is NodeId in config?
    if !cfg.nodes.categories.iter().any(|category| match category {
        parsing::nodes::Category::Meta { info, id: _ } => info == &parsing::nodes::MetaInfo::NodeId,
        parsing::nodes::Category::Metric { unit: _, id: _ } | parsing::nodes::Category::Ignored => {
            false
        }
    }) {
        return Err(String::from(
            "The provided config-file doesn't contain a NodeId, but needs to.",
        ));
    }

    // check nodes' coordinates

    if !cfg.nodes.categories.iter().any(|category| match category {
        parsing::nodes::Category::Metric { unit, id: _ } => {
            unit == &parsing::nodes::metrics::UnitInfo::Latitude
        }
        parsing::nodes::Category::Meta { info: _, id: _ } | parsing::nodes::Category::Ignored => {
            false
        }
    }) {
        return Err(String::from(
            "The provided config-file doesn't contain a latitude, but needs to.",
        ));
    }

    if !cfg.nodes.categories.iter().any(|category| match category {
        parsing::nodes::Category::Metric { unit, id: _ } => {
            unit == &parsing::nodes::metrics::UnitInfo::Longitude
        }
        parsing::nodes::Category::Meta { info: _, id: _ } | parsing::nodes::Category::Ignored => {
            false
        }
    }) {
        return Err(String::from(
            "The provided config-file doesn't contain a longitude, but needs to.",
        ));
    }

    // check edges' metric-memory-capacity

    let dim = cfg.edges.metrics.units.len()
        + if let Some(generating_cfg) = &cfg.generating {
            generating_cfg
                .edges
                .categories
                .iter()
                .filter(|category| match category {
                    generating::edges::Category::Meta { info: _, id: _ } => false,
                    generating::edges::Category::Convert { from: _, to: _ } => false,
                    generating::edges::Category::Calc {
                        result: _,
                        a: _,
                        b: _,
                    } => true,
                    generating::edges::Category::Copy { from: _, to: _ } => true,
                    generating::edges::Category::Haversine { unit: _, id: _ } => true,
                })
                .count()
        } else {
            0
        };

    if dim > capacity::SMALL_VEC_INLINE_SIZE {
        return Err(format!(
            "The provided config-file has more metrics for the graph ({}) \
             than the parser has been compiled to ({}).",
            dim,
            capacity::SMALL_VEC_INLINE_SIZE
        ));
    } else if dim < capacity::SMALL_VEC_INLINE_SIZE {
        warn!(
            "The provided config-file has less metrics for the graph ({}) \
             than the parser has been compiled to ({}). \
             Compiling accordingly saves memory.",
            dim,
            capacity::SMALL_VEC_INLINE_SIZE
        );
    }

    // check count of shortcut-edge-indices

    let count =
        cfg.edges
            .categories
            .iter()
            .filter(|category| match category {
                parsing::edges::Category::Meta { info, id: _ } => match info {
                    parsing::edges::MetaInfo::ShortcutIdx0
                    | parsing::edges::MetaInfo::ShortcutIdx1 => true,
                    parsing::edges::MetaInfo::SrcId
                    | parsing::edges::MetaInfo::SrcIdx
                    | parsing::edges::MetaInfo::DstId
                    | parsing::edges::MetaInfo::DstIdx => false,
                },
                parsing::edges::Category::Metric { unit: _, id: _ }
                | parsing::edges::Category::Ignored => false,
            })
            .count();
    if count > 0 && count != 2 {
        return Err(format!(
            "The config-file has {} shortcut-indices, but should have 0 or 2.",
            count
        ));
    }

    Ok(())
}
