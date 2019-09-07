use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::path;
use std::str;

use log::{error, info, warn};

use crate::osm;
use crate::routing;
use osm::geo;
use routing::Graph;
use routing::GraphBuilder;

mod pbf {
    pub use osmpbfreader::reader::Iter;
    pub use osmpbfreader::reader::OsmPbfReader as Reader;
    pub use osmpbfreader::NodeId;
    pub use osmpbfreader::{Node, OsmObj, Way};
}

//------------------------------------------------------------------------------------------------//

enum StreetType {
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
    Path,
}

impl str::FromStr for StreetType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized_s = s.trim().to_ascii_lowercase();

        match normalized_s.as_ref() {
            // known and used
            "highway:motorway" => Ok(StreetType::Motorway),
            "highway:motorway_link" => Ok(StreetType::MotorwayLink),
            "highway:trunk" => Ok(StreetType::Trunk),
            "highway:trunk_link" => Ok(StreetType::TrunkLink),
            "highway:primary" => Ok(StreetType::Primary),
            "highway:primary_link" => Ok(StreetType::PrimaryLink),
            "highway:secondary" => Ok(StreetType::Secondary),
            "highway:secondary_link" => Ok(StreetType::SecondaryLink),
            "highway:tertiary" => Ok(StreetType::Tertiary),
            "highway:tertiary_link" => Ok(StreetType::TertiaryLink),
            "highway:unclassified" => Ok(StreetType::Unclassified),
            "highway:residential" => Ok(StreetType::Residential),
            "highway:living_street" => Ok(StreetType::LivingStreet),
            "highway:service" => Ok(StreetType::Service),
            "highway:track" => Ok(StreetType::Track),
            "highway:road" => Ok(StreetType::Road),
            "highway:cycleway" | "highway:bridleway" => Ok(StreetType::Cycleway),
            "highway:pedestrian" | "highway:footway" | "highway:steps" => {
                Ok(StreetType::Pedestrian)
            }
            "highway:path" => Ok(StreetType::Path),
            // ignored
            "highway:byway" | "highway:bus_stop" | "highway:raceway" => Err(normalized_s),
            // unknown
            _ => {
                warn!("Could not parse highway-tag `{}`", s);
                Err(normalized_s)
            }
        }
    }
}

impl fmt::Display for StreetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                StreetType::Motorway => "motorway",
                StreetType::MotorwayLink => "motorway_link",
                StreetType::Trunk => "trunk",
                StreetType::TrunkLink => "trunk_link",
                StreetType::Primary => "primary",
                StreetType::PrimaryLink => "primary_link",
                StreetType::Secondary => "secondary",
                StreetType::SecondaryLink => "secondary_link",
                StreetType::Tertiary => "tertiary",
                StreetType::TertiaryLink => "tertiary_link",
                StreetType::Unclassified => "unclassified",
                StreetType::Residential => "residential",
                StreetType::LivingStreet => "living_street",
                StreetType::Service => "service",
                StreetType::Track => "track",
                StreetType::Road => "road",
                StreetType::Cycleway => "cycleway",
                StreetType::Pedestrian => "pedestrian",
                StreetType::Path => "path",
            }
        )
    }
}

impl StreetType {
    //--------------------------------------------------------------------------------------------//
    // defaults

    fn maxspeed(&self) -> u16 {
        match self {
            StreetType::Motorway => 130,
            StreetType::MotorwayLink => 50,
            StreetType::Trunk => 100,
            StreetType::TrunkLink => 50,
            StreetType::Primary => 100,
            StreetType::PrimaryLink => 30,
            StreetType::Secondary => 70,
            StreetType::SecondaryLink => 30,
            StreetType::Tertiary => 70,
            StreetType::TertiaryLink => 30,
            StreetType::Unclassified => 50,
            StreetType::Residential => 50,
            StreetType::LivingStreet => 15,
            StreetType::Service => 20,
            StreetType::Track => 30,
            StreetType::Road => 50,
            StreetType::Cycleway => 25,
            StreetType::Pedestrian => 5,
            StreetType::Path => 15,
        }
    }

