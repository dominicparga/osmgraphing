//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::{length::Meters, speed::KilometersPerHour};
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Milliseconds(pub f32);

impl Display for Milliseconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ms", self.0)
    }
}

impl Deref for Milliseconds {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl DerefMut for Milliseconds {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<Milliseconds> for Milliseconds {
    type Output = Milliseconds;

    fn add(self, other: Milliseconds) -> Milliseconds {
        Milliseconds(self.0 + other.0)
    }
}

impl AddAssign<Milliseconds> for Milliseconds {
    fn add_assign(&mut self, other: Milliseconds) {
        self.0 += other.0;
    }
}

impl Sub<Milliseconds> for Milliseconds {
    type Output = Milliseconds;

    fn sub(self, other: Milliseconds) -> Milliseconds {
        Milliseconds(self.0 - other.0)
    }
}

impl SubAssign<Milliseconds> for Milliseconds {
    fn sub_assign(&mut self, other: Milliseconds) {
        self.0 -= other.0;
    }
}

/// s = v * t
impl Mul<KilometersPerHour> for Milliseconds {
    type Output = Meters;

    fn mul(self, speed: KilometersPerHour) -> Meters {
        Meters((*speed) * self.0 / 3_600.0)
    }
}
