use super::Metric;
use std::{
    fmt,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign},
};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct MetricU32(pub u32);

impl Display for MetricU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Metric for MetricU32 {
    fn zero() -> MetricU32 {
        MetricU32(0)
    }

    fn neg_inf() -> MetricU32 {
        MetricU32(std::u32::MIN)
    }

    fn inf() -> MetricU32 {
        MetricU32(std::u32::MAX)
    }
}

impl MetricU32 {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_not_zero(&self) -> bool {
        self.0 != 0
    }
}

impl From<u8> for MetricU32 {
    fn from(value: u8) -> MetricU32 {
        MetricU32(value as u32)
    }
}

impl From<u16> for MetricU32 {
    fn from(value: u16) -> MetricU32 {
        MetricU32(value as u32)
    }
}

impl From<u32> for MetricU32 {
    fn from(value: u32) -> MetricU32 {
        MetricU32(value)
    }
}

impl Deref for MetricU32 {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl DerefMut for MetricU32 {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<MetricU32> for MetricU32 {
    type Output = MetricU32;

    fn add(self, other: MetricU32) -> MetricU32 {
        MetricU32(self.0 + other.0)
    }
}

impl AddAssign<MetricU32> for MetricU32 {
    fn add_assign(&mut self, other: MetricU32) {
        self.0 += other.0;
    }
}

impl Mul<u32> for MetricU32 {
    type Output = MetricU32;

    fn mul(self, scale: u32) -> MetricU32 {
        MetricU32(self.0 * scale)
    }
}

impl MulAssign<u32> for MetricU32 {
    fn mul_assign(&mut self, scale: u32) {
        self.0 *= scale;
    }
}

impl Mul<f64> for MetricU32 {
    type Output = MetricU32;

    fn mul(self, scale: f64) -> MetricU32 {
        let new_value = scale * (self.0 as f64) * scale;
        MetricU32(new_value as u32)
    }
}

impl MulAssign<f64> for MetricU32 {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.0 as f64);
        self.0 = new_value as u32;
    }
}

impl Div<u32> for MetricU32 {
    type Output = MetricU32;

    fn div(self, rhs: u32) -> MetricU32 {
        MetricU32(self.0 / rhs)
    }
}

impl DivAssign<u32> for MetricU32 {
    fn div_assign(&mut self, rhs: u32) {
        self.0 /= rhs;
    }
}
