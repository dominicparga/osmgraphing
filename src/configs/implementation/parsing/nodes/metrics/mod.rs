use serde::Deserialize;
pub mod raw;
use crate::configs::parsing::generating::nodes::metrics as gen;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum UnitInfo {
    Latitude,
    Longitude,
    Height,
}

impl From<raw::UnitInfo> for UnitInfo {
    fn from(raw_unit: raw::UnitInfo) -> UnitInfo {
        match raw_unit {
            raw::UnitInfo::Latitude => UnitInfo::Latitude,
            raw::UnitInfo::Longitude => UnitInfo::Longitude,
        }
    }
}

impl From<gen::UnitInfo> for UnitInfo {
    fn from(gen_unit: gen::UnitInfo) -> UnitInfo {
        match gen_unit {
            gen::UnitInfo::Latitude => UnitInfo::Latitude,
            gen::UnitInfo::Longitude => UnitInfo::Longitude,
            gen::UnitInfo::Height => UnitInfo::Height,
        }
    }
}
