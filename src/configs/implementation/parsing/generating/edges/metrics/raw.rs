use crate::configs::SimpleId;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub unit: UnitInfo,
    pub id: SimpleId,
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
