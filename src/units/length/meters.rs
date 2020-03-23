use crate::{
    helpers::ApproxEq,
    units::{
        length::Kilometers,
        speed::KilometersPerHour,
        time::{Hours, Minutes, Seconds},
    },
};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Meters(pub f64);

impl Meters {
    pub fn new(m: f64) -> Meters {
        Meters(m)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

impl From<Kilometers> for Meters {
    fn from(kilometers: Kilometers) -> Meters {
        Meters(kilometers.0 * 1_000.0)
    }
}

impl From<&Kilometers> for Meters {
    fn from(kilometers: &Kilometers) -> Meters {
        Meters::from(*kilometers)
    }
}

impl Deref for Meters {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Meters {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl ApproxEq<Meters> for Meters {
    fn approx_eq(&self, other: &Meters) -> bool {
        self.0.approx_eq(other)
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
        KilometersPerHour((*Kilometers::from(self)) / (*Hours::from(duration)))
    }
}

/// v = s / t
impl Div<Minutes> for Meters {
    type Output = KilometersPerHour;

    fn div(self, duration: Minutes) -> KilometersPerHour {
        KilometersPerHour((*Kilometers::from(self)) / (*Hours::from(duration)))
    }
}

/// v = s / t
impl Div<Hours> for Meters {
    type Output = KilometersPerHour;

    fn div(self, duration: Hours) -> KilometersPerHour {
        KilometersPerHour((*Kilometers::from(self)) / (*duration))
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Meters {
    type Output = Hours;

    fn div(self, speed: KilometersPerHour) -> Hours {
        Hours((*Kilometers::from(self)) / (*speed))
    }
}
