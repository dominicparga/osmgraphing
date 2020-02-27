use super::{speed::KilometersPerHour, time::Milliseconds, Metric, MetricU32};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign},
};

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
        0u32.into()
    }

    fn neg_inf() -> Meters {
        std::u32::MIN.into()
    }

    fn inf() -> Meters {
        std::u32::MAX.into()
    }
}

impl From<u8> for Meters {
    fn from(value: u8) -> Meters {
        Meters {
            value: value as u32,
        }
    }
}

impl From<u16> for Meters {
    fn from(value: u16) -> Meters {
        Meters {
            value: value as u32,
        }
    }
}

impl From<u32> for Meters {
    fn from(value: u32) -> Meters {
        Meters { value }
    }
}

impl From<MetricU32> for Meters {
    fn from(metric: MetricU32) -> Meters {
        (*metric).into()
    }
}

impl Into<MetricU32> for Meters {
    fn into(self) -> MetricU32 {
        self.value.into()
    }
}

impl Deref for Meters {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.value
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<Meters> for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
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

    fn mul(self, scale: u32) -> Meters {
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

    fn mul(self, scale: f64) -> Meters {
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
        KilometersPerHour::from(3600 * self.value / (*rhs))
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Meters {
    type Output = Milliseconds;

    fn div(self, rhs: KilometersPerHour) -> Milliseconds {
        Milliseconds::from(3600 * self.value / (*rhs))
    }
}
