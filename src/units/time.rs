//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//
// other modules

use super::{length::Meters, speed::KilometersPerHour, Metric};
use crate::network::NodeIdx;
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign},
};

//------------------------------------------------------------------------------------------------//

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Milliseconds {
    value: u32,
}

impl Display for Milliseconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ms", self.value)
    }
}

impl Metric for Milliseconds {
    fn new<M: Into<Milliseconds>>(meters: M) -> Milliseconds {
        meters.into()
    }

    fn zero() -> Milliseconds {
        Milliseconds { value: 0 }
    }

    fn neg_inf() -> Milliseconds {
        Milliseconds {
            value: std::u32::MIN,
        }
    }

    fn inf() -> Milliseconds {
        Milliseconds {
            value: std::u32::MAX,
        }
    }
}

impl Milliseconds {
    pub fn value(&self) -> u32 {
        self.value
    }
}

//--------------------------------------------------------------------------------------------//
// indexing

impl Index<NodeIdx> for Vec<Milliseconds> {
    type Output = Milliseconds;

    fn index(&self, idx: NodeIdx) -> &Self::Output {
        let idx: usize = idx.to_usize();
        &self[idx]
    }
}

impl IndexMut<NodeIdx> for Vec<Milliseconds> {
    fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
        let idx: usize = idx.to_usize();
        &mut self[idx]
    }
}

//--------------------------------------------------------------------------------------------//
// conversion from/to

impl Into<f64> for Milliseconds {
    fn into(self) -> f64 {
        self.value as f64
    }
}

impl From<f64> for Milliseconds {
    fn from(value: f64) -> Self {
        Milliseconds {
            value: value as u32,
        }
    }
}

impl Into<u32> for Milliseconds {
    fn into(self) -> u32 {
        self.value
    }
}

/// Note that the result could have rounding errors due to up-scaling (* 1000.0) and cutting afterwards (f64 -> u32)
impl From<u32> for Milliseconds {
    fn from(value: u32) -> Self {
        Milliseconds { value: value }
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<Milliseconds> for Milliseconds {
    type Output = Milliseconds;

    fn add(self, other: Milliseconds) -> Self {
        Milliseconds {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<Milliseconds> for Milliseconds {
    fn add_assign(&mut self, other: Milliseconds) {
        self.value += other.value;
    }
}

impl Mul<u32> for Milliseconds {
    type Output = Milliseconds;

    fn mul(self, scale: u32) -> Self {
        Milliseconds {
            value: scale * self.value,
        }
    }
}

impl MulAssign<u32> for Milliseconds {
    fn mul_assign(&mut self, scale: u32) {
        self.value *= scale;
    }
}

impl Mul<f64> for Milliseconds {
    type Output = Milliseconds;

    fn mul(self, scale: f64) -> Self {
        let new_value = scale * (self.value as f64) * scale;
        Milliseconds {
            value: new_value as u32,
        }
    }
}

impl MulAssign<f64> for Milliseconds {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.value as f64);
        self.value = new_value as u32;
    }
}

/// s = v * t
impl Mul<KilometersPerHour> for Milliseconds {
    type Output = Meters;

    fn mul(self, rhs: KilometersPerHour) -> Meters {
        let time = self.value;
        let speed = rhs.value() as u32;
        (speed * time / 3_600).into()
    }
}
