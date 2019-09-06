use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::path;
use std::str;

use log::{debug, error, info, trace};

use crate::err;
use crate::osm::geo;
use crate::routing;
use routing::Graph;
use routing::GraphBuilder;

mod pbf {
    pub use osmpbfreader::reader::Iter;
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::NodeId;
    pub use osmpbfreader::{OsmObj, Way};
}

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub enum ParseError {
    Custom(err::Error),
    UnknownHighway(String),
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        ParseError::Custom(err::Error::new(msg))
    }

    pub fn unknown_highway(highway: &str) -> Self {
        ParseError::UnknownHighway(String::from(highway))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Custom(e) => e.fmt(f),
            ParseError::UnknownHighway(highway) => highway.fmt(f),
        }
    }
}

//------------------------------------------------------------------------------------------------//

enum HighwayTag {
    Motorway,
    MotorwayLink,
    Trunk,
    TrunkLink,
    Primary,
    PrimaryLink,
    Secondary,
    SecondaryLink,
    Tertiary,
    TertiaryLink,
    Unclassified,
    Residential,
    LivingStreet,
    Service,
    Track,
    Road,
    Cycleway,
    Pedestrian,
    Footway,
    Steps,
    Path,
}

impl HighwayTag {
    fn maxspeed(&self) -> u16 {
        match self {
            HighwayTag::Motorway => 130,
            HighwayTag::MotorwayLink => 50,
            HighwayTag::Trunk => 100,
            HighwayTag::TrunkLink => 50,
            HighwayTag::Primary => 100,
            HighwayTag::PrimaryLink => 30,
            HighwayTag::Secondary => 70,
            HighwayTag::SecondaryLink => 30,
            HighwayTag::Tertiary => 70,
            HighwayTag::TertiaryLink => 30,
            HighwayTag::Unclassified => 50,
            HighwayTag::Residential => 50,
            HighwayTag::LivingStreet => 15,
            HighwayTag::Service => 20,
            HighwayTag::Track => 30,
            HighwayTag::Road => 50,
            HighwayTag::Cycleway => 25,
            HighwayTag::Pedestrian => 5,
            HighwayTag::Footway => 5,
            HighwayTag::Steps => 5,
            HighwayTag::Path => 5,
        }
    }

    fn _is_for_vehicles(&self, is_suitable: bool) -> bool {
        match self {
            HighwayTag::Motorway => true,
            HighwayTag::MotorwayLink => true,
            HighwayTag::Trunk => true,
            HighwayTag::TrunkLink => true,
            HighwayTag::Primary => true,
            HighwayTag::PrimaryLink => true,
            HighwayTag::Secondary => true,
            HighwayTag::SecondaryLink => true,
            HighwayTag::Tertiary => true,
            HighwayTag::TertiaryLink => true,
            HighwayTag::Unclassified => true,
            HighwayTag::Residential => true,
            HighwayTag::LivingStreet => true,
            HighwayTag::Service => !is_suitable,
            HighwayTag::Track => !is_suitable,
            HighwayTag::Road => !is_suitable,
            HighwayTag::Cycleway => false,
            HighwayTag::Pedestrian => false,
            HighwayTag::Footway => false,
            HighwayTag::Steps => false,
            HighwayTag::Path => false,
        }
    }

    fn _is_for_bicycles(&self, is_suitable: bool) -> bool {
        match self {
            HighwayTag::Motorway => false,
            HighwayTag::MotorwayLink => false,
            HighwayTag::Trunk => false,
            HighwayTag::TrunkLink => false,
            HighwayTag::Primary => !is_suitable,
            HighwayTag::PrimaryLink => !is_suitable,
            HighwayTag::Secondary => !is_suitable,
            HighwayTag::SecondaryLink => !is_suitable,
            HighwayTag::Tertiary => true,
            HighwayTag::TertiaryLink => true,
            HighwayTag::Unclassified => true,
            HighwayTag::Residential => true,
            HighwayTag::LivingStreet => true,
            HighwayTag::Service => true,
            HighwayTag::Track => !is_suitable,
            HighwayTag::Road => !is_suitable,
            HighwayTag::Cycleway => false,
            HighwayTag::Pedestrian => !is_suitable,
            HighwayTag::Footway => false,
            HighwayTag::Steps => false,
            HighwayTag::Path => !is_suitable,
        }
    }

