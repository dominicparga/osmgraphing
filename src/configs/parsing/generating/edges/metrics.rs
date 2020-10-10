use crate::{configs::SimpleId, defaults::capacity::DimVec, helpers::err};
use kissunits::{
    distance::{Kilometers, Meters},
    speed::KilometersPerHour,
    time::{Hours, Minutes, Seconds},
};
use serde::Deserialize;

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

impl From<ProtoCategory> for Category {
    fn from(proto_category: ProtoCategory) -> Category {
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

impl From<ProtoUnitInfo> for UnitInfo {
    fn from(proto_unit: ProtoUnitInfo) -> UnitInfo {
        match proto_unit {
            ProtoUnitInfo::Meters => UnitInfo::Meters,
            ProtoUnitInfo::Kilometers => UnitInfo::Kilometers,
            ProtoUnitInfo::Seconds => UnitInfo::Seconds,
            ProtoUnitInfo::Minutes => UnitInfo::Minutes,
            ProtoUnitInfo::Hours => UnitInfo::Hours,
            ProtoUnitInfo::KilometersPerHour => UnitInfo::KilometersPerHour,
            ProtoUnitInfo::LaneCount => UnitInfo::LaneCount,
            ProtoUnitInfo::F64 => UnitInfo::F64,
        }
    }
}

impl UnitInfo {
    pub fn try_convert(&self, to: &UnitInfo, raw_value: f64) -> err::Result<f64> {
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
            Ok(new_raw_value)
        } else {
            Err(format!("Unit {:?} can't be converted to {:?}.", self, to).into())
        }
    }

    pub fn try_calc(
        &self,
        unit_a: &UnitInfo,
        raw_a: f64,
        unit_b: &UnitInfo,
        raw_b: f64,
    ) -> err::Result<f64> {
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
            Ok(raw_result)
        } else {
            Err(format!(
                "{:?} can't be calculated by {:?} and {:?}.",
                self, unit_a, unit_b
            )
            .into())
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProtoCategory {
    pub unit: ProtoUnitInfo,
    pub id: SimpleId,
}

impl From<RawCategory> for ProtoCategory {
    fn from(raw_category: RawCategory) -> ProtoCategory {
        ProtoCategory {
            unit: ProtoUnitInfo::from(raw_category.unit),
            id: raw_category.id,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ProtoUnitInfo {
    Meters,
    Kilometers,
    Seconds,
    Minutes,
    Hours,
    KilometersPerHour,
    LaneCount,
    F64,
}

impl From<RawUnitInfo> for ProtoUnitInfo {
    fn from(raw_unit: RawUnitInfo) -> ProtoUnitInfo {
        match raw_unit {
            RawUnitInfo::Meters => ProtoUnitInfo::Meters,
            RawUnitInfo::Kilometers => ProtoUnitInfo::Kilometers,
            RawUnitInfo::Seconds => ProtoUnitInfo::Seconds,
            RawUnitInfo::Minutes => ProtoUnitInfo::Minutes,
            RawUnitInfo::Hours => ProtoUnitInfo::Hours,
            RawUnitInfo::KilometersPerHour => ProtoUnitInfo::KilometersPerHour,
            RawUnitInfo::LaneCount => ProtoUnitInfo::LaneCount,
            RawUnitInfo::F64 => ProtoUnitInfo::F64,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawCategory {
    pub unit: RawUnitInfo,
    pub id: SimpleId,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum RawUnitInfo {
    Meters,
    Kilometers,
    Seconds,
    Minutes,
    Hours,
    KilometersPerHour,
    LaneCount,
    F64,
}
