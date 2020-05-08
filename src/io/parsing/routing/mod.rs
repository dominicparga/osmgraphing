use crate::{
    configs,
    io::SupportingFileExts,
    network::{Graph, NodeIdx},
};
use log::info;

pub mod routes;

pub struct Parser;

impl Parser {
    pub fn parse(cfg: &configs::routing::Config) -> Result<Vec<(i64, i64, usize)>, String> {
        let routes_file = cfg.routes_file.as_ref().expect("No routes-file specified.");

        match Parser::find_supported_ext(routes_file) {
            Ok(_) => routes::Parser::new().parse(cfg),
            Err(msg) => Err(format!("Wrong parser-routes-file: {}", msg)),
        }
    }

    pub fn parse_and_finalize(
        cfg: &configs::routing::Config,
        graph: &Graph,
    ) -> Result<Vec<(NodeIdx, NodeIdx, usize)>, String> {
        let routes_file = cfg.routes_file.as_ref().expect("No routes-file specified.");

        match Parser::find_supported_ext(routes_file) {
            Ok(_) => routes::Parser::new().parse_and_finalize(cfg, graph),
            Err(msg) => Err(format!("Wrong parser-routes-file: {}", msg)),
        }
    }
}

impl SupportingFileExts for Parser {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["route-pairs"]
    }
}

trait Parsing {
    fn preprocess(&mut self, cfg: &configs::routing::Config) -> Result<(), String> {
        let routes_file = cfg.routes_file.as_ref().expect("No routes-file specified.");

        match Parser::find_supported_ext(routes_file) {
            Ok(_) => (),
            Err(msg) => return Err(format!("Wrong routes-file in parser: {}", msg)),
        }

        Ok(())
    }

    fn parse_routes(
        &self,
        cfg: &configs::routing::Config,
    ) -> Result<Vec<(i64, i64, usize)>, String>;

    fn parse(&mut self, cfg: &configs::routing::Config) -> Result<Vec<(i64, i64, usize)>, String> {
        info!("START Process given file");
        self.preprocess(cfg)?;
        let routes = self.parse_routes(cfg)?;
        info!("FINISHED");

        Ok(routes)
    }

    fn parse_and_finalize(
        &mut self,
        cfg: &configs::routing::Config,
        graph: &Graph,
    ) -> Result<Vec<(NodeIdx, NodeIdx, usize)>, String> {
        let routes = self.parse(cfg)?;

        let nodes = graph.nodes();
        Ok(routes
            .into_iter()
            .map(|(src_id, dst_id, n)| {
                let src_idx = nodes.idx_from(src_id).expect(&format!(
                    "Route-file contains src-id {}, which is not part of the graph.",
                    src_id
                ));
                let dst_idx = nodes.idx_from(dst_id).expect(&format!(
                    "Route-file contains dst-id {}, which is not part of the graph.",
                    dst_id
                ));
                (src_idx, dst_idx, n)
            })
            .collect())
    }
}
