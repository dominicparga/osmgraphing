pub mod nodes {
    use crate::configs::{raw::parsing::nodes as raw, SimpleId};
    use serde::Deserialize;
    use std::fmt::{self, Display};

    #[derive(Clone, Debug, Deserialize)]
    pub enum Category {
        Meta { info: MetaInfo, id: SimpleId },
        Metric { unit: UnitInfo, id: SimpleId },
        Ignored,
    }

    impl Display for Category {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::Category> for Category {
        fn from(raw_category: raw::Category) -> Category {
            match raw_category {
                raw::Category::Meta { info, id } => Category::Meta {
                    info: info.into(),
                    id,
                },
                raw::Category::Metric { unit, id } => Category::Metric {
                    unit: unit.into(),
                    id,
                },
                raw::Category::Ignored => Category::Ignored,
            }
        }
    }

    #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
    pub enum MetaInfo {
        NodeId,
        NodeIdx,
        Level,
    }

    impl Display for MetaInfo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::MetaInfo> for MetaInfo {
        fn from(raw_info: raw::MetaInfo) -> MetaInfo {
            match raw_info {
                raw::MetaInfo::NodeId => MetaInfo::NodeId,
                raw::MetaInfo::Level => MetaInfo::Level,
            }
        }
    }

    #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
    pub enum UnitInfo {
        Latitude,
        Longitude,
        Height,
    }

    impl Display for UnitInfo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::UnitInfo> for UnitInfo {
        fn from(raw_unit: raw::UnitInfo) -> UnitInfo {
            match raw_unit {
                raw::UnitInfo::Latitude => UnitInfo::Latitude,
                raw::UnitInfo::Longitude => UnitInfo::Longitude,
            }
        }
    }
}

pub mod edges {
    use crate::configs::{raw::parsing::edges as raw, SimpleId};
    use kissunits::{
        distance::{Kilometers, Meters},
        time::{Hours, Minutes, Seconds},
    };
    use serde::Deserialize;
    use std::fmt::{self, Display};

    #[derive(Clone, Debug, Deserialize)]
    pub enum Category {
        Meta { info: MetaInfo, id: SimpleId },
        Metric { unit: UnitInfo, id: SimpleId },
        Ignored,
    }

    impl Display for Category {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::Category> for Category {
        fn from(raw_category: raw::Category) -> Category {
            match raw_category {
                raw::Category::Meta { info, id } => Category::Meta {
                    info: info.into(),
                    id,
                },
                raw::Category::Metric { unit, id } => Category::Metric {
                    unit: unit.into(),
                    id,
                },
                raw::Category::Ignored => Category::Ignored,
            }
        }
    }

    #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
    pub enum MetaInfo {
        SrcId,
        SrcIdx,
        DstId,
        DstIdx,
        ShortcutEdgeIdx0,
        ShortcutEdgeIdx1,
    }

    impl Display for MetaInfo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::MetaInfo> for MetaInfo {
        fn from(raw_info: raw::MetaInfo) -> MetaInfo {
            match raw_info {
                raw::MetaInfo::SrcId => MetaInfo::SrcId,
                raw::MetaInfo::DstId => MetaInfo::DstId,
                raw::MetaInfo::ShortcutEdgeIdx0 => MetaInfo::ShortcutEdgeIdx0,
                raw::MetaInfo::ShortcutEdgeIdx1 => MetaInfo::ShortcutEdgeIdx1,
            }
        }
    }

    #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
    pub enum UnitInfo {
        Meters,
        Kilometers,
        Seconds,
        Minutes,
        Hours,
        KilometersPerHour,
        LaneCount,
        F64,
    }

    impl Display for UnitInfo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::UnitInfo> for UnitInfo {
        fn from(raw_unit: raw::UnitInfo) -> UnitInfo {
            match raw_unit {
                raw::UnitInfo::Meters => UnitInfo::Meters,
                raw::UnitInfo::Kilometers => UnitInfo::Kilometers,
                raw::UnitInfo::Seconds => UnitInfo::Seconds,
                raw::UnitInfo::Minutes => UnitInfo::Minutes,
                raw::UnitInfo::Hours => UnitInfo::Hours,
                raw::UnitInfo::KilometersPerHour => UnitInfo::KilometersPerHour,
                raw::UnitInfo::LaneCount => UnitInfo::LaneCount,
                raw::UnitInfo::F64 => UnitInfo::F64,
            }
        }
    }

    impl UnitInfo {
        pub fn convert(&self, to: &UnitInfo, raw_value: f64) -> f64 {
            let new_raw_value = match self {
                UnitInfo::Meters => match to {
                    UnitInfo::Meters | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Kilometers => Some(*Kilometers::from(Meters(raw_value))),
                    UnitInfo::Seconds
                    | UnitInfo::Minutes
                    | UnitInfo::Hours
                    | UnitInfo::KilometersPerHour
                    | UnitInfo::LaneCount => None,
                },
                UnitInfo::Kilometers => match to {
                    UnitInfo::Meters => Some(*Meters::from(Kilometers(raw_value))),
                    UnitInfo::Kilometers | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Seconds
                    | UnitInfo::Minutes
                    | UnitInfo::Hours
                    | UnitInfo::KilometersPerHour
                    | UnitInfo::LaneCount => None,
                },
                UnitInfo::Seconds => match to {
                    UnitInfo::Seconds | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Minutes => Some(*Minutes::from(Seconds(raw_value))),
                    UnitInfo::Hours => Some(*Hours::from(Seconds(raw_value))),
                    UnitInfo::Meters
                    | UnitInfo::Kilometers
                    | UnitInfo::KilometersPerHour
                    | UnitInfo::LaneCount => None,
                },
                UnitInfo::Minutes => match to {
                    UnitInfo::Minutes | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Seconds => Some(*Seconds::from(Minutes(raw_value))),
                    UnitInfo::Hours => Some(*Hours::from(Minutes(raw_value))),
                    UnitInfo::Meters
                    | UnitInfo::Kilometers
                    | UnitInfo::KilometersPerHour
                    | UnitInfo::LaneCount => None,
                },
                UnitInfo::Hours => match to {
                    UnitInfo::Hours | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Seconds => Some(*Seconds::from(Hours(raw_value))),
                    UnitInfo::Minutes => Some(*Minutes::from(Hours(raw_value))),
                    UnitInfo::Meters
                    | UnitInfo::Kilometers
                    | UnitInfo::KilometersPerHour
                    | UnitInfo::LaneCount => None,
                },
                UnitInfo::KilometersPerHour => match to {
                    UnitInfo::KilometersPerHour | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Meters
                    | UnitInfo::Kilometers
                    | UnitInfo::Seconds
                    | UnitInfo::Minutes
                    | UnitInfo::Hours
                    | UnitInfo::LaneCount => None,
                },
                UnitInfo::LaneCount => match to {
                    UnitInfo::LaneCount | UnitInfo::F64 => Some(raw_value),
                    UnitInfo::Meters
                    | UnitInfo::Kilometers
                    | UnitInfo::Seconds
                    | UnitInfo::Minutes
                    | UnitInfo::Hours
                    | UnitInfo::KilometersPerHour => None,
                },
                UnitInfo::F64 => Some(raw_value),
            };

            if let Some(new_raw_value) = new_raw_value {
                new_raw_value
            } else {
                panic!("Unit {} can't be converted to {}.", self, to)
            }
        }
    }
}
