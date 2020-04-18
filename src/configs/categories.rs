pub mod nodes {
    use crate::configs::{raw, SimpleId};
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

    impl From<raw::parsing::nodes::Category> for Category {
        fn from(raw_category: raw::parsing::nodes::Category) -> Category {
            match raw_category {
                raw::parsing::nodes::Category::Meta { info, id } => Category::Meta {
                    info: info.into(),
                    id,
                },
                raw::parsing::nodes::Category::Metric { unit, id } => Category::Metric {
                    unit: unit.into(),
                    id,
                },
                raw::parsing::nodes::Category::Ignored => Category::Ignored,
            }
        }
    }

    impl From<raw::parsing::generating::nodes::Category> for Category {
        fn from(raw_category: raw::parsing::generating::nodes::Category) -> Category {
            match raw_category {
                raw::parsing::generating::nodes::Category::Meta { info, id } => Category::Meta {
                    info: info.into(),
                    id,
                },
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

    impl From<raw::parsing::nodes::MetaInfo> for MetaInfo {
        fn from(raw_info: raw::parsing::nodes::MetaInfo) -> MetaInfo {
            match raw_info {
                raw::parsing::nodes::MetaInfo::NodeId => MetaInfo::NodeId,
                raw::parsing::nodes::MetaInfo::Level => MetaInfo::Level,
            }
        }
    }

    impl From<raw::parsing::generating::nodes::MetaInfo> for MetaInfo {
        fn from(raw_info: raw::parsing::generating::nodes::MetaInfo) -> MetaInfo {
            match raw_info {
                raw::parsing::generating::nodes::MetaInfo::NodeIdx => MetaInfo::NodeIdx,
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

    impl From<raw::parsing::nodes::UnitInfo> for UnitInfo {
        fn from(raw_unit: raw::parsing::nodes::UnitInfo) -> UnitInfo {
            match raw_unit {
                raw::parsing::nodes::UnitInfo::Latitude => UnitInfo::Latitude,
                raw::parsing::nodes::UnitInfo::Longitude => UnitInfo::Longitude,
            }
        }
    }
}

pub mod edges {
    use crate::configs::{raw, SimpleId};
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

    impl From<raw::parsing::edges::Category> for Category {
        fn from(raw_category: raw::parsing::edges::Category) -> Category {
            match raw_category {
                raw::parsing::edges::Category::Meta { info, id } => Category::Meta {
                    info: info.into(),
                    id,
                },
                raw::parsing::edges::Category::Metric { unit, id } => Category::Metric {
                    unit: unit.into(),
                    id,
                },
                raw::parsing::edges::Category::Ignored => Category::Ignored,
            }
        }
    }

    impl From<raw::parsing::generating::edges::Category> for Category {
        fn from(raw_category: raw::parsing::generating::edges::Category) -> Category {
            match raw_category {
                raw::parsing::generating::edges::Category::Meta { info, id } => Category::Meta {
                    info: info.into(),
                    id,
                },
            }
        }
    }

    #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
    pub enum MetaInfo {
        SrcId,
        SrcIdx,
        DstId,
        DstIdx,
        ShortcutIdx0,
        ShortcutIdx1,
    }

    impl Display for MetaInfo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl From<raw::parsing::edges::MetaInfo> for MetaInfo {
        fn from(raw_info: raw::parsing::edges::MetaInfo) -> MetaInfo {
            match raw_info {
                raw::parsing::edges::MetaInfo::SrcId => MetaInfo::SrcId,
                raw::parsing::edges::MetaInfo::DstId => MetaInfo::DstId,
                raw::parsing::edges::MetaInfo::ShortcutEdgeIdx0 => MetaInfo::ShortcutIdx0,
                raw::parsing::edges::MetaInfo::ShortcutEdgeIdx1 => MetaInfo::ShortcutIdx1,
            }
        }
    }

    impl From<raw::parsing::generating::edges::MetaInfo> for MetaInfo {
        fn from(raw_info: raw::parsing::generating::edges::MetaInfo) -> MetaInfo {
            match raw_info {
                raw::parsing::generating::edges::MetaInfo::SrcIdx => MetaInfo::SrcIdx,
                raw::parsing::generating::edges::MetaInfo::DstIdx => MetaInfo::DstIdx,
                raw::parsing::generating::edges::MetaInfo::ShortcutIdx0 => MetaInfo::ShortcutIdx0,
                raw::parsing::generating::edges::MetaInfo::ShortcutIdx1 => MetaInfo::ShortcutIdx1,
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

    impl From<raw::parsing::edges::UnitInfo> for UnitInfo {
        fn from(raw_unit: raw::parsing::edges::UnitInfo) -> UnitInfo {
            match raw_unit {
                raw::parsing::edges::UnitInfo::Meters => UnitInfo::Meters,
                raw::parsing::edges::UnitInfo::Kilometers => UnitInfo::Kilometers,
                raw::parsing::edges::UnitInfo::Seconds => UnitInfo::Seconds,
                raw::parsing::edges::UnitInfo::Minutes => UnitInfo::Minutes,
                raw::parsing::edges::UnitInfo::Hours => UnitInfo::Hours,
                raw::parsing::edges::UnitInfo::KilometersPerHour => UnitInfo::KilometersPerHour,
                raw::parsing::edges::UnitInfo::LaneCount => UnitInfo::LaneCount,
                raw::parsing::edges::UnitInfo::F64 => UnitInfo::F64,
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
