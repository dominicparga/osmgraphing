//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::{length::Meters, speed::KilometersPerHour, Metric, MetricU32};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Milliseconds(pub u32);

impl Display for Milliseconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ms", self.0)
    }
}

impl Metric for Milliseconds {
    fn zero() -> Milliseconds {
        0u32.into()
    }

    fn neg_inf() -> Milliseconds {
        std::u32::MIN.into()
    }

    fn inf() -> Milliseconds {
        std::u32::MAX.into()
    }
}

impl From<u8> for Milliseconds {
    fn from(value: u8) -> Milliseconds {
        Milliseconds(value as u32)
    }
}

impl From<u16> for Milliseconds {
    fn from(value: u16) -> Milliseconds {
        Milliseconds(value as u32)
    }
}

impl From<u32> for Milliseconds {
    fn from(value: u32) -> Milliseconds {
        Milliseconds(value)
    }
}

impl From<MetricU32> for Milliseconds {
    fn from(metric: MetricU32) -> Milliseconds {
        Milliseconds(*metric)
    }
}

impl Into<MetricU32> for Milliseconds {
    fn into(self) -> MetricU32 {
        MetricU32(self.0)
    }
}

impl Deref for Milliseconds {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl DerefMut for Milliseconds {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<Milliseconds> for Milliseconds {
    type Output = Milliseconds;

    fn add(self, other: Milliseconds) -> Milliseconds {
        Milliseconds(self.0 + other.0)
    }
}

impl AddAssign<Milliseconds> for Milliseconds {
    fn add_assign(&mut self, other: Milliseconds) {
        self.0 += other.0;
    }
}

impl Mul<u32> for Milliseconds {
    type Output = Milliseconds;

    fn mul(self, scale: u32) -> Milliseconds {
        Milliseconds(self.0 * scale)
    }
}

impl MulAssign<u32> for Milliseconds {
    fn mul_assign(&mut self, scale: u32) {
        self.0 *= scale;
    }
}

impl Mul<f64> for Milliseconds {
    type Output = Milliseconds;

    fn mul(self, scale: f64) -> Milliseconds {
        let new_value = scale * (self.0 as f64) * scale;
        Milliseconds(new_value as u32)
    }
}

impl MulAssign<f64> for Milliseconds {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.0 as f64);
        self.0 = new_value as u32;
    }
}

/// s = v * t
impl Mul<KilometersPerHour> for Milliseconds {
    type Output = Meters;

    fn mul(self, rhs: KilometersPerHour) -> Meters {
        let time = self.0;
        let speed = *rhs;
        Meters::from(speed * time / 3_600)
    }
}
