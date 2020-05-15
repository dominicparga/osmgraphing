use crate::{configs::SimpleId, defaults::capacity::DimVec};
use kissunits::{
    distance::{Kilometers, Meters},
    speed::KilometersPerHour,
    time::{Hours, Minutes, Seconds},
};
use serde::Deserialize;
pub mod proto;
pub mod raw;

#[derive(Debug)]
pub struct Config {
    pub units: DimVec<UnitInfo>,
    pub ids: DimVec<SimpleId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub unit: UnitInfo,
    pub id: SimpleId,
}

impl From<proto::Category> for Category {
    fn from(proto_category: proto::Category) -> Category {
        Category {
            unit: proto_category.unit.into(),
            id: proto_category.id,
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

impl From<proto::UnitInfo> for UnitInfo {
    fn from(proto_unit: proto::UnitInfo) -> UnitInfo {
        match proto_unit {
            proto::UnitInfo::Meters => UnitInfo::Meters,
            proto::UnitInfo::Kilometers => UnitInfo::Kilometers,
            proto::UnitInfo::Seconds => UnitInfo::Seconds,
            proto::UnitInfo::Minutes => UnitInfo::Minutes,
            proto::UnitInfo::Hours => UnitInfo::Hours,
            proto::UnitInfo::KilometersPerHour => UnitInfo::KilometersPerHour,
            proto::UnitInfo::LaneCount => UnitInfo::LaneCount,
            proto::UnitInfo::F64 => UnitInfo::F64,
        }
    }
}

impl UnitInfo {
    pub fn convert(&self, to: &UnitInfo, raw_value: f64) -> f64 {
        let new_raw_value = match self {
            UnitInfo::Meters => match to {
                UnitInfo::Meters => Some(raw_value),
                UnitInfo::Kilometers => Some(*Kilometers::from(Meters(raw_value))),
                UnitInfo::Seconds => None,
                UnitInfo::Minutes => None,
                UnitInfo::Hours => None,
                UnitInfo::KilometersPerHour => None,
                UnitInfo::LaneCount => None,
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::Kilometers => match to {
                UnitInfo::Meters => Some(*Meters::from(Kilometers(raw_value))),
                UnitInfo::Kilometers => Some(raw_value),
                UnitInfo::Seconds => None,
                UnitInfo::Minutes => None,
                UnitInfo::Hours => None,
                UnitInfo::KilometersPerHour => None,
                UnitInfo::LaneCount => None,
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::Seconds => match to {
                UnitInfo::Meters => None,
                UnitInfo::Kilometers => None,
                UnitInfo::Seconds => Some(raw_value),
                UnitInfo::Minutes => Some(*Minutes::from(Seconds(raw_value))),
                UnitInfo::Hours => Some(*Hours::from(Seconds(raw_value))),
                UnitInfo::KilometersPerHour => None,
                UnitInfo::LaneCount => None,
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::Minutes => match to {
                UnitInfo::Meters => None,
                UnitInfo::Kilometers => None,
                UnitInfo::Seconds => Some(*Seconds::from(Minutes(raw_value))),
                UnitInfo::Minutes => Some(raw_value),
                UnitInfo::Hours => Some(*Hours::from(Minutes(raw_value))),
                UnitInfo::KilometersPerHour => None,
                UnitInfo::LaneCount => None,
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::Hours => match to {
                UnitInfo::Meters => None,
                UnitInfo::Kilometers => None,
                UnitInfo::Seconds => Some(*Seconds::from(Hours(raw_value))),
                UnitInfo::Minutes => Some(*Minutes::from(Hours(raw_value))),
                UnitInfo::Hours => Some(raw_value),
                UnitInfo::KilometersPerHour => None,
                UnitInfo::LaneCount => None,
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::KilometersPerHour => match to {
                UnitInfo::Meters => None,
                UnitInfo::Kilometers => None,
                UnitInfo::Seconds => None,
                UnitInfo::Minutes => None,
                UnitInfo::Hours => None,
                UnitInfo::KilometersPerHour => Some(raw_value),
                UnitInfo::LaneCount => None,
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::LaneCount => match to {
                UnitInfo::Meters => None,
                UnitInfo::Kilometers => None,
                UnitInfo::Seconds => None,
                UnitInfo::Minutes => None,
                UnitInfo::Hours => None,
                UnitInfo::KilometersPerHour => None,
                UnitInfo::LaneCount => Some(raw_value),
                UnitInfo::F64 => Some(raw_value),
            },
            UnitInfo::F64 => Some(raw_value),
        };

        if let Some(new_raw_value) = new_raw_value {
            new_raw_value
        } else {
            panic!("Unit {:?} can't be converted to {:?}.", self, to)
        }
    }

    pub fn calc(&self, unit_a: &UnitInfo, raw_a: f64, unit_b: &UnitInfo, raw_b: f64) -> f64 {
        let raw_result = match self {
            UnitInfo::Meters => None,
            UnitInfo::Kilometers => None,
            UnitInfo::Seconds => {
                if match unit_a {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => true,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => false,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } && match unit_b {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => false,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => true,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } {
                    Some(*Seconds::from(Kilometers(raw_a) / KilometersPerHour(raw_b)))
                } else {
                    None
                }
            }
            UnitInfo::Minutes => {
                if match unit_a {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => true,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => false,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } && match unit_b {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => false,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => true,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } {
                    Some(*Minutes::from(Kilometers(raw_a) / KilometersPerHour(raw_b)))
                } else {
                    None
                }
            }
            UnitInfo::Hours => {
                if match unit_a {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => true,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => false,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } && match unit_b {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => false,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => true,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } {
                    Some(*Hours::from(Kilometers(raw_a) / KilometersPerHour(raw_b)))
                } else {
                    None
                }
            }
            UnitInfo::KilometersPerHour => {
                if match unit_a {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => true,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => false,
                    UnitInfo::KilometersPerHour => false,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } && match unit_b {
                    UnitInfo::Meters => false,
                    UnitInfo::Kilometers => false,
                    UnitInfo::Seconds => false,
                    UnitInfo::Minutes => false,
                    UnitInfo::Hours => true,
                    UnitInfo::KilometersPerHour => false,
                    UnitInfo::LaneCount => false,
                    UnitInfo::F64 => false,
                } {
                    Some(*KilometersPerHour::from(Kilometers(raw_a) / Hours(raw_b)))
                } else {
                    None
                }
            }
            UnitInfo::LaneCount => None,
            UnitInfo::F64 => None,
        };

        if let Some(raw_result) = raw_result {
            raw_result
        } else {
            panic!(
                "{:?} can't be calculated by {:?} and {:?}.",
                self, unit_a, unit_b
            )
        }
    }
}
