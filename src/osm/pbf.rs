use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use crate::routing;
use routing::Graph;

mod pbf {
    pub use osmpbfreader::reader::Iter;
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::{OsmObj, OsmPbfReader, RelationId};
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
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        //------------------------------------------------------------------------------------------
        // get reader

        let path = Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        let mut reader = pbf::Reader::new(file);

        //------------------------------------------------------------------------------------------
        // filter

        let ways: Vec<pbf::OsmObj> = reader
            .iter()
            .map(|obj| obj.expect("File is corrupted."))
            .filter(|obj| obj.is_way())
            .filter_map(|way| {
                // now: "way" really is a way
                // check if it's a street
                println!("{:?}", way.tags().get("highway")); // WIP: just barely added maxspeed in graph
                panic!(")")
                // Some(obj)
            })
            .collect();

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
