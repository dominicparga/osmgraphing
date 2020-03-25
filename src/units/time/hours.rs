use crate::{
    helpers::ApproxEq,
    units::{
        distance::Kilometers,
        speed::KilometersPerHour,
        time::{Minutes, Seconds},
    },
};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Hours(pub f64);

impl Hours {
    pub fn new(h: f64) -> Hours {
        Hours(h)
    }
}

impl Display for Hours {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} h", self.0)
    }
}

impl From<Seconds> for Hours {
    fn from(seconds: Seconds) -> Hours {
        Hours(seconds.0 / 3_600.0)
    }
}

impl From<&Seconds> for Hours {
    fn from(seconds: &Seconds) -> Hours {
        Hours::from(*seconds)
    }
}

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        Hours(minutes.0 / 60.0)
    }
}

impl From<&Minutes> for Hours {
    fn from(minutes: &Minutes) -> Hours {
        Hours::from(*minutes)
    }
}

impl Deref for Hours {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl DerefMut for Hours {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl ApproxEq<Hours> for Hours {
    fn approx_eq(&self, hours: &Hours) -> bool {
        self.0.approx_eq(hours)
    }
}

impl Add<Hours> for Hours {
    type Output = Hours;

    fn add(self, other: Hours) -> Hours {
        Hours(self.0 + other.0)
    }
}

impl AddAssign<Hours> for Hours {
    fn add_assign(&mut self, other: Hours) {
        self.0 += other.0;
    }
}

impl Sub<Hours> for Hours {
    type Output = Hours;

    fn sub(self, other: Hours) -> Hours {
        Hours(self.0 - other.0)
    }
}

impl SubAssign<Hours> for Hours {
    fn sub_assign(&mut self, other: Hours) {
        self.0 -= other.0;
    }
}

/// s = t * v
impl Mul<KilometersPerHour> for Hours {
    type Output = Kilometers;

    fn mul(self, speed: KilometersPerHour) -> Kilometers {
        Kilometers((*speed) * self.0)
    }
}
