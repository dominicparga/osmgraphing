pub mod accuracy {
    /// value is good because it refers to
    /// - 1 cm for km
    /// - 10 µs for s
    /// - lat/lon
    ///   - lat: 1/60 ~ 0.0167 degrees equals 1_852 m
    ///     -> 1e-5 degrees equals around 1.1 m
    ///   - lon: distance depends on latitude
    ///     -> 1e-5 degrees equals <= 1.1 m (equator)
    ///   -> 1e-5 degrees points to a person in a house, see https://xkcd.com/2170/
    pub const F64_ABS: f64 = 0.000_01; // = 10^(-F64__FMT_DIGITS)
    pub const F64_FMT_DIGITS: usize = 5;
}

pub mod speed {
    pub const MAX_KMH: u16 = 130;
    pub const MIN_KMH: u8 = 5;
}

pub mod capacity {
    // For optimal performance and memory-usage:
    // Change this value before compiling, dependent of your number of stored metrics in the graph.
    pub const SMALL_VEC_INLINE_SIZE: usize = 4;
    pub type DimVec<T> = smallvec::SmallVec<[T; SMALL_VEC_INLINE_SIZE]>;
    pub const MAX_BYTE_PER_CHUNK: usize = 200 * 1_000_000;
}

pub mod parser {
    // provided by multi-ch-constructor
    pub const NO_SHORTCUT_IDX: &str = "-1";
}

pub mod generator {
    pub use super::parser::NO_SHORTCUT_IDX;
}

pub mod network {
    use crate::{
        defaults,
        network::{StreetCategory, VehicleCategory},
    };
    use log::warn;
    use osmpbfreader::Way;
    use std::{cmp::max, fmt, fmt::Display, str::FromStr};

    impl StreetCategory {
        fn lane_count(&self) -> u8 {
            match self {
                StreetCategory::Motorway => 3,
                StreetCategory::MotorwayLink => 1,
                StreetCategory::Trunk => 2,
                StreetCategory::TrunkLink => 1,
                StreetCategory::Primary => 2,
                StreetCategory::PrimaryLink => 1,
                StreetCategory::Secondary => 1,
                StreetCategory::SecondaryLink => 1,
                StreetCategory::Tertiary => 1,
                StreetCategory::TertiaryLink => 1,
                StreetCategory::Unclassified => 1,
                StreetCategory::Residential => 1,
                StreetCategory::LivingStreet => 1,
                StreetCategory::Service => 1,
                StreetCategory::Track => 1,
                StreetCategory::Road => 1,
                StreetCategory::Cycleway => 1,
                StreetCategory::Pedestrian => 1,
                StreetCategory::Path => 1,
            }
        }

        fn maxspeed(&self) -> u16 {
            match self {
                StreetCategory::Motorway => 130,
                StreetCategory::MotorwayLink => 50,
                StreetCategory::Trunk => 100,
                StreetCategory::TrunkLink => 50,
                StreetCategory::Primary => 100,
                StreetCategory::PrimaryLink => 30,
                StreetCategory::Secondary => 70,
                StreetCategory::SecondaryLink => 30,
                StreetCategory::Tertiary => 70,
                StreetCategory::TertiaryLink => 30,
                StreetCategory::Unclassified => 50,
                StreetCategory::Residential => 50,
                StreetCategory::LivingStreet => 15,
                StreetCategory::Service => 20,
                StreetCategory::Track => 30,
                StreetCategory::Road => 50,
                StreetCategory::Cycleway => 25,
                StreetCategory::Pedestrian => 5,
                StreetCategory::Path => 15,
            }
        }

        pub fn is_for(&self, vehicle_category: &VehicleCategory, is_driver_picky: bool) -> bool {
            match vehicle_category {
                VehicleCategory::Car => self.is_for_vehicles(is_driver_picky),
                VehicleCategory::Bicycle => self.is_for_bicycles(is_driver_picky),
                VehicleCategory::Pedestrian => self.is_for_pedestrians(is_driver_picky),
            }
        }

