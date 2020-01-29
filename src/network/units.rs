//------------------------------------------------------------------------------------------------//
// own modules

pub use meters::Meters;
pub use quantities::Kilo;

trait Metric<T>: From<T> + Into<T> {
    fn value(&mut self) -> &mut T;
}

mod meters {
    //--------------------------------------------------------------------------------------------//
    // other modules

    use super::Metric;
    use crate::network::NodeIdx;
    use std::cmp::{Ord, Ordering};
    use std::ops::{Add, Index, IndexMut};

    //--------------------------------------------------------------------------------------------//
    // meters

    #[derive(Copy, Clone)]
    pub struct Meters {
        value: u32,
    }

    impl Default for Meters {
        fn default() -> Meters {
            Meters { value: 0 }
        }
    }

    impl Metric<u32> for Meters {
        fn value(&mut self) -> &mut u32 {
            &mut self.value
        }
    }

    impl Meters {
        pub fn new() -> Meters {
            Meters {
                ..Default::default()
            }
        }

        pub fn min() -> Meters {
            std::u32::MIN.into()
        }

        pub fn max() -> Meters {
            std::u32::MAX.into()
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }

    //--------------------------------------------------------------------------------------------//
    // conversion from/to

    impl Into<u32> for Meters {
        fn into(self) -> u32 {
            self.value
        }
    }

    impl From<u32> for Meters {
        fn from(meters: u32) -> Self {
            Meters { value: meters }
        }
    }

    //--------------------------------------------------------------------------------------------//
    // ordering

    impl Ord for Meters {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl PartialOrd for Meters {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for Meters {}

    impl PartialEq for Meters {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    //--------------------------------------------------------------------------------------------//
    // operations

    impl Add<Meters> for Meters {
        type Output = Meters;

        fn add(self, other: Meters) -> Self {
            Meters {
                value: self.value + other.value,
            }
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

mod quantities {
    use super::{Metric};

    pub struct Kilo<M> {
        metric: M,
    }

    impl<M> Kilo<M>
    where
        M: Metric<u32>,
    {
        pub fn from_u32(value: u32) -> Self {
            Kilo { metric: (1000 * value).into() }
        }

        /// Note that the result could have rounding errors due to up-scaling (* 1000.0) and cutting afterwards (f64 -> u32)
        pub fn from_f64(value: f64) -> Self {
            Kilo { metric: ((1000.0 * value) as u32).into() }
        }

        pub fn from_metric(metric: M) -> Self {
            *metric.value() /= 1000;
            Kilo { metric }
        }

        pub fn into_metric(self) -> M {
            *self.metric.value() *= 1000;
            self.metric
        }
    }
}
