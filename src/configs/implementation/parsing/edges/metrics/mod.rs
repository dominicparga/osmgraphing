use crate::{
    configs::{parsing::generating::edges::metrics as gen, SimpleId},
    defaults::capacity::DimVec,
};
use kissunits::{
    distance::{Kilometers, Meters},
    time::{Hours, Minutes, Seconds},
};
use serde::Deserialize;
pub mod raw;

#[derive(Debug)]
pub struct Config {
    pub units: DimVec<UnitInfo>,
    pub ids: DimVec<SimpleId>,
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

impl From<gen::UnitInfo> for UnitInfo {
    fn from(gen_unit: gen::UnitInfo) -> UnitInfo {
        match gen_unit {
            gen::UnitInfo::Meters => UnitInfo::Meters,
            gen::UnitInfo::Kilometers => UnitInfo::Kilometers,
            gen::UnitInfo::Seconds => UnitInfo::Seconds,
            gen::UnitInfo::Minutes => UnitInfo::Minutes,
            gen::UnitInfo::Hours => UnitInfo::Hours,
            gen::UnitInfo::KilometersPerHour => UnitInfo::KilometersPerHour,
            gen::UnitInfo::LaneCount => UnitInfo::LaneCount,
            gen::UnitInfo::F64 => UnitInfo::F64,
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
            panic!("Unit {:?} can't be converted to {:?}.", self, to)
        }
    }
}
