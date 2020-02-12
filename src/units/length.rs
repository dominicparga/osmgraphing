//------------------------------------------------------------------------------------------------//
// other modules

use super::speed::KilometersPerHour;
use super::time::Milliseconds;
use super::Metric;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Meters {
    value: u32,
}

impl Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.value)
    }
}

impl Metric for Meters {
    fn zero() -> Meters {
        Meters { value: 0 }
    }

    fn neg_inf() -> Meters {
        Meters {
            value: std::u32::MIN,
        }
    }

    fn inf() -> Meters {
        Meters {
            value: std::u32::MAX,
        }
    }
}

impl Meters {
    pub fn value(&self) -> u32 {
        self.value
    }
}

//--------------------------------------------------------------------------------------------//
// conversion from/to

impl Into<f64> for Meters {
    fn into(self) -> f64 {
        self.value as f64
    }
}

impl From<f64> for Meters {
    fn from(value: f64) -> Self {
        Meters {
            value: value as u32,
        }
    }
}

impl Into<u32> for Meters {
    fn into(self) -> u32 {
        self.value
    }
}

/// Note that the result could have rounding errors due to up-scaling (* 1000.0) and cutting afterwards (f64 -> u32)
impl From<u32> for Meters {
    fn from(value: u32) -> Self {
        Meters { value: value }
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<Meters> for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Self {
        Meters {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<Meters> for Meters {
    fn add_assign(&mut self, other: Meters) {
        self.value += other.value;
    }
}

impl Mul<u32> for Meters {
    type Output = Meters;

    fn mul(self, scale: u32) -> Self {
        Meters {
            value: scale * self.value,
        }
    }
}

impl MulAssign<u32> for Meters {
    fn mul_assign(&mut self, scale: u32) {
        self.value *= scale;
    }
}

impl Mul<f64> for Meters {
    type Output = Meters;

    fn mul(self, scale: f64) -> Self {
        let new_value = scale * (self.value as f64) * scale;
        Meters {
            value: new_value as u32,
        }
    }
}

impl MulAssign<f64> for Meters {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.value as f64);
        self.value = new_value as u32;
    }
}

impl Div<u32> for Meters {
    type Output = Meters;

    fn div(self, rhs: u32) -> Meters {
        Meters {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<u32> for Meters {
    fn div_assign(&mut self, rhs: u32) {
        self.value /= rhs;
    }
}

/// v = s / t
impl Div<Milliseconds> for Meters {
    type Output = KilometersPerHour;

    fn div(self, rhs: Milliseconds) -> KilometersPerHour {
        KilometersPerHour::new((3600 * self.value / rhs.value()) as u16)
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Meters {
    type Output = Milliseconds;

    fn div(self, rhs: KilometersPerHour) -> Milliseconds {
        Milliseconds::new(3600 * self.value / (rhs.value() as u32))
    }
}
