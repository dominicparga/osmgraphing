use super::{speed::KilometersPerHour, time::Seconds};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Meters(pub f32);

impl Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

impl Deref for Meters {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl DerefMut for Meters {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}

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

impl Sub<Meters> for Meters {
    type Output = Meters;

    fn sub(self, other: Meters) -> Meters {
        Meters(self.0 - other.0)
    }
}

impl SubAssign<Meters> for Meters {
    fn sub_assign(&mut self, other: Meters) {
        self.0 -= other.0;
    }
}

/// v = s / t
impl Div<Seconds> for Meters {
    type Output = KilometersPerHour;

    fn div(self, duration: Seconds) -> KilometersPerHour {
        KilometersPerHour(3_600.0 * self.0 / (*duration))
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Meters {
    type Output = Seconds;

    fn div(self, speed: KilometersPerHour) -> Seconds {
        Seconds(3.6 * self.0 / (*speed))
    }
}
