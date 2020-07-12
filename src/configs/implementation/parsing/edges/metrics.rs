use crate::{
    configs::{
        parsing::generating::edges::{merge::metrics as merge_gen, metrics as gen},
        SimpleId,
    },
    defaults::capacity::DimVec,
    helpers::err,
    network::MetricIdx,
};
use kissunits::{
    distance::{Kilometers, Meters},
    time::{Hours, Minutes, Seconds},
};
use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    pub units: DimVec<UnitInfo>,
    pub ids: DimVec<SimpleId>,
}

impl Config {
    pub fn try_idx_of<S>(&self, id: S) -> err::Result<MetricIdx>
    where
        S: AsRef<str>,
    {
        Ok(MetricIdx(
            match self.ids.iter().position(|self_id| self_id.0 == id.as_ref()) {
                Some(idx) => idx,
                None => {
                    return Err(format!(
                        "Metric-id {} should be existent in graph, but isn't.",
                        id.as_ref()
                    )
                    .into())
                }
            },
        ))
    }

    /// Panics if id doesn't exist
    pub fn idx_of<S>(&self, id: S) -> MetricIdx
    where
        S: AsRef<str>,
    {
        match self.try_idx_of(&id) {
            Ok(idx) => idx,
            Err(msg) => panic!("{}", msg),
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
impl From<merge_gen::UnitInfo> for UnitInfo {
    fn from(gen_unit: merge_gen::UnitInfo) -> UnitInfo {
        match gen_unit {
            merge_gen::UnitInfo::Meters => UnitInfo::Meters,
            merge_gen::UnitInfo::Kilometers => UnitInfo::Kilometers,
            merge_gen::UnitInfo::Seconds => UnitInfo::Seconds,
            merge_gen::UnitInfo::Minutes => UnitInfo::Minutes,
            merge_gen::UnitInfo::Hours => UnitInfo::Hours,
            merge_gen::UnitInfo::KilometersPerHour => UnitInfo::KilometersPerHour,
            merge_gen::UnitInfo::LaneCount => UnitInfo::LaneCount,
            merge_gen::UnitInfo::F64 => UnitInfo::F64,
        }
    }
}

impl UnitInfo {
    pub fn try_convert(&self, to: &UnitInfo, raw_value: f64) -> err::Result<f64> {
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
            Ok(new_raw_value)
        } else {
            Err(format!("Unit {:?} can't be converted to {:?}.", self, to).into())
        }
    }

    pub fn convert(&self, to: &UnitInfo, raw_value: f64) -> f64 {
        match self.try_convert(to, raw_value) {
            Ok(new_raw_value) => new_raw_value,
            Err(msg) => panic!("{}", msg),
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
    fn from(raw_info: RawUnitInfo) -> ProtoUnitInfo {
        match raw_info {
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
