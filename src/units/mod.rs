pub mod geo;
use std::cmp::Ordering;
pub mod length;
pub mod speed;
pub mod time;
use std::{
    fmt,
    fmt::{Debug, Display},
};

pub trait Metric: Clone + Copy + Debug + Default + Display {
    fn new<M: Into<Self>>(value: M) -> Self;

    fn zero() -> Self;

    fn neg_inf() -> Self;

    fn inf() -> Self;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Metric2D<M0, M1>
where
    M0: Metric,
    M1: Metric,
{
    m0: M0,
    m1: M1,
}

impl<M0, M1> Display for Metric2D<M0, M1>
where
    M0: Metric,
    M1: Metric,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.m0, self.m1)
    }
}

impl<M0, M1> Metric for Metric2D<M0, M1>
where
    M0: Metric,
    M1: Metric,
{
    fn new<M: Into<Self>>(value: M) -> Self {
        value.into()
    }

    fn zero() -> Self {
        Metric2D {
            m0: M0::zero(),
            m1: M1::zero(),
        }
    }

    fn neg_inf() -> Self {
        Metric2D {
            m0: M0::zero(),
            m1: M1::zero(),
        }
    }

    fn inf() -> Self {
        Metric2D {
            m0: M0::inf(),
            m1: M1::inf(),
        }
    }
}

impl<M0, M1> Ord for Metric2D<M0, M1>
where
    M0: Metric + Ord,
    M1: Metric + Ord,
{
    fn cmp(&self, other: &Metric2D<M0, M1>) -> Ordering {
        let first = match self.m0.cmp(&other.m0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let second = match self.m1.cmp(&other.m1) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        // equal means pareto-equal
        (first + second).cmp(&0)
    }
}

impl<M0, M1> PartialOrd for Metric2D<M0, M1>
where
    M0: Metric + PartialOrd,
    M1: Metric + PartialOrd,
{
    fn partial_cmp(&self, other: &Metric2D<M0, M1>) -> Option<Ordering> {
        let first = match self.m0.partial_cmp(&other.m0)? {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let second = match self.m1.partial_cmp(&other.m1)? {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        // equal means pareto-equal
        Some((first + second).cmp(&0))
    }
}

impl<M0, M1> Eq for Metric2D<M0, M1>
where
    M0: Metric + Eq,
    M1: Metric + Eq,
{
}

impl<M0, M1> PartialEq for Metric2D<M0, M1>
where
    M0: Metric + PartialEq,
    M1: Metric + PartialEq,
{
    fn eq(&self, other: &Metric2D<M0, M1>) -> bool {
        self.m0 == other.m0 && self.m1 == other.m1
    }
}
