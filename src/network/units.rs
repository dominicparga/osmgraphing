//------------------------------------------------------------------------------------------------//
// other modules

//------------------------------------------------------------------------------------------------//
// own modules

pub use meters::Meters;
use metrics::Metric;

mod meters {
    //--------------------------------------------------------------------------------------------//
    // other modules

    use super::Metric;
    use crate::network::NodeIdx;
    use std::ops::{Index, IndexMut};

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug, Copy, Clone)]
    pub struct Meters {
        metric: Metric<u32>
    }

    impl Default for Meters {
        fn default() -> Meters {
            Meters {
                metric: Default::default()
            }
        }
    }

    impl Meters {
        pub fn new() -> Meters {
            Meters {
                ..Default::default()
            }
        }

        pub fn from<M: Into<Metric<u32>>>(value: M) -> Meters {
            Meters { metric: value.into() }
        }

        pub fn value(&self) -> u32 {
            self.metric.value()
        }

        pub fn min() -> Meters {
            Meters { metric: Metric::min() }
        }

        pub fn max() -> Meters {
            Meters { metric: Metric::max() }
        }
    }

    //--------------------------------------------------------------------------------------------//
    // indexing

    impl Index<NodeIdx> for Vec<Meters> {
        type Output = Meters;

        fn index(&self, idx: NodeIdx) -> &Self::Output {
            let idx: usize = idx.into();
            &self[idx]
        }
    }

    impl IndexMut<NodeIdx> for Vec<Meters> {
        fn index_mut(&mut self, idx: NodeIdx) -> &mut Self::Output {
            let idx: usize = idx.into();
            &mut self[idx]
        }
    }
}

/// For general functionality like adding, multiplying etc.
mod metrics {
    //--------------------------------------------------------------------------------------------//
    // other modules

    use std::cmp::{Ord, Ordering};
    use std::ops::{Add, AddAssign, Mul, MulAssign};

    //--------------------------------------------------------------------------------------------//

    #[derive(Copy, Clone, Debug)]
    pub struct Metric<T> {
        value: T,
    }

    impl<T> Default for Metric<T>
    where
        T: Default,
    {
        fn default() -> Metric<T> {
            Metric {
                value: Default::default(),
            }
        }
    }

    impl<T> Metric<T>
    where
        T: Default,
    {
        pub fn new() -> Metric<T> {
            Metric {
                ..Default::default()
            }
        }

        pub fn value(&self) -> T {
            self.value
        }
    }

    //--------------------------------------------------------------------------------------------//
    // min/max values of type T

    impl Metric<u32> {
        pub fn min() -> Metric<u32> {
            std::u32::MIN.into()
        }

        pub fn max() -> Metric<u32> {
            std::u32::MAX.into()
        }
    }

    //--------------------------------------------------------------------------------------------//
    // conversion from/to

    impl Into<f64> for Metric<u32> {
        fn into(self) -> f64 {
            self.value as f64
        }
    }

    impl From<f64> for Metric<u32> {
        fn from(value: f64) -> Self {
            Metric {
                value: value as u32,
            }
        }
    }

    impl Into<u32> for Metric<u32> {
        fn into(self) -> u32 {
            self.value
        }
    }

    /// Note that the result could have rounding errors due to up-scaling (* 1000.0) and cutting afterwards (f64 -> u32)
    impl From<u32> for Metric<u32> {
        fn from(value: u32) -> Self {
            Metric { value: value }
        }
    }

    //--------------------------------------------------------------------------------------------//
    // ordering

    impl<T> Ord for Metric<T>
    where
        T: Ord,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl<T> PartialOrd for Metric<T>
    where
        T: Ord + PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Eq for Metric<T> where T: Eq {}

    impl<T> PartialEq for Metric<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    //--------------------------------------------------------------------------------------------//
    // arithmetic operations

    impl Add<Metric<u32>> for Metric<u32> {
        type Output = Metric<u32>;

        fn add(self, other: Metric<u32>) -> Self {
            Metric {
                value: self.value + other.value,
            }
        }
    }

    impl AddAssign<Metric<u32>> for Metric<u32> {
        fn add_assign(&mut self, other: Metric<u32>) {
            self.value += other.value;
        }
    }

    impl Mul<u32> for Metric<u32> {
        type Output = Metric<u32>;

        fn mul(self, scale: u32) -> Self {
            Metric {
                value: scale * self.value,
            }
        }
    }

    impl MulAssign<u32> for Metric<u32> {
        fn mul_assign(&mut self, scale: u32) {
            self.value *= scale;
        }
    }

    impl Mul<f64> for Metric<u32> {
        type Output = Metric<u32>;

        fn mul(self, scale: f64) -> Self {
            let new_value = scale * (self.value as f64) * scale;
            Metric {
                value: new_value as u32,
            }
        }
    }

    impl MulAssign<f64> for Metric<u32> {
        fn mul_assign(&mut self, scale: f64) {
            let new_value = scale * (self.value as f64);
            self.value = new_value as u32;
        }
    }
}
