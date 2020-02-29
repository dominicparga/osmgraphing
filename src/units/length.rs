use super::{speed::KilometersPerHour, time::Milliseconds, Metric};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign},
};

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Meters(pub u32);

impl Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

impl Metric for Meters {
    fn zero() -> Meters {
        Meters(0)
    }

    fn neg_inf() -> Meters {
        Meters(std::u32::MIN)
    }

    fn inf() -> Meters {
        Meters(std::u32::MAX)
    }
}

impl Deref for Meters {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl DerefMut for Meters {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<Meters> for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

impl AddAssign<Meters> for Meters {
    fn add_assign(&mut self, other: Meters) {
        self.0 += other.0;
    }
}

impl Mul<u32> for Meters {
    type Output = Meters;

    fn mul(self, scale: u32) -> Meters {
        Meters(self.0 * scale)
    }
}

impl MulAssign<u32> for Meters {
    fn mul_assign(&mut self, scale: u32) {
        self.0 *= scale;
    }
}

impl Mul<f64> for Meters {
    type Output = Meters;

    fn mul(self, scale: f64) -> Meters {
        let new_value = scale * (self.0 as f64) * scale;
        Meters(new_value as u32)
    }
}

impl MulAssign<f64> for Meters {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.0 as f64);
        self.0 = new_value as u32;
    }
}

impl Div<u32> for Meters {
    type Output = Meters;

    fn div(self, rhs: u32) -> Meters {
        Meters(self.0 / rhs)
    }
}

impl DivAssign<u32> for Meters {
    fn div_assign(&mut self, rhs: u32) {
        self.0 /= rhs;
    }
}

/// v = s / t
impl Div<Milliseconds> for Meters {
    type Output = KilometersPerHour;

    fn div(self, rhs: Milliseconds) -> KilometersPerHour {
        KilometersPerHour(3600 * self.0 / (*rhs))
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Meters {
    type Output = Milliseconds;

    fn div(self, rhs: KilometersPerHour) -> Milliseconds {
        Milliseconds(3600 * self.0 / (*rhs))
    }
}
