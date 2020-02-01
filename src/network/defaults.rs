use log::warn;
use std::{cmp, fmt, str};
mod pbf {
    pub use osmpbfreader::Way;
}

//------------------------------------------------------------------------------------------------//

pub const MAX_SPEED_KMH: u8 = 130;
pub const MIN_SPEED_KMH: u8 = 5;

pub enum StreetType {
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
impl StreetType {
    //--------------------------------------------------------------------------------------------//
    // defaults

    fn lane_count(&self) -> u8 {
        match self {
            StreetType::Motorway => 3,
            StreetType::MotorwayLink => 1,
            StreetType::Trunk => 2,
            StreetType::TrunkLink => 1,
            StreetType::Primary => 2,
            StreetType::PrimaryLink => 1,
            StreetType::Secondary => 1,
            StreetType::SecondaryLink => 1,
            StreetType::Tertiary => 1,
            StreetType::TertiaryLink => 1,
            StreetType::Unclassified => 1,
            StreetType::Residential => 1,
            StreetType::LivingStreet => 1,
            StreetType::Service => 1,
            StreetType::Track => 1,
            StreetType::Road => 1,
            StreetType::Cycleway => 1,
            StreetType::Pedestrian => 1,
            StreetType::Path => 1,
        }
    }

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

    pub fn is_for_vehicles(&self, is_suitable: bool) -> bool {
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

    pub fn from(way: &pbf::Way) -> Option<StreetType> {
        // read highway-tag from way
        way.tags.get("highway").and_then(|highway_tag_value| {
            // and parse the value if valid
            match format!("highway:{}", highway_tag_value).parse::<StreetType>() {
                Ok(highway_tag) => Some(highway_tag),
                Err(is_unknown) => {
                    if is_unknown {
                        warn!(
                            "Unknown highway-tag `highway:{}` of way-id `{}` -> ignored",
                            highway_tag_value, way.id.0
                        );
                    }
                    None
                }
            }
        })
    }

    pub fn parse_lane_count(&self, _way: &pbf::Way) -> u8 {
        // TODO
        self.lane_count()
    }

    pub fn parse_maxspeed(&self, way: &pbf::Way) -> u16 {
        let snippet = match way.tags.get("maxspeed") {
            Some(snippet) => snippet,
            None => return self.maxspeed(),
        };

        // parse given maxspeed and return
        match snippet.parse::<u16>() {
            Ok(maxspeed) => cmp::max(MIN_SPEED_KMH.into(), maxspeed),
            Err(_) => match snippet.trim().to_ascii_lowercase().as_ref() {
                // motorway
                "de:motorway"
                => StreetType::Motorway.maxspeed(),
                // 100 kmh
                "100, 70" // way-id: 319046425
                | "100;70" // way-id: 130006647
                | "100;70;50" // way-id: 313097404
                | "100|70" // way-id: 118245446
                | "100; 50" // way-id: 130880229
                | "60 mph"
                | "50;100" // way-id: 266299302
                | "50; 100" // way-id: 152374728
                => 100,
                // 80 kmh
                "80;60" // way-id: 25154358
                | "60;80" // way-id: 24441573
                | "50 mph"
                => 80,
                // 70 kmh
                "70; 50" // way-id: 260835537
                | "50;70" // way-id: 48581258
                | "50; 70" // way-id: 20600128
                | "40 mph"
                => 70,
                // 60 kmh
                "60;50" // way-id: 48453714
                => 60,
                // 50 kmh
                "20; 50" // way-id: 308645778
                | "30;50" // way-id: 4954059
                | "30; 50" // way-id: 305677124
                | "30,50" // way-id: 28293340
                | "30 mph"
                | "50;30" // way-id: 25616305
                | "50; 30" // way-id: 28494183
                | "50b"
                | "de:urban" // way-id: 111446158
                | "maxspeed=50"
                => 50,
                // 30 kmh
                "20 mph"
                | "30 kph"
                | "30;10" // way-id: 111450904
                | "30 @ (mo-fr 06:00-18:00)" // way-id: 558224330
                | "conditional=30 @ (mo-fr 06:00-22:00)" // way-id: 612333030
                | "de:zone30"
                | "de:zone:30" // way-id: 32657912
                | "zone:maxspeed=de:30" // way-id: 26521170
                => 30,
                // 25 kmh
                "15 mph"
                => 25,
                // 20 kmh
                "2ß"
                => 20,
                // bicycle
                "de:bicycle_road"
                => StreetType::Cycleway.maxspeed(),
                // walk (<= 15 kmh)
                "3 mph"
                | "4-7"
                | "4-6"
                | "5 mph"
                | "6,5" // way-id: 27172163
                | "7-10" // way-id: 60805930
                | "10 mph"
                | "1ß"
                | "de:living_street"
                | "de:walk"
                | "schrittgeschwindigkeit" // way-id: 212487477
                | "walk"
                => StreetType::LivingStreet.maxspeed(),
                // known defaults/weirdos
                "*" // way-id: 4682329
                | "20:forward" // way-id: 24215081
                | "30+" // way-id: 87765739
                | "at:urban" // way-id: 30504860
                | "at:rural" // way-id: 23622533
                | "de:rural" // way-id: 15558598
                | "de" // way-id: 180794115
                | "fixme:höchster üblicher wert" // way-id: 8036120
                | "nome" // way-id: 67659840
                | "none" // way-id: 3061397
                | "posted time dependent" // way-id: 168135218
                | "signals" // way-id: 3996833
                | "variable" // way-id: 461169632
                => self.maxspeed(),
                // unknown
                _ => {
                    warn!(
                        "Unknown maxspeed `{}` of way-id `{}` -> default: (`{}`,`{}`)",
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
    pub fn parse_oneway(&self, way: &pbf::Way) -> (bool, bool) {
        let is_oneway = true;
        let is_reverse = true;

        match way.tags.get("oneway") {
            Some(oneway_value) => {
                match oneway_value.trim().to_ascii_lowercase().as_ref() {
                    // yes
                    "1"
                    | "left;through"
                    | "recommended"
                    | "shelter"
                    | "yes"
                    => (is_oneway, !is_reverse),
                    // yes but reverse
                    "´-1" // way-id: 721848168
                    | "-1"
                    | "-1;no"
                    => (is_oneway, is_reverse),
                    // no
                    "alternating"
                    | "bicycle"
                    | "cycle_barrier"
                    | "fixme"
                    | "no"
                    | "reversible"
                    | "undefined"
                    | "unknown"
                    | "use_sidepath" // way-id: 3701112
                    | "yes @ (2018 aug 0 - 2018 dec 21)" // way-id: 24379239
                    | "yes;no" // way-id: 158249443
                    => (!is_oneway, !is_reverse),
                    // unknown or unhandled
                    _ => {
                        warn!(
                            "Unknown oneway `{}` of way-id `{}` -> default: `oneway=no`",
                            oneway_value, way.id.0
                        );
                        (!is_oneway, !is_reverse)
                    }
                }
            }
            None => (!is_oneway, !is_reverse),
        }
    }
}
impl str::FromStr for StreetType {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_unknown = true;
        match s.trim().to_ascii_lowercase().as_ref() {
            // known and used
            "highway:motorway"
            => Ok(StreetType::Motorway),
            "highway:motorway_link"
            => Ok(StreetType::MotorwayLink),
            "highway:trunk"
            => Ok(StreetType::Trunk),
            "highway:trunk_link"
            => Ok(StreetType::TrunkLink),
            "highway:primary"
            => Ok(StreetType::Primary),
            "highway:primary_link"
            => Ok(StreetType::PrimaryLink),
            "highway:secondary"
            => Ok(StreetType::Secondary),
            "highway:secondary_link"
            => Ok(StreetType::SecondaryLink),
            "highway:tertiary"
            => Ok(StreetType::Tertiary),
            "highway:tertiary_link"
            | "highway:unclassified_link" // way-id: 460413095
            => Ok(StreetType::TertiaryLink),
            "highway:unclassified"
            => Ok(StreetType::Unclassified),
            "highway:residential"
            | "highway:junction" // way-id: 589935900
            => Ok(StreetType::Residential),
            "highway:living_street"
            => Ok(StreetType::LivingStreet),
            "highway:service"
            | "highway:service;yes" // way-id: 170702046
            => Ok(StreetType::Service),
            "highway:byway" // way-id: 29881284
            | "highway:historic" // way-id: 192265844
            | "highway:path;unclassified" // way-id: 38480982
            | "highway:tra#" // way-id: 721881475
            | "highway:track"
            | "highway:track;path" // way-id: 640616710
            | "highway:trank" // way-id: 685079101
            => Ok(StreetType::Track),
            "highway:bridge" // way-id: 696697784
            | "highway:road"
            | "highway:yes" // way-id: 684234513
            => Ok(StreetType::Road),
            "highway:cycleway"
            | "highway:bridleway" // way-id: 3617168
            => Ok(StreetType::Cycleway),
            "highway:access" // way-id: 357086739
            | "highway:access_ramp" // way-id: 24975340
            | "highway:alley" // way-id: 24453717
            | "highway:corridor" // way-id: 210464225
            | "highway:crossing" // way-id: 679590652
            | "highway:footway"
            | "highway:footway;service" // way-id: 245106042
            | "highway:pa" // way-id: 193668915
            | "highway:pedestrian"
            | "highway:private_footway" // way-id: 61557441
            | "highway:ramp" // way-id: 60561495
            | "highway:sidewalk" // way-id: 492983410
            | "highway:stairs" // way-id: 698856376
            | "highway:steps"
            | "highway:trail" // way-id: 606170671
            | "highway:vitrual" // way-id: 699685919
            | "highway:virtual" // way-id: 612194863
            | "highway:yes;footway" // way-id: 634213443
            => Ok(StreetType::Pedestrian),
            "highway:informal_path" // way-id: 27849992
            | "highway:ladder" // way-id: 415352091
            | "highway:path"
            | "highway:path/cycleway" // way-id: 152848247
            => Ok(StreetType::Path),
            // ignored
            "highway:85" // way-id: 28682800
            | "highway:abandoned" // way-id: 551167806
            | "highway:abandoned:highway" // way-id: 243670918
            | "highway:abandoned:path" // way-id: 659187494
            | "highway:abandoned:service" // way-id: 668073809
            | "highway:bus" // way-id: 653176966
            | "highway:bus_guideway" // way-id: 659097872
            | "highway:bus_stop" // way-id: 551048594
            | "highway:centre_line" // way-id: 131730185
            | "highway:climbing_access" // way-id: 674680967
            | "highway:common" // way-id: 680432920
            | "highway:construction" // way-id: 23692144
            | "highway:demolished" // way-id: 146859260
            | "highway:dismantled" // way-id: 138717422
            | "highway:disused" // way-id: 4058936
            | "highway:disused:track" // way-id: 660999751
            | "highway:elevator" // way-id: 166960177
            | "highway:emergency_access_point" // way-id: 124039649
            | "highway:emergency_bay" // way-id: 510872933
            | "highway:escape" // way-id: 166519327
            | "highway:foot" // way-id: 675407702
            | "highway:fuel" // way-id: 385074661
            | "highway:in planung" // way-id: 713888400
            | "highway:island" // way-id: 670953148
            | "highway:layby" // way-id: 171879602
            | "highway:loading_place" // way-id: 473983427
            | "highway:lohwiese" // way-id: 699398300
            | "highway:never_built" // way-id: 310787147
            | "highway:nicht mehr in benutzung" // way-id: 477193801
            | "highway:no" // way-id: 23605191
            | "highway:none" // way-id: 144657573
            | "highway:passing_place" // way-id: 678674065
            | "highway:place" // way-id: 228745170
            | "highway:planned" // way-id: 509400222
            | "highway:platform" // way-id: 552088750
            | "highway:piste" // way-id: 299032574
            | "highway:private" // way-id: 707015329
            | "highway:project" // way-id: 698166909
            | "highway:projected" // way-id: 698166910
            | "highway:proposed" // way-id: 23551790
            | "highway:raceway" // way-id: 550503761
            | "highway:razed" // way-id: 23653804
            | "highway:removed" // way-id: 667029512
            | "highway:rest_area" // way-id: 23584797
            | "highway:sere" // way-id: 167276926
            | "highway:ser" // way-id: 27215798
            | "highway:services" // way-id: 111251693
            | "highway:stop" // way-id: 669234427
            | "highway:stop_line" // way-id: 569603293
            | "highway:street_lamp" // way-id: 614573217
            | "highway:traffic_signals" // way-id: 300419851
            | "highway:tidal_path" // way-id: 27676473
            | "highway:traffic_island" // way-id: 263644518
            | "highway:turning_circle" // way-id: 669184618
            | "highway:turning_loop" // way-id: 31516941
            | "highway:via_ferrata" // way-id: 23939968
            => Err(!is_unknown),
            // unknown
            _ => Err(is_unknown),
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
