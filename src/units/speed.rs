//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::{length::Meters, time::Milliseconds, Metric, MetricU32};
use std::{
    fmt,
    fmt::Display,
    ops::Deref,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct KilometersPerHour {
    value: u32,
}

impl Display for KilometersPerHour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km/h", self.value)
    }
}

impl Metric for KilometersPerHour {
    fn zero() -> KilometersPerHour {
        0u32.into()
    }

    fn neg_inf() -> KilometersPerHour {
        std::u32::MIN.into()
    }

    fn inf() -> KilometersPerHour {
        std::u32::MAX.into()
    }
}

impl From<u8> for KilometersPerHour {
    fn from(value: u8) -> KilometersPerHour {
        KilometersPerHour {
            value: value as u32,
        }
    }
}

impl From<u16> for KilometersPerHour {
    fn from(value: u16) -> KilometersPerHour {
        KilometersPerHour {
            value: value as u32,
        }
    }
}

impl From<u32> for KilometersPerHour {
    fn from(value: u32) -> KilometersPerHour {
        KilometersPerHour { value }
    }
}

impl From<MetricU32> for KilometersPerHour {
    fn from(metric: MetricU32) -> KilometersPerHour {
        (*metric).into()
    }
}

impl Into<MetricU32> for KilometersPerHour {
    fn into(self) -> MetricU32 {
        self.value.into()
    }
}

impl Deref for KilometersPerHour {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.value
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<KilometersPerHour> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn add(self, other: KilometersPerHour) -> KilometersPerHour {
        (self.value + other.value).into()
    }
}

impl AddAssign<KilometersPerHour> for KilometersPerHour {
    fn add_assign(&mut self, other: KilometersPerHour) {
        self.value += other.value;
    }
}

impl Mul<u32> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn mul(self, scale: u32) -> KilometersPerHour {
        (scale * self.value).into()
    }
}

impl MulAssign<u32> for KilometersPerHour {
    fn mul_assign(&mut self, scale: u32) {
        self.value *= scale;
    }
}

/// s = v * t
impl Mul<Milliseconds> for KilometersPerHour {
    type Output = Meters;

    fn mul(self, rhs: Milliseconds) -> Meters {
        let speed = self.value as u32;
        let time = *rhs;
        Meters::from(speed * time / 3_600)
    }
}

impl Div<u32> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn div(self, rhs: u32) -> KilometersPerHour {
        (self.value / rhs).into()
    }
}

impl DivAssign<u32> for KilometersPerHour {
    fn div_assign(&mut self, rhs: u32) {
        self.value /= rhs;
    }
}

impl Mul<f64> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn mul(self, scale: f64) -> KilometersPerHour {
        let new_value = scale * (self.value as f64) * scale;
        let new_value = new_value as u32;
        new_value.into()
    }
}

impl MulAssign<f64> for KilometersPerHour {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.value as f64);
        self.value = new_value as u32;
    }
}
