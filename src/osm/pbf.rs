use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

mod pbf {
    pub use osmpbfreader::reader::Iter;
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::{OsmObj, Way};
}

//--------------------------------------------------------------------------------------------------

enum WayTag {
    Motorway(),
    MotorwayLink(),
    Trunk(),
    TrunkLink(),
    Primary(),
    PrimaryLink(),
    Secondary(),
    SecondaryLink(),
    Tertiary(),
    TertiaryLink(),
    Unclassified(),
    Residential(),
    LivingStreet(),
    Service(),
    Track(),
    Road(),
    Cycleway(),
    Pedestrian(),
    Footway(),
    Steps(),
    Path(),
}

impl WayTag {
    fn maxspeed(&self) -> u8 {
        match self {
            Motorway => 130,
            MotorwayLink => 50,
            Trunk => 100,
            TrunkLink => 50,
            Primary => 100,
            PrimaryLink => 30,
            Secondary => 70,
            SecondaryLink => 30,
            Tertiary => 70,
            TertiaryLink => 30,
            Unclassified => 50,
            Residential => 50,
            LivingStreet => 15,
            Service => 20,
            Track => 30,
            Road => 50,
            Cycleway => 25,
            Pedestrian => 5,
            Footway => 5,
            Steps => 5,
            Path => 5,
        }
    }

    fn is_for_vehicles(&self, is_suitable: bool) -> bool {
        match self {
            Motorway => true,
            MotorwayLink => true,
            Trunk => true,
            TrunkLink => true,
            Primary => true,
            PrimaryLink => true,
            Secondary => true,
            SecondaryLink => true,
            Tertiary => true,
            TertiaryLink => true,
            Unclassified => true,
            Residential => true,
            LivingStreet => true,
            Service => !is_suitable,
            Track => !is_suitable,
            Road => !is_suitable,
            Cycleway => false,
            Pedestrian => false,
            Footway => false,
            Steps => false,
            Path => false,
        }
    }

    fn is_for_bicycles(&self, is_suitable: bool) -> bool {
        match self {
            Motorway => false,
            MotorwayLink => false,
            Trunk => false,
            TrunkLink => false,
            Primary => !is_suitable,
            PrimaryLink => !is_suitable,
            Secondary => !is_suitable,
            SecondaryLink => !is_suitable,
            Tertiary => true,
            TertiaryLink => true,
            Unclassified => true,
            Residential => true,
            LivingStreet => true,
            Service => true,
            Track => !is_suitable,
            Road => !is_suitable,
            Cycleway => false,
            Pedestrian => !is_suitable,
            Footway => false,
            Steps => false,
            Path => !is_suitable,
        }
    }

    fn is_for_pedestrians(&self, is_suitable: bool) -> bool {
        match self {
            Motorway => false,
            MotorwayLink => false,
            Trunk => false,
            TrunkLink => false,
            Primary => false,
            PrimaryLink => false,
            Secondary => false,
            SecondaryLink => false,
            Tertiary => false,
            TertiaryLink => false,
            Unclassified => false,
            Residential => true,
            LivingStreet => true,
            Service => true,
            Track => true,
            Road => !is_suitable,
            Cycleway => false,
            Pedestrian => true,
            Footway => true,
            Steps => true,
            Path => true,
        }
    }
}

//--------------------------------------------------------------------------------------------------

pub struct Parser;

impl Parser {
    fn build_edges_from_way(obj: &pbf::OsmObj) -> Vec<routing::Edge> {
        let mut new_edges = Vec::new();

        // match obj.tags().get("highway") {
        //     Some(_) => {
        //         println!("{:?}", obj);
        //         panic!()
        //         // Some(way)
        //     }
        //     None => None,
        // }

        new_edges
    }

    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        //------------------------------------------------------------------------------------------
        // get reader

        let path = Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        let mut reader = pbf::Reader::new(file);

        //------------------------------------------------------------------------------------------
        // init graphbuilder

        let mut graph_builder = GraphBuilder::new();

        //------------------------------------------------------------------------------------------
        // collect all ways from file and generate edges from them

        reader
            .iter()
            .filter_map(|obj| {
                if let pbf::OsmObj::Way(way) = obj.expect("pbf-File is corrupted.") {
                    Some(way)
                } else {
                    None
                }
            })
            .filter(|way| way.tags.get("highway").is_some())
            .filter(|way| way.nodes.len() > 1)
            .map(|way| {
                let mut nodes_iter = way.nodes.iter();
                let mut src = nodes_iter.next();
                for dst in nodes_iter {
                    // TODO implement after
                    // refactoring graphbuilder to generate internal IDs (= array-idx) by itself
                    // graph_builder.push_edge(id: usize, osm_id: Option<usize>, src: usize, dst: usize, meters: u64, maxspeed: u16);
                }
            });

        // old
        // let highways: Vec<pbf::Way> = reader
        //     .iter()
        //     .filter_map(|obj| {
        //         if let pbf::OsmObj::Way(way) = obj.expect("pbf-File is corrupted.") {
        //             Some(way)
        //         } else {
        //             None
        //         }
        //     })
        //     .filter(|way| way.tags.get("highway").is_some())
        //     .flat_map(|way| {
        //         let mut edges = Vec::new();

        //         if way.nodes.len() > 1 {
        //             let nodes_iter = way.nodes.iter();
        //             let mut src = nodes_iter.next();
        //             for dst in nodes_iter {
        //                 edges.push(routing::Edge)
        //             }
        //         }

        //         edges.iter()
        //     })
        //     .flatten()
        //     .collect();

        //------------------------------------------------------------------------------------------
        // create edges from preselected highways

        //------------------------------------------------------------------------------------------
        // check and remove intermediate nodes with only one incoming and one leaving edge
        // like (... -> a -> ...)

        // TODO
        // - Get all Ways
        // - Filter Ways
        //   - WayId -> osm_id
        //   - Tags
        //     - oneway vs multiway
        //     - maxspeed
        //     - nodes (could be empty or single-node)
        // - Get all Relations
        // - Get all Nodes
        // - Filter Nodes

        // println!();
        // println!("{}", ways);
        // println!();

        unimplemented!()
    }
}