    fn _is_for_pedestrians(&self, is_suitable: bool) -> bool {
        match self {
            HighwayTag::Motorway => false,
            HighwayTag::MotorwayLink => false,
            HighwayTag::Trunk => false,
            HighwayTag::TrunkLink => false,
            HighwayTag::Primary => false,
            HighwayTag::PrimaryLink => false,
            HighwayTag::Secondary => false,
            HighwayTag::SecondaryLink => false,
            HighwayTag::Tertiary => false,
            HighwayTag::TertiaryLink => false,
            HighwayTag::Unclassified => false,
            HighwayTag::Residential => true,
            HighwayTag::LivingStreet => true,
            HighwayTag::Service => true,
            HighwayTag::Track => true,
            HighwayTag::Road => !is_suitable,
            HighwayTag::Cycleway => false,
            HighwayTag::Pedestrian => true,
            HighwayTag::Footway => true,
            HighwayTag::Steps => true,
            HighwayTag::Path => true,
        }
    }
}

impl str::FromStr for HighwayTag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_ascii_lowercase();

        match s.as_ref() {
            "motorway" => Ok(HighwayTag::Motorway),
            "motorway_link" => Ok(HighwayTag::MotorwayLink),
            "trunk" => Ok(HighwayTag::Trunk),
            "trunk_link" => Ok(HighwayTag::TrunkLink),
            "primary" => Ok(HighwayTag::Primary),
            "primary_link" => Ok(HighwayTag::PrimaryLink),
            "secondary" => Ok(HighwayTag::Secondary),
            "secondary_link" => Ok(HighwayTag::SecondaryLink),
            "tertiary" => Ok(HighwayTag::Tertiary),
            "tertiary_link" => Ok(HighwayTag::TertiaryLink),
            "unclassified" => Ok(HighwayTag::Unclassified),
            "residential" => Ok(HighwayTag::Residential),
            "living_street" => Ok(HighwayTag::LivingStreet),
            "service" => Ok(HighwayTag::Service),
            "track" => Ok(HighwayTag::Track),
            "road" => Ok(HighwayTag::Road),
            "cycleway" => Ok(HighwayTag::Cycleway),
            "pedestrian" => Ok(HighwayTag::Pedestrian),
            "footway" => Ok(HighwayTag::Footway),
            "steps" => Ok(HighwayTag::Steps),
            "path" => Ok(HighwayTag::Path),
            unknown => Err(ParseError::unknown_highway(unknown)),
        }
    }
}

impl fmt::Display for HighwayTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                HighwayTag::Motorway => "motorway",
                HighwayTag::MotorwayLink => "motorway_link",
                HighwayTag::Trunk => "trunk",
                HighwayTag::TrunkLink => "trunk_link",
                HighwayTag::Primary => "primary",
                HighwayTag::PrimaryLink => "primary_link",
                HighwayTag::Secondary => "secondary",
                HighwayTag::SecondaryLink => "secondary_link",
                HighwayTag::Tertiary => "tertiary",
                HighwayTag::TertiaryLink => "tertiary_link",
                HighwayTag::Unclassified => "unclassified",
                HighwayTag::Residential => "residential",
                HighwayTag::LivingStreet => "living_street",
                HighwayTag::Service => "service",
                HighwayTag::Track => "track",
                HighwayTag::Road => "road",
                HighwayTag::Cycleway => "cycleway",
                HighwayTag::Pedestrian => "pedestrian",
                HighwayTag::Footway => "footway",
                HighwayTag::Steps => "steps",
                HighwayTag::Path => "path",
            }
        )
    }
}

//------------------------------------------------------------------------------------------------//

pub struct Parser;

impl Parser {
    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        info!("Starting parsing ..");

        //----------------------------------------------------------------------------------------//
        // get reader

        let path = path::Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        let mut reader = pbf::Reader::new(file);

        //----------------------------------------------------------------------------------------//
        // init graphbuilder

        let mut graph_builder = GraphBuilder::new();

        //----------------------------------------------------------------------------------------//
        // collect all nodes and ways

