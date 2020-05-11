use crate::{configs, io::SupportingFileExts, network::RoutePair};
use log::info;

mod routes;

pub struct Parser;

impl Parser {
    pub fn parse(cfg: &configs::routing::Config) -> Result<Vec<(RoutePair<i64>, usize)>, String> {
        let route_pairs_file = cfg
            .route_pairs_file
            .as_ref()
            .expect("No routes-file specified.");

        match Parser::find_supported_ext(route_pairs_file) {
            Ok(_) => routes::Parser::new().parse(cfg),
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
        let route_pairs_file = cfg
            .route_pairs_file
            .as_ref()
            .expect("No routes-file specified.");

        match Parser::find_supported_ext(route_pairs_file) {
            Ok(_) => (),
            Err(msg) => return Err(format!("Wrong routes-file in parser: {}", msg)),
        }

        Ok(())
    }

    fn parse_route_pairs(
        &self,
        cfg: &configs::routing::Config,
    ) -> Result<Vec<(RoutePair<i64>, usize)>, String>;

    fn parse(
        &mut self,
        cfg: &configs::routing::Config,
    ) -> Result<Vec<(RoutePair<i64>, usize)>, String> {
        info!("START Process given file");
        self.preprocess(cfg)?;
        let routes = self.parse_route_pairs(cfg)?;
        info!("FINISHED");

        Ok(routes)
    }
}