    fn _is_for_vehicles(&self, is_suitable: bool) -> bool {
        match self {
            StreetType::Motorway => true,
            StreetType::MotorwayLink => true,
            StreetType::Trunk => true,
            StreetType::TrunkLink => true,
            StreetType::Primary => true,
            StreetType::PrimaryLink => true,
            StreetType::Secondary => true,
            StreetType::SecondaryLink => true,
            StreetType::Tertiary => true,
            StreetType::TertiaryLink => true,
            StreetType::Unclassified => true,
            StreetType::Residential => true,
            StreetType::LivingStreet => true,
            StreetType::Service => !is_suitable,
            StreetType::Track => !is_suitable,
            StreetType::Road => !is_suitable,
            StreetType::Cycleway => false,
            StreetType::Pedestrian => false,
            StreetType::Path => false,
        }
    }

    fn _is_for_bicycles(&self, is_suitable: bool) -> bool {
        match self {
            StreetType::Motorway => false,
            StreetType::MotorwayLink => false,
            StreetType::Trunk => false,
            StreetType::TrunkLink => false,
            StreetType::Primary => !is_suitable,
            StreetType::PrimaryLink => !is_suitable,
            StreetType::Secondary => !is_suitable,
            StreetType::SecondaryLink => !is_suitable,
            StreetType::Tertiary => true,
            StreetType::TertiaryLink => true,
            StreetType::Unclassified => true,
            StreetType::Residential => true,
            StreetType::LivingStreet => true,
            StreetType::Service => true,
            StreetType::Track => !is_suitable,
            StreetType::Road => !is_suitable,
            StreetType::Cycleway => true,
            StreetType::Pedestrian => !is_suitable,
            StreetType::Path => !is_suitable,
        }
    }

    fn _is_for_pedestrians(&self, is_suitable: bool) -> bool {
        match self {
            StreetType::Motorway => false,
            StreetType::MotorwayLink => false,
            StreetType::Trunk => false,
            StreetType::TrunkLink => false,
            StreetType::Primary => false,
            StreetType::PrimaryLink => false,
            StreetType::Secondary => false,
            StreetType::SecondaryLink => false,
            StreetType::Tertiary => false,
            StreetType::TertiaryLink => false,
            StreetType::Unclassified => false,
            StreetType::Residential => true,
            StreetType::LivingStreet => true,
            StreetType::Service => true,
            StreetType::Track => true,
            StreetType::Road => !is_suitable,
            StreetType::Cycleway => false,
            StreetType::Pedestrian => true,
            StreetType::Path => true,
        }
    }

    //--------------------------------------------------------------------------------------------//
    // parsing

    fn from(way: &pbf::Way) -> Option<StreetType> {
        // read highway-tag from way
        way.tags.get("highway").and_then(|highway_tag_value| {
            // and parse the value if valid
            format!("highway:{}", highway_tag_value)
                .parse::<StreetType>()
                .ok()
        })
    }

    fn parse_maxspeed(&self, way: &pbf::Way) -> u16 {
        let snippet = match way.tags.get("maxspeed") {
            Some(snippet) => snippet,
            None => return self.maxspeed(),
        };

        // parse given maxspeed and return
        match snippet.parse::<u16>() {
            Ok(maxspeed) => maxspeed,
            Err(_) => match snippet.to_ascii_lowercase().as_ref() {
                // motorway
                "de:motorway" => StreetType::Motorway.maxspeed(),

                // // urban
                // "de:urban" | "de:rural" | "at:urban" | "at:rural" => {
                //     StreetType::Tertiary.maxspeed()
                // }

                // 100 kmh
                "60 mph" => 100,
                // 80 kmh
                "50 mph" => 80,
                // 70 kmh
                "40 mph" => 70,
                // 50 kmh
                "30 mph" | "maxspeed=50" | "50b" => 50,
                // 30 kmh
                "20 mph" | "de:zone30" | "30 kph" => 30,
                // 25 kmh
                "15 mph" => 25,
                // 20 kmh
                "2ß" => 20,
                // bicycle
                "de:bicycle_road" => StreetType::Cycleway.maxspeed(),
                // walk (<= 15 kmh)
                "10 mph"
                | "5 mph"
                | "3 mph"
                | "1ß"
                | "4-7"
                | "4-6"
                | "Schrittgeschwindigkeit"
                | "de:living_street"
                | "de:walk"
                | "walk" => StreetType::LivingStreet.maxspeed(),
                // known defaults
                "none"
                | "signals"
                | "*"
                | "variable"
                | "fixme:höchster üblicher Wert"
                | "posted time dependent"
                | "de" => self.maxspeed(),
                // unknown
                _ => {
                    warn!(
                        "Unknown maxspeed `{}` of way-id `{}` \
                         -> default: (`{}`,`{}`)",
                        snippet,
                        way.id.0,
                        self,
                        self.maxspeed()
                    );
                    self.maxspeed()
                }
            },
        }
    }

