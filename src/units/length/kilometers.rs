use crate::{
    helpers::ApproxEq,
    units::{
        length::Meters,
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
pub struct Kilometers(pub f64);

impl Kilometers {
    pub fn new(km: f64) -> Kilometers {
        Kilometers(km)
    }
}

impl Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}

impl From<Meters> for Kilometers {
    fn from(meters: Meters) -> Kilometers {
        Kilometers(meters.0 / 1_000.0)
    }
}

impl From<&Meters> for Kilometers {
    fn from(meters: &Meters) -> Kilometers {
        Kilometers::from(*meters)
    }
}

impl Deref for Kilometers {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Kilometers {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl ApproxEq<Kilometers> for Kilometers {
    fn approx_eq(&self, other: &Kilometers) -> bool {
        self.0.approx_eq(other)
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
        KilometersPerHour(self.0 / (*Hours::from(duration)))
    }
}

/// v = s / t
impl Div<Minutes> for Kilometers {
    type Output = KilometersPerHour;

    fn div(self, duration: Minutes) -> KilometersPerHour {
        KilometersPerHour(self.0 / (*Hours::from(duration)))
    }
}

/// v = s / t
impl Div<Hours> for Kilometers {
    type Output = KilometersPerHour;

    fn div(self, duration: Hours) -> KilometersPerHour {
        KilometersPerHour(self.0 / (*duration))
    }
}

/// t = s / v
impl Div<KilometersPerHour> for Kilometers {
    type Output = Hours;

    fn div(self, speed: KilometersPerHour) -> Hours {
        Hours(self.0 / (*speed))
    }
}