        fn is_for_vehicles(&self, is_driver_picky: bool) -> bool {
            match self {
                StreetCategory::Motorway => true,
                StreetCategory::MotorwayLink => true,
                StreetCategory::Trunk => true,
                StreetCategory::TrunkLink => true,
                StreetCategory::Primary => true,
                StreetCategory::PrimaryLink => true,
                StreetCategory::Secondary => true,
                StreetCategory::SecondaryLink => true,
                StreetCategory::Tertiary => true,
                StreetCategory::TertiaryLink => true,
                StreetCategory::Unclassified => true,
                StreetCategory::Residential => true,
                StreetCategory::LivingStreet => true,
                StreetCategory::Service => !is_driver_picky,
                StreetCategory::Track => !is_driver_picky,
                StreetCategory::Road => !is_driver_picky,
                StreetCategory::Cycleway => false,
                StreetCategory::Pedestrian => false,
                StreetCategory::Path => false,
            }
        }

        fn is_for_bicycles(&self, is_driver_picky: bool) -> bool {
            match self {
                StreetCategory::Motorway => false,
                StreetCategory::MotorwayLink => false,
                StreetCategory::Trunk => false,
                StreetCategory::TrunkLink => false,
                StreetCategory::Primary => !is_driver_picky,
                StreetCategory::PrimaryLink => !is_driver_picky,
                StreetCategory::Secondary => !is_driver_picky,
                StreetCategory::SecondaryLink => !is_driver_picky,
                StreetCategory::Tertiary => true,
                StreetCategory::TertiaryLink => true,
                StreetCategory::Unclassified => true,
                StreetCategory::Residential => true,
                StreetCategory::LivingStreet => true,
                StreetCategory::Service => true,
                StreetCategory::Track => !is_driver_picky,
                StreetCategory::Road => !is_driver_picky,
                StreetCategory::Cycleway => true,
                StreetCategory::Pedestrian => !is_driver_picky,
                StreetCategory::Path => !is_driver_picky,
            }
        }

        fn is_for_pedestrians(&self, is_driver_picky: bool) -> bool {
            match self {
                StreetCategory::Motorway => false,
                StreetCategory::MotorwayLink => false,
                StreetCategory::Trunk => false,
                StreetCategory::TrunkLink => false,
                StreetCategory::Primary => false,
                StreetCategory::PrimaryLink => false,
                StreetCategory::Secondary => false,
                StreetCategory::SecondaryLink => false,
                StreetCategory::Tertiary => false,
                StreetCategory::TertiaryLink => false,
                StreetCategory::Unclassified => false,
                StreetCategory::Residential => true,
                StreetCategory::LivingStreet => true,
                StreetCategory::Service => true,
                StreetCategory::Track => true,
                StreetCategory::Road => !is_driver_picky,
                StreetCategory::Cycleway => false,
                StreetCategory::Pedestrian => true,
                StreetCategory::Path => true,
            }
        }

        //--------------------------------------------------------------------------------------------//
        // parsing

