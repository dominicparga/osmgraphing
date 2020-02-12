//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::{length::Meters, time::Milliseconds, Metric};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct KilometersPerHour {
    value: u16,
}

impl Display for KilometersPerHour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km/h", self.value)
    }
}

impl Metric for KilometersPerHour {
    fn zero() -> KilometersPerHour {
        KilometersPerHour { value: 0 }
    }

    fn neg_inf() -> KilometersPerHour {
        KilometersPerHour {
            value: std::u16::MIN,
        }
    }

    fn inf() -> KilometersPerHour {
        KilometersPerHour {
            value: std::u16::MAX,
        }
    }
}

impl KilometersPerHour {
    pub fn value(&self) -> u16 {
        self.value
    }
}

//--------------------------------------------------------------------------------------------//
// conversion from/to

impl Into<u16> for KilometersPerHour {
    fn into(self) -> u16 {
        self.value
    }
}

/// Note that the result could have rounding errors due to up-scaling (* 1000.0) and cutting afterwards (f64 -> u16)
impl From<u16> for KilometersPerHour {
    fn from(value: u16) -> Self {
        KilometersPerHour { value: value }
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<KilometersPerHour> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn add(self, other: KilometersPerHour) -> Self {
        KilometersPerHour {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<KilometersPerHour> for KilometersPerHour {
    fn add_assign(&mut self, other: KilometersPerHour) {
        self.value += other.value;
    }
}

impl Mul<u16> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn mul(self, scale: u16) -> Self {
        KilometersPerHour {
            value: scale * self.value,
        }
    }
}

impl MulAssign<u16> for KilometersPerHour {
    fn mul_assign(&mut self, scale: u16) {
        self.value *= scale;
    }
}

/// s = v * t
impl Mul<Milliseconds> for KilometersPerHour {
    type Output = Meters;

    fn mul(self, rhs: Milliseconds) -> Meters {
        let speed = self.value as u32;
        let time = rhs.value();
        (speed * time / 3_600).into()
    }
}

impl Div<u16> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn div(self, rhs: u16) -> KilometersPerHour {
        KilometersPerHour {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<u16> for KilometersPerHour {
    fn div_assign(&mut self, rhs: u16) {
        self.value /= rhs;
    }
}

impl Mul<f64> for KilometersPerHour {
    type Output = KilometersPerHour;

    fn mul(self, scale: f64) -> Self {
        let new_value = scale * (self.value as f64) * scale;
        KilometersPerHour {
            value: new_value as u16,
        }
    }
}

impl MulAssign<f64> for KilometersPerHour {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.value as f64);
        self.value = new_value as u16;
    }
}
