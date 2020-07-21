use crate::defaults::accuracy;
use kissunits::geo::Coordinate;
use smallvec::{Array, SmallVec};
use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    fmt::Debug,
};

pub trait Approx<O> {
    fn approx(self) -> O;
}

pub trait ApproxEq<O> {
    fn approx_eq(&self, other: &O) -> bool;
}

pub trait ApproxCmp<O> {
    fn approx_partial_cmp(&self, other: &O) -> Option<Ordering>;
    fn approx_cmp(&self, other: &O) -> Ordering;
    fn approx_le(&self, other: &O) -> bool {
        let cmp = self.approx_cmp(other);
        cmp == Ordering::Less || cmp == Ordering::Equal
    }
    fn approx_ge(&self, other: &O) -> bool {
        let cmp = self.approx_cmp(other);
        cmp == Ordering::Greater || cmp == Ordering::Equal
    }
}

impl<T> ApproxCmp<T> for T
where
    T: ApproxEq<T> + PartialOrd + Debug,
{
    fn approx_partial_cmp(&self, other: &T) -> Option<Ordering> {
        match (self < other, self > other, self.approx_eq(other)) {
            (false, false, false) => None,
            (false, true, false) => Some(Greater),
            (true, false, false) => Some(Less),
            (true, true, false) | (_, _, true) => Some(Equal),
        }
    }

    fn approx_cmp(&self, other: &T) -> Ordering {
        self.approx_partial_cmp(other).expect(&format!(
            "No comparison for {:?} and {:?} possible.",
            self, other
        ))
    }
}

impl<T> Approx<Option<T>> for Option<T>
where
    T: Approx<T>,
{
    fn approx(self) -> Option<T> {
        Some(self?.approx())
    }
}

impl<T> ApproxEq<Option<T>> for Option<T>
where
    T: ApproxEq<T>,
{
    fn approx_eq(&self, other: &Option<T>) -> bool {
        match (self, other) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(left), Some(right)) => left.approx_eq(right),
        }
    }
}

impl ApproxEq<Option<f64>> for f64 {
    fn approx_eq(&self, other: &Option<f64>) -> bool {
        if let Some(other) = other {
            self.approx_eq(other)
        } else {
            false
        }
    }
}

impl ApproxEq<f64> for Option<f64> {
    fn approx_eq(&self, other: &f64) -> bool {
        if let Some(this) = self {
            this.approx_eq(other)
        } else {
            false
        }
    }
}

impl Approx<f64> for f64 {
    fn approx(self) -> f64 {
        (self / accuracy::F64_ABS).round() * accuracy::F64_ABS
    }
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(&self, other: &f64) -> bool {
        (self - other).approx().abs() <= accuracy::F64_ABS
    }
}

impl ApproxEq<Coordinate> for Coordinate {
    fn approx_eq(&self, other: &Coordinate) -> bool {
        self.lat.approx_eq(&other.lat) && self.lon.approx_eq(&other.lon)
    }
}

impl<T> Approx<Vec<T>> for Vec<T>
where
    T: Approx<T>,
{
    fn approx(self) -> Vec<T> {
        self.into_iter().map(|value| value.approx()).collect()
    }
}

impl<T, A> Approx<SmallVec<A>> for SmallVec<A>
where
    T: Approx<T>,
    A: Array<Item = T>,
{
    fn approx(self) -> SmallVec<A> {
        self.into_iter().map(|value| value.approx()).collect()
    }
}

impl<T> ApproxEq<Vec<T>> for Vec<T>
where
    T: ApproxEq<T>,
{
    fn approx_eq(&self, other: &Vec<T>) -> bool {
        self.iter()
            .zip(other)
            .fold(true, |acc, (aa, bb)| acc && aa.approx_eq(bb))
    }
}

impl<T, A> ApproxEq<SmallVec<A>> for SmallVec<A>
where
    T: ApproxEq<T>,
    A: Array<Item = T>,
{
    fn approx_eq(&self, other: &SmallVec<A>) -> bool {
        self.iter()
            .zip(other)
            .fold(true, |acc, (aa, bb)| acc && aa.approx_eq(bb))
    }
}