        pub fn from(way: &Way) -> Option<StreetCategory> {
            // read highway-tag from way
            way.tags.get("highway").and_then(|highway_tag_value| {
                // and parse the value if valid
                match format!("highway:{}", highway_tag_value).parse::<StreetCategory>() {
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

        pub fn parse_lane_count(&self, _way: &Way) -> u8 {
            // TODO parse lanes
            self.lane_count()
        }

        pub fn parse_maxspeed(&self, way: &Way) -> u16 {
            let snippet = match way.tags.get("maxspeed") {
                Some(snippet) => snippet,
                None => return self.maxspeed(),
            };

            // parse given maxspeed and return
            match snippet.parse::<u16>() {
                Ok(maxspeed) => max(defaults::speed::MIN_KMH.into(), maxspeed),
                Err(_) => match snippet.trim().to_ascii_lowercase().as_ref() {
                    // motorway
                    "de:motorway"
                    => StreetCategory::Motorway.maxspeed(),
                    // 100 kmh
                    | "100, 70" // way-id: 319046425
                    | "100; 50" // way-id: 130880229
                    | "100;70;50" // way-id: 313097404
                    | "100;70" // way-id: 130006647
                    | "100;80" // way-id: 161216768
                    | "100|70" // way-id: 118245446
                    | "50; 100" // way-id: 152374728
                    | "50;100" // way-id: 266299302
                    | "60 mph"
                    => 100,
                    // 80 kmh
                    | "50 mph"
                    | "60;80" // way-id: 24441573
                    | "80;60" // way-id: 25154358
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
                    | "20; 50" // way-id: 308645778
                    | "30 mph"
                    | "30,50" // way-id: 28293340
                    | "30; 50" // way-id: 305677124
                    | "30;50" // way-id: 4954059
                    | "50; 30" // way-id: 28494183
                    | "50;30" // way-id: 25616305
                    | "50b"
                    | "5ß" // way-id: 8367325
                    | "de:urban" // way-id: 111446158
                    | "maxspeed=50"
                    => 50,
                    // 30 kmh
                    | "20 mph"
                    | "30 @ (mo-fr 06:00-18:00)" // way-id: 558224330
                    | "30 kph"
                    | "30;10" // way-id: 111450904
                    | "30; 40" // way-id: 28311529
                    | "3ß" // way-id: 4045417
                    | "conditional=30 @ (mo-fr 06:00-22:00)" // way-id: 612333030
                    | "de:zone:30" // way-id: 32657912
                    | "de:zone30"
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
                    => StreetCategory::Cycleway.maxspeed(),
                    // walk (<= 15 kmh)
                    | "10 mph"
                    | "10#" // way-id: 301985410
                    | "1ß"
                    | "3 mph"
                    | "4-6"
                    | "4-7"
                    | "5 mph"
                    | "6 km/h" // way-id: 60066367
                    | "6,5" // way-id: 27172163
                    | "7-10" // way-id: 60805930
                    | "de:living_street"
                    | "de:walk"
                    | "schrittgeschwindigkeit" // way-id: 212487477
                    | "walk"
                    => StreetCategory::LivingStreet.maxspeed(),
                    // known defaults/weirdos
                    | "*" // way-id: 4682329
                    | "20:forward" // way-id: 24215081
                    | "30+" // way-id: 87765739
                    | "at:rural" // way-id: 23622533
                    | "at:urban" // way-id: 30504860
                    | "cz:urban" // way-id: 683729581
                    | "de:274.1[30]" // way-id: 458676403
                    | "de:rural" // way-id: 15558598
                    | "de" // way-id: 180794115
                    | "fixme:höchster üblicher wert" // way-id: 8036120
                    | "hgv=30" // way-id: 33172848
                    | "nome" // way-id: 67659840
                    | "none" // way-id: 3061397
                    | "posted time dependent" // way-id: 168135218
                    | "signal" // way-id: 189189059
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

        /// return (is_oneway, is_reverse)
        pub fn parse_oneway(&self, way: &Way) -> (bool, bool) {
            let is_oneway = true;
            let is_reverse = true;

            match way.tags.get("oneway") {
                Some(oneway_value) => {
                    match oneway_value.trim().to_ascii_lowercase().as_ref() {
                        // yes
                        | "1"
                        | "left;through" // way-id: 679817792
                        | "motor_vehicle" // way-id: 172676596
                        | "recommended" // way-id: 38250792
                        | "shelter" // way-id: 680612616
                        | "use_sidepath" // way-id: 3701112
                        | "yes + oneway:bicycle=no" // way-id: 25013800
                        | "yes"
                        => (is_oneway, !is_reverse),
                        // yes but reverse
                        | "´-1" // way-id: 721848168
                        | "-1"
                        | "-1;no" // way-id: 180680762
                        => (is_oneway, is_reverse),
                        // no
                        | "alternating" // way-id: 5051072
                        | "bicycle" // way-id: 25596393
                        | "cycle_barrier" // way-id: 691452957
                        | "fixme" // way-id: 199388177
                        | "no"
                        | "reversible" // way-id: 4005347
                        | "undefined" // way-id: 331847642
                        | "unknown" // way-id: 380885551
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

    impl FromStr for StreetCategory {
        type Err = bool;

        fn from_str(s: &str) -> Result<StreetCategory, bool> {
            let is_unknown = true;
            match s.trim().to_ascii_lowercase().as_ref() {
                // known and used
                | "highway:motorway"
                => Ok(StreetCategory::Motorway),
                | "highway:motorway_link"
                => Ok(StreetCategory::MotorwayLink),
                | "highway:trunk"
                => Ok(StreetCategory::Trunk),
                | "highway:trunk_link"
                => Ok(StreetCategory::TrunkLink),
                | "highway:primary"
                => Ok(StreetCategory::Primary),
                | "highway:primary_link"
                => Ok(StreetCategory::PrimaryLink),
                | "highway:secondary"
                => Ok(StreetCategory::Secondary),
                | "highway:secondary_link"
                => Ok(StreetCategory::SecondaryLink),
                | "highway:tertiary"
                => Ok(StreetCategory::Tertiary),
                | "highway:tertiary_link"
                | "highway:traffic_calming" // way-id: 746304770
                | "highway:unclassified_link" // way-id: 460413095
                => Ok(StreetCategory::TertiaryLink),
                | "highway:give_way" // way-id: 61580672
                | "highway:unclassified"
                | "highway:unclasified" // way-id: 71428454
                => Ok(StreetCategory::Unclassified),
                | "highway:area:residential" // way-id: 36986745
                | "highway:asphalt" // way-id: 773144688
                | "highway:junction" // way-id: 589935900
                | "highway:mini_roundabout" // way-id: 745748272
                | "highway:residential"
                => Ok(StreetCategory::Residential),
                | "highway:living_street"
                => Ok(StreetCategory::LivingStreet),
                | "highway:razed:service" // way-id: 415355747
                | "highway:service;yes" // way-id: 170702046
                | "highway:service"
                | "highway:sevice" // way-id: 557625537
                | "highway:service2" // way-id: 553698179
                | "highway:swervice" // way-id: 551728065
                => Ok(StreetCategory::Service),
                | "highway:byway" // way-id: 29881284
                | "highway:historic" // way-id: 192265844
                | "highway:path;unclassified" // way-id: 38480982
                | "highway:tra#" // way-id: 721881475
                | "highway:track"
                | "highway:track;path" // way-id: 640616710
                | "highway:trank" // way-id: 685079101
                | "highway:track; cycleway; cycleway; track; track" // way-id: 128073314
                => Ok(StreetCategory::Track),
                | "highway:4" // way-id: 23128594545
                | "highway:bridge" // way-id: 696697784
                | "highway:fixme" // way-id: 371216260
                | "highway:parking_aisle" // way-id: 552156572
                | "highway:road"
                | "highway:yes" // way-id: 684234513
                => Ok(StreetCategory::Road),
                | "highway:cycleway"
                | "highway:bridleway" // way-id: 3617168
                => Ok(StreetCategory::Cycleway),
                | "highway:access_ramp" // way-id: 24975340
                | "highway:access" // way-id: 357086739
                | "highway:alley" // way-id: 24453717
                | "highway:corridor" // way-id: 210464225
                | "highway:crossing" // way-id: 679590652
                | "highway:elevator" // way-id: 166960177
                | "highway:footpath" // way-id: 306304178
                | "highway:footway rad frei" // way-id: 45786636
                | "highway:footway;service" // way-id: 245106042
                | "highway:footway"
                | "highway:fo" // way-id: 558233034
                | "highway:f" // way-id: 562267514
                | "highway:pa" // way-id: 193668915
                | "highway:pedestrian"
                | "highway:private_footway" // way-id: 61557441
                | "highway:ramp" // way-id: 60561495
                | "highway:schoolyard" // way-id: 254357487
                | "highway:sidewalk" // way-id: 492983410
                | "highway:stairs" // way-id: 698856376
                | "highway:steps"
                | "highway:trail" // way-id: 606170671
                | "highway:virtual" // way-id: 612194863
                | "highway:vitrual" // way-id: 699685919
                | "highway:yes;footway" // way-id: 634213443
                => Ok(StreetCategory::Pedestrian),
                | "highway:informal_path" // way-id: 27849992
                | "highway:ladder" // way-id: 415352091
                | "highway:path---" // way-id: 753671939
                | "highway:path;steps" // way-id: 768826568
                | "highway:path"
                | "highway:path/cycleway" // way-id: 152848247
                | "highway:pathless" // way-id: 529231499
                => Ok(StreetCategory::Path),
                // ignored
                | "highway:85" // way-id: 28682800
                | "highway:abondoned" // way-id: 550607106
                | "highway:abandoned:highway" // way-id: 243670918
                | "highway:abandoned:path" // way-id: 659187494
                | "highway:abandoned:service" // way-id: 668073809
                | "highway:abandoned" // way-id: 551167806
                | "highway:bus_guideway" // way-id: 659097872
                | "highway:bus_stop" // way-id: 551048594
                | "highway:bus" // way-id: 653176966
                | "highway:busway" // way-id: 26178605
                | "highway:centre_line" // way-id: 131730185
                | "highway:climbing_access" // way-id: 674680967
                | "highway:common" // way-id: 680432920
                | "highway:under construction" // way-id: 557005264
                | "highway:construction" // way-id: 23692144
                | "highway:constuction" // way-id: 40101546
                | "highway:demolished" // way-id: 146859260
                | "highway:dismantled" // way-id: 138717422
                | "highway:disused:track" // way-id: 660999751
                | "highway:disused" // way-id: 4058936
                | "highway:duckboards" // way-id: 121884826
                | "highway:emergency_access_point" // way-id: 124039649
                | "highway:emergency_bay" // way-id: 510872933
                | "highway:escalator" // way-id: 49542657
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
                | "highway:p" // way-id: 279099565
                | "highway:passing_place" // way-id: 678674065
                | "highway:piste" // way-id: 299032574
                | "highway:place" // way-id: 228745170
                | "highway:planned" // way-id: 509400222
                | "highway:platform" // way-id: 552088750
                | "highway:private" // way-id: 707015329
                | "highway:project" // way-id: 698166909
                | "highway:projected" // way-id: 698166910
                | "highway:proposed" // way-id: 23551790
                | "highway:raceway" // way-id: 550503761
                | "highway:razed" // way-id: 23653804
                | "highway:removed" // way-id: 667029512
                | "highway:rest_area" // way-id: 23584797
                | "highway:ser" // way-id: 27215798
                | "highway:sere" // way-id: 167276926
                | "highway:services" // way-id: 111251693
                | "highway:stop_line" // way-id: 569603293
                | "highway:stop" // way-id: 669234427
                | "highway:street_lamp" // way-id: 614573217
                | "highway:tidal_path" // way-id: 27676473
                | "highway:traffic_island" // way-id: 263644518
                | "highway:traffic_signals" // way-id: 300419851
                | "highway:turning_circle" // way-id: 669184618
                | "highway:turning_loop" // way-id: 31516941
                | "highway:unused" // way-id: 37888214
                | "highway:via_ferrata" // way-id: 23939968
                | "highway:virtual_rail" // way-id: 772152425
                => Err(!is_unknown),
                // unknown
                _ => Err(is_unknown),
            }
        }
    }

    impl Display for StreetCategory {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                match &self {
                    StreetCategory::Motorway => "motorway",
                    StreetCategory::MotorwayLink => "motorway_link",
                    StreetCategory::Trunk => "trunk",
                    StreetCategory::TrunkLink => "trunk_link",
                    StreetCategory::Primary => "primary",
                    StreetCategory::PrimaryLink => "primary_link",
                    StreetCategory::Secondary => "secondary",
                    StreetCategory::SecondaryLink => "secondary_link",
                    StreetCategory::Tertiary => "tertiary",
                    StreetCategory::TertiaryLink => "tertiary_link",
                    StreetCategory::Unclassified => "unclassified",
                    StreetCategory::Residential => "residential",
                    StreetCategory::LivingStreet => "living_street",
                    StreetCategory::Service => "service",
                    StreetCategory::Track => "track",
                    StreetCategory::Road => "road",
                    StreetCategory::Cycleway => "cycleway",
                    StreetCategory::Pedestrian => "pedestrian",
                    StreetCategory::Path => "path",
                }
            )
        }
    }
}
