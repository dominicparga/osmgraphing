use crate::configs::parsing::generating::nodes::metrics as gen;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum UnitInfo {
    Latitude,
    Longitude,
    Height,
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

impl From<ProtoUnitInfo> for UnitInfo {
    fn from(proto_unit: ProtoUnitInfo) -> UnitInfo {
        match proto_unit {
            ProtoUnitInfo::Latitude => UnitInfo::Latitude,
            ProtoUnitInfo::Longitude => UnitInfo::Longitude,
            ProtoUnitInfo::Height => UnitInfo::Height,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ProtoUnitInfo {
    Latitude,
    Longitude,
    Height,
}

impl From<RawUnitInfo> for ProtoUnitInfo {
    fn from(raw_unit: RawUnitInfo) -> ProtoUnitInfo {
        match raw_unit {
            RawUnitInfo::Latitude => ProtoUnitInfo::Latitude,
            RawUnitInfo::Longitude => ProtoUnitInfo::Longitude,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum RawUnitInfo {
    Latitude,
    Longitude,
}