        info!("Starting processing given pbf-file ..");
        for obj in reader.par_iter().filter_map(|obj| match obj {
            Ok(obj) => Some(obj),
            Err(_) => {
                error!("pbf-File is corrupted. Skipping obj {:?}", obj);
                None
            }
        }) {
            match obj {
                // if node -> just add every node to filter them out later
                pbf::OsmObj::Node(node) => {
                    debug!("{:?}", node);
                    graph_builder.push_node(
                        node.id.0,
                        geo::Coordinate::new(node.decimicro_lat, node.decimicro_lon),
                    );
                }
                // if edge -> filter and push as edge
                pbf::OsmObj::Way(mut way) => {
                    if way.nodes.len() < 2 {
                        continue;
                    }

                    // read highway-tag from way
                    let highway_tag_value = match way.tags.get("highway") {
                        Some(value) => value,
                        None => continue,
                    };

                    // get highway tag for defaults
                    let highway_tag = match highway_tag_value.parse::<HighwayTag>() {
                        Ok(highway_tag) => highway_tag,
                        Err(e) => {
                            debug!("Ignored highway-tag `{}`", e);
                            continue;
                        }
                    };

                    // maxspeed
                    let maxspeed = match way.tags.get("maxspeed") {
                        Some(s) => {
                            // parse given maxspeed
                            match s.parse::<u16>() {
                                Ok(maxspeed) => maxspeed,
                                Err(_) => match s.to_ascii_lowercase().as_ref() {
                                    // motorway
                                    "de:motorway" => HighwayTag::Motorway.maxspeed(),
                                    // urban
                                    "de:urban" | "de:rural" | "at:urban" | "at:rural" => 70,
                                    // 50 kmh
                                    "30 mph" | "maxspeed=50" | "50b" => 50,
                                    // 30 kmh
                                    "de:zone30" | "30 kph" => 30,
                                    // bicycle
                                    "de:bicycle_road" => HighwayTag::Cycleway.maxspeed(),
                                    // walk
                                    "Schrittgeschwindigkeit"
                                    | "de:living_street"
                                    | "de:walk"
                                    | "walk"
                                    | "5 mph"
                                    | "10 mph"
                                    | "4-6"
                                    | "4-7" => HighwayTag::LivingStreet.maxspeed(),
                                    // default
                                    "none"
                                    | "signals"
                                    | "*"
                                    | "variable"
                                    | "fixme:höchster üblicher Wert"
                                    | "posted time dependent"
                                    | "1ß"
                                    | "2ß"
                                    | "de" => highway_tag.maxspeed(),
                                    // unknown or unhandled
                                    _ => {
                                        debug!(
                                            "Unknown maxspeed `{}` of way-id `{}` \
                                             -> default: (`{}`,`{}`)",
                                            s,
                                            way.id.0,
                                            highway_tag,
                                            highway_tag.maxspeed()
                                        );
                                        highway_tag.maxspeed()
                                    }
                                },
                            }
                        }
                        None => {
                            trace!(
                                "Take default-maxspeed {} km/h for highway-tag `{:}` \
                                 since no maxspeed in way-id `{}`.",
                                highway_tag.maxspeed(),
                                highway_tag,
                                way.id.0
                            );
                            highway_tag.maxspeed()
                        }
                    };

                    let mut is_both_way = false;
                    // and process tag `oneway`
                    match way.tags.get("oneway") {
                        Some(oneway_value) => {
                            let oneway_value = oneway_value.split_whitespace().next().expect(
                                "`oneway_value has already been matched, so it should match again.",
                            );
                            match oneway_value.as_ref() {
                                // yes
                                "yes" | "1" | "recommended" | "left;through" | "shelter" => (),
                                // yes but reverse
                                "-1" | "-1;no" => way.nodes.reverse(),
                                // no
                                "no" | "reversible" | "alternating" | "fixme" | "undefined"
                                | "unknown" | "cycle_barrier" => is_both_way = true,
                                // for bicycle, e.g. WayId(3701112)
                                // -> secondary
                                // -> handled by highway_tag above
                                "use_sidepath" => (),
                                "bicycle" => is_both_way = true,
                                // unknown or unhandled
                                _ => {
                                    error!(
                                        "Setting `oneway=no` for unknown value of way {:?}",
                                        way
                                    );
                                }
                            }
                        }
                        None => is_both_way = true,
                    }

                    // if both way -> add node-IDs reversed to generate edges forwards and backwards
                    let iter_range = if is_both_way {
                        0..way.nodes.len() - 1
                    } else {
                        0..0
                    };
                    let mut nodes_iter = way.nodes.iter().chain(way.nodes[iter_range].iter().rev());

                    // add edges per node-pair in way.nodes
                    let mut src_id = nodes_iter.next().expect(
                        format!("Way.nodes.len()={} but should be >1.", way.nodes.len()).as_ref(),
                    );
                    for dst_id in nodes_iter {
                        graph_builder.push_edge(Some(way.id.0), src_id.0, dst_id.0, None, maxspeed);
                        src_id = dst_id;
                    }
                }
                _ => {
                    debug!("Unused object in pbf-file: {:?}", obj);
                }
            }
        }
        info!("Finished processing given pbf-file.");

        graph_builder.finalize()
    }
}
