use super::{speed::KilometersPerHour, time::Seconds};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Kilometers(pub f32);

impl Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}

impl Deref for Kilometers {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl DerefMut for Kilometers {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}

impl Add<Kilometers> for Kilometers {
    type Output = Kilometers;

    fn add(self, other: Kilometers) -> Kilometers {
        Kilometers(self.0 + other.0)
    }
}

impl AddAssign<Kilometers> for Kilometers {
    fn add_assign(&mut self, other: Kilometers) {
        self.0 += other.0;
    }
}

impl Sub<Kilometers> for Kilometers {
    type Output = Kilometers;

    fn sub(self, other: Kilometers) -> Kilometers {
        Kilometers(self.0 - other.0)
    }
}

impl SubAssign<Kilometers> for Kilometers {
    fn sub_assign(&mut self, other: Kilometers) {
        self.0 -= other.0;
    }
}

/// v = s / t
impl Div<Seconds> for Kilometers {
    type Output = KilometersPerHour;

    fn div(self, duration: Seconds) -> KilometersPerHour {
        KilometersPerHour(3_600.0 * self.0 / (*duration))
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Kilometers {
    type Output = Seconds;

    fn div(self, speed: KilometersPerHour) -> Seconds {
        Seconds(3_600.0 * self.0 / (*speed))
    }
}
