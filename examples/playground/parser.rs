use std::ffi::OsString;

use osmgraphing::osm;

fn main() {
    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => OsString::from("resources/osm/small.fmi"),
    };

    let graph = match osm::Support::from_path(&path) {
        Ok(osm::Support::PBF) => {
            let parser = osm::pbf::Parser;
            parser.parse(&path)
        },
        Ok(osm::Support::FMI) => {
            let parser = osm::fmi::Parser;
            parser.parse(&path)
        },
        Ok(osm::Support::XML) => {
            unimplemented!()
        },
        Err(e) => panic!("{:}", e),
    };

    println!("{}", graph);
}
