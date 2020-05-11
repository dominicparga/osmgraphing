use crate::{configs, helpers};
use log::info;
use std::{
    io::{BufRead, BufReader},
    ops::Range,
};

pub struct Parser {
    route_lines: Range<usize>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { route_lines: 1..0 }
    }

    fn is_line_functional(line: &String) -> bool {
        line.len() > 0 && line.chars().next() != Some('#')
    }
}

impl super::Parsing for Parser {
    fn preprocess(&mut self, cfg: &configs::routing::Config) -> Result<(), String> {
        info!("START Start preprocessing routes-parser.");

        let route_pairs_file = cfg
            .route_pairs_file
            .as_ref()
            .expect("No routes-file specified.");

        // only functional-lines are counted
        let mut line_number = 0;
        let mut is_taking_counts = false;
        // counts are only metric-count, node-count, edge-count (in this order)
        let mut counts = vec![];
        let file = helpers::open_file(route_pairs_file)?;
        for line in BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            // If there is a count, remember it.
            // The first occuring count let `is_taking_counts` getting true.
            // If all counts have been processed, `is_taking_counts` would change to false,
            // where the loop should stop and remember the line-number.
            let params: Vec<&str> = line.split_whitespace().collect();
            if params.len() == 1 {
                is_taking_counts = true;

                if let Ok(count) = params[0].parse::<usize>() {
                    counts.push(count);
                }
            } else if is_taking_counts {
                break;
            }

            line_number += 1;
        }

        // add counts
        if counts.len() < 1 {
            return Err(format!(
                "The provided routes-file doesn't have the routes-count."
            ));
        }

        // Current state: Last line-number is first route-line.
        let routes_count = counts.pop().unwrap();

        let start = line_number;
        let end = start + routes_count;
        self.route_lines = start..end;

        info!("FINISHED");
        Ok(())
    }

    fn parse_routes(
        &self,
        cfg: &configs::routing::Config,
    ) -> Result<Vec<(i64, i64, usize)>, String> {
        info!("START Create routes from input-file.");

        let mut route_pairs = Vec::with_capacity(self.route_lines.len());
        let route_pairs_file = cfg
            .route_pairs_file
            .as_ref()
            .expect("No routes-file specified.");

        let mut line_number = 0;
        let file = helpers::open_file(route_pairs_file)?;
        for line in BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(Self::is_line_functional)
        {
            // check if line contains route
            if !self.route_lines.contains(&line_number) {
                line_number += 1;
                continue;
            }
            line_number += 1;

            // create route

            let params: Vec<&str> = line.split_whitespace().collect();
            if params.len() != 3 {
                return Err(format!(
                    "A route-line is expected to consist of (src-id, dst-id, count), \
                     but {} values are provided.",
                    params.len()
                ));
            }

            let param = params[0];
            let src_id = param
                .parse::<i64>()
                .ok()
                .ok_or(format!("Could not parse route's src-id {}", param))?;
            let param = params[1];
            let dst_id = param
                .parse::<i64>()
                .ok()
                .ok_or(format!("Could not parse route's dst-id {}", param))?;
            let param = params[2];
            let n = param
                .parse::<usize>()
                .ok()
                .ok_or(format!("Could not parse route's count {}", param))?;

            route_pairs.push((src_id, dst_id, n));
        }

        info!("FINISHED");
        Ok(route_pairs)
    }
}
