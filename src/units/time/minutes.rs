use crate::{
    helpers::ApproxEq,
    units::{
        length::Kilometers,
        speed::KilometersPerHour,
        time::{Hours, Seconds},
    },
};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Minutes(pub f64);

impl Minutes {
    pub fn new(min: f64) -> Minutes {
        Minutes(min)
    }
}

impl Display for Minutes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} min", self.0)
    }
}

impl From<Seconds> for Minutes {
    fn from(seconds: Seconds) -> Minutes {
        Minutes(seconds.0 / 60.0)
    }
}

impl From<&Seconds> for Minutes {
    fn from(seconds: &Seconds) -> Minutes {
        Minutes::from(*seconds)
    }
}

impl From<Hours> for Minutes {
    fn from(hours: Hours) -> Minutes {
        Minutes(hours.0 * 60.0)
    }
}

impl From<&Hours> for Minutes {
    fn from(hours: &Hours) -> Minutes {
        Minutes::from(*hours)
    }
}

impl Deref for Minutes {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Minutes {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl ApproxEq<Minutes> for Minutes {
    fn approx_eq(&self, other: &Minutes) -> bool {
        self.0.approx_eq(other)
    }
}

impl Add<Minutes> for Minutes {
    type Output = Minutes;

    fn add(self, other: Minutes) -> Minutes {
        Minutes(self.0 + other.0)
    }
}

impl AddAssign<Minutes> for Minutes {
    fn add_assign(&mut self, other: Minutes) {
        self.0 += other.0;
    }
}

impl Sub<Minutes> for Minutes {
    type Output = Minutes;

    fn sub(self, other: Minutes) -> Minutes {
        Minutes(self.0 - other.0)
    }
}

impl SubAssign<Minutes> for Minutes {
    fn sub_assign(&mut self, other: Minutes) {
        self.0 -= other.0;
    }
}

/// s = t * v
impl Mul<KilometersPerHour> for Minutes {
    type Output = Kilometers;

    fn mul(self, speed: KilometersPerHour) -> Kilometers {
        Kilometers((*speed) * (*Hours::from(self)))
    }
}