    // return (is_oneway, is_reverse)
    fn parse_oneway(&self, way: &pbf::Way) -> (bool, bool) {
        let is_oneway = true;
        let is_reverse = true;

        match way.tags.get("oneway") {
            Some(oneway_value) => {
                let oneway_value = oneway_value
                    .split_whitespace()
                    .next()
                    .expect("`oneway_value has already been matched, so it should match again.");
                match oneway_value.as_ref() {
                    // yes
                    "yes" | "1" | "recommended" | "left;through" | "shelter"
                    => (is_oneway, !is_reverse),
                    // yes but reverse
                    "-1" | "-1;no" => (is_oneway, is_reverse),
                    // no
                    "no" | "reversible" | "alternating" | "fixme" | "undefined"
                    | "unknown" | "cycle_barrier"
                    // for bicycle, e.g. WayId(3701112)
                    // -> secondary
                    // -> handled by highway_tag above
                    | "use_sidepath"
                    //
                    | "bicycle" => (!is_oneway, !is_reverse),
                    // unknown or unhandled
                    _ => {
                        error!(
                            "Setting `oneway=no` for unknown value of way {:?}",
                            way
                        );
                        (!is_oneway, !is_reverse)
                    }
                }
            }
            None => (!is_oneway, !is_reverse),
        }
    }
}

//------------------------------------------------------------------------------------------------//

pub struct Parser;

impl Parser {
    fn open_reader<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> pbf::Reader<File> {
        let path = path::Path::new(&path);
        let file =
            File::open(&path).expect(&format!("Expects the given path {:?} to exist.", path));
        pbf::Reader::new(file)
    }

    fn parse_ways<S: AsRef<OsStr> + ?Sized>(&self, path: &S, graph_builder: &mut GraphBuilder) {
        info!("Starting edge-creation using ways ..");
        for mut way in self
            .open_reader(&path)
            .par_iter()
            .filter_map(Result::ok)
            .filter_map(|obj| match obj {
                pbf::OsmObj::Way(way) => Some(way),
                _ => None,
            })
        {
            if way.nodes.len() < 2 {
                continue;
            }

            // collect relevant data from file
            let highway_tag = match StreetType::from(&way) {
                Some(highway_tag) => highway_tag,
                None => continue,
            };
            let maxspeed = highway_tag.parse_maxspeed(&way);
            let (is_oneway, is_reverse) = highway_tag.parse_oneway(&way);

            // create (proto-)edges
            if is_reverse {
                way.nodes.reverse();
            }
            let iter_range = if is_oneway {
                0..0
            } else {
                // if not oneway
                // -> add node-IDs reversed to generate edges forwards and backwards
                0..way.nodes.len() - 1
            };
            let mut nodes_iter = way.nodes.iter().chain(way.nodes[iter_range].iter().rev());

            // add edges, one per node-pair in way.nodes
            let mut src_id = nodes_iter
                .next()
                .expect(format!("Way.nodes.len()={} but should be >1.", way.nodes.len()).as_ref());
            for dst_id in nodes_iter {
                graph_builder.push_edge(Some(way.id.0), src_id.0, dst_id.0, None, maxspeed);
                src_id = dst_id;
            }
        }
        info!("Finished edge-creation using ways");
    }

    fn parse_nodes<S: AsRef<OsStr> + ?Sized>(&self, path: &S, graph_builder: &mut GraphBuilder) {
        info!("Starting node-creation using ways ..");
        for node in self
            .open_reader(&path)
            .par_iter()
            .filter_map(Result::ok)
            .filter_map(|obj| match obj {
                pbf::OsmObj::Node(node) => Some(node),
                _ => None,
            })
        {
            if graph_builder.is_node_in_edge(node.id.0) {
                graph_builder.push_node(
                    node.id.0,
                    geo::Coordinate::new(node.decimicro_lat, node.decimicro_lon),
                );
            }
        }
        info!("Finished node-creation using ways");
    }

    pub fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S) -> Graph {
        info!("Starting parsing ..");

        // TODO parse "cycleway" and others
        // see https://wiki.openstreetmap.org/wiki/Key:highway

        let mut graph_builder = GraphBuilder::new();

        info!("Starting processing given pbf-file ..");
        self.parse_ways(&path, &mut graph_builder);
        self.parse_nodes(&path, &mut graph_builder);
        info!("Finished processing given pbf-file");

        let graph = graph_builder.finalize();
        info!("Finished parsing");
        graph
    }
}
