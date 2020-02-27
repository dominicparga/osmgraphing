//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::{length::Meters, time::Milliseconds, Metric, MetricU32};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct KilometersPerHour(pub u32);

impl Display for KilometersPerHour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km/h", self.0)
    }
}

impl Metric for KilometersPerHour {
    fn zero() -> KilometersPerHour {
        KilometersPerHour(0)
    }

    fn neg_inf() -> KilometersPerHour {
        KilometersPerHour(std::u32::MIN)
    }

    fn inf() -> KilometersPerHour {
        KilometersPerHour(std::u32::MAX)
    }
}

impl From<u8> for KilometersPerHour {
    fn from(value: u8) -> KilometersPerHour {
        KilometersPerHour(value as u32)
    }
}

impl From<u16> for KilometersPerHour {
    fn from(value: u16) -> KilometersPerHour {
        KilometersPerHour(value as u32)
    }
}

impl From<u32> for KilometersPerHour {
    fn from(value: u32) -> KilometersPerHour {
        KilometersPerHour(value)
    }
}

impl From<MetricU32> for KilometersPerHour {
    fn from(metric: MetricU32) -> KilometersPerHour {
        (*metric).into()
    }
}

impl Into<MetricU32> for KilometersPerHour {
    fn into(self) -> MetricU32 {
        self.0.into()
    }
}

impl Deref for KilometersPerHour {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl DerefMut for KilometersPerHour {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<KilometersPerHour> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn add(self, other: KilometersPerHour) -> KilometersPerHour {
        KilometersPerHour(self.0 + other.0)
    }
}

impl AddAssign<KilometersPerHour> for KilometersPerHour {
    fn add_assign(&mut self, other: KilometersPerHour) {
        self.0 += other.0;
    }
}

impl Mul<u32> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn mul(self, scale: u32) -> KilometersPerHour {
        KilometersPerHour(scale * self.0)
    }
}

impl MulAssign<u32> for KilometersPerHour {
    fn mul_assign(&mut self, scale: u32) {
        self.0 *= scale;
    }
}

/// s = v * t
impl Mul<Milliseconds> for KilometersPerHour {
    type Output = Meters;

    fn mul(self, rhs: Milliseconds) -> Meters {
        let speed = self.0 as u32;
        let time = *rhs;
        Meters::from(speed * time / 3_600)
    }
}

impl Div<u32> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn div(self, rhs: u32) -> KilometersPerHour {
        KilometersPerHour(self.0 / rhs)
    }
}

impl DivAssign<u32> for KilometersPerHour {
    fn div_assign(&mut self, rhs: u32) {
        self.0 /= rhs;
    }
}

impl Mul<f64> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn mul(self, scale: f64) -> KilometersPerHour {
        let new_value = scale * (self.0 as f64) * scale;
        KilometersPerHour(new_value as u32)
    }
}

impl MulAssign<f64> for KilometersPerHour {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.0 as f64);
        self.0 = new_value as u32;
    }
}
