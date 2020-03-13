use super::{length::Kilometers, speed::KilometersPerHour};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Seconds(pub f64);

impl Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} s", self.0)
    }
}

impl Deref for Seconds {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Seconds {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl Add<Seconds> for Seconds {
    type Output = Seconds;

    fn add(self, other: Seconds) -> Seconds {
        Seconds(self.0 + other.0)
    }
}

impl AddAssign<Seconds> for Seconds {
    fn add_assign(&mut self, other: Seconds) {
        self.0 += other.0;
    }
}

impl Sub<Seconds> for Seconds {
    type Output = Seconds;

    fn sub(self, other: Seconds) -> Seconds {
        Seconds(self.0 - other.0)
    }
}

impl SubAssign<Seconds> for Seconds {
    fn sub_assign(&mut self, other: Seconds) {
        self.0 -= other.0;
    }
}

/// s = v * t
impl Mul<KilometersPerHour> for Seconds {
    type Output = Kilometers;

    fn mul(self, speed: KilometersPerHour) -> Kilometers {
        Kilometers((*speed) * self.0 / 3_600.0)
    }
}
