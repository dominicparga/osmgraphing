use super::Metric;
use std::{
    fmt,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign},
};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct MetricU32 {
    value: u32,
}

impl Display for MetricU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Metric for MetricU32 {
    fn zero() -> MetricU32 {
        0u32.into()
    }

    fn neg_inf() -> MetricU32 {
        std::u32::MIN.into()
    }

    fn inf() -> MetricU32 {
        std::u32::MAX.into()
    }
}

impl MetricU32 {
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn is_not_zero(&self) -> bool {
        self.value != 0
    }
}

impl From<u8> for MetricU32 {
    fn from(value: u8) -> MetricU32 {
        MetricU32 {
            value: value as u32,
        }
    }
}

impl From<u16> for MetricU32 {
    fn from(value: u16) -> MetricU32 {
        MetricU32 {
            value: value as u32,
        }
    }
}

impl From<u32> for MetricU32 {
    fn from(value: u32) -> MetricU32 {
        MetricU32 { value }
    }
}

impl Deref for MetricU32 {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.value
    }
}

//--------------------------------------------------------------------------------------------//
// arithmetic operations

impl Add<MetricU32> for MetricU32 {
    type Output = MetricU32;

    fn add(self, other: MetricU32) -> MetricU32 {
        MetricU32 {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<MetricU32> for MetricU32 {
    fn add_assign(&mut self, other: MetricU32) {
        self.value += other.value;
    }
}

impl Mul<u32> for MetricU32 {
    type Output = MetricU32;

    fn mul(self, scale: u32) -> MetricU32 {
        MetricU32 {
            value: scale * self.value,
        }
    }
}

impl MulAssign<u32> for MetricU32 {
    fn mul_assign(&mut self, scale: u32) {
        self.value *= scale;
    }
}

impl Mul<f64> for MetricU32 {
    type Output = MetricU32;

    fn mul(self, scale: f64) -> MetricU32 {
        let new_value = scale * (self.value as f64) * scale;
        MetricU32 {
            value: new_value as u32,
        }
    }
}

impl MulAssign<f64> for MetricU32 {
    fn mul_assign(&mut self, scale: f64) {
        let new_value = scale * (self.value as f64);
        self.value = new_value as u32;
    }
}

impl Div<u32> for MetricU32 {
    type Output = MetricU32;

    fn div(self, rhs: u32) -> MetricU32 {
        MetricU32 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<u32> for MetricU32 {
    fn div_assign(&mut self, rhs: u32) {
        self.value /= rhs;
    }
}
