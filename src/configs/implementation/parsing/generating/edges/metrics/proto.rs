use crate::configs::{implementation::parsing::generating::edges::metrics::raw, SimpleId};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub unit: UnitInfo,
    pub id: SimpleId,
}

impl From<raw::Category> for Category {
    fn from(raw_category: raw::Category) -> Category {
        Category {
            unit: raw_category.unit.into(),
            id: raw_category.id,
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
