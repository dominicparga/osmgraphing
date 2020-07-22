use crate::defaults::accuracy;
use kissunits::geo::Coordinate;
use smallvec::{Array, SmallVec};
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::Debug,
};

#[derive(Clone, Copy, Debug)]
pub struct Approx<T>(pub T);

impl<T> PartialOrd for Approx<&T>
where
    T: PartialOrd + Copy,
    Approx<T>: PartialOrd,
{
    fn partial_cmp(&self, other: &Approx<&T>) -> Option<Ordering> {
        Approx(*self.0).partial_cmp(&Approx(*other.0))
    }
}

impl<T> Ord for Approx<&T>
where
    T: Copy + Ord,
    Approx<T>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Approx(*self.0).cmp(&Approx(*other.0))
    }
}

impl<T> PartialEq for Approx<&T>
where
    T: Copy + PartialEq,
    Approx<T>: PartialEq,
{
    fn eq(&self, other: &Approx<&T>) -> bool {
        Approx(*self.0) == Approx(*other.0)
    }
}

impl<T> Eq for Approx<&T>
where
    T: Copy + Eq,
    Approx<T>: Eq,
{
}

impl<T> PartialOrd for Approx<Option<T>>
where
    T: Copy + PartialOrd,
    Approx<T>: PartialOrd,
{
    fn partial_cmp(&self, other: &Approx<Option<T>>) -> Option<Ordering> {
        match (self.0, other.0) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(a), Some(b)) => Approx(a).partial_cmp(&Approx(b)),
        }
    }
}

impl<T> Ord for Approx<Option<T>>
where
    T: Copy + Ord,
    Approx<T>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => Approx(a).cmp(&Approx(b)),
        }
    }
}

impl<T> PartialEq for Approx<Option<T>>
where
    T: Copy + PartialEq,
    Approx<T>: PartialEq,
{
    fn eq(&self, other: &Approx<Option<T>>) -> bool {
        match (self.0, other.0) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(a), Some(b)) => Approx(a) == Approx(b),
        }
    }
}

impl<T> Eq for Approx<Option<T>>
where
    T: Copy + Eq,
    Approx<T>: Eq,
{
}

impl<T> PartialOrd for Approx<&[T]>
where
    T: PartialOrd + Copy,
    Approx<T>: PartialOrd,
{
    fn partial_cmp(&self, other: &Approx<&[T]>) -> Option<Ordering> {
        let mut iterator = self
            .0
            .iter()
            .zip(other.0)
            .map(|(a, b)| Approx(a).partial_cmp(&Approx(b)));
        let cmp_0 = iterator.next()?;
        if iterator.any(|cmp_i| cmp_0 != cmp_i) {
            None
        } else {
            cmp_0
        }
    }
}

impl<T> PartialEq for Approx<&[T]>
where
    T: PartialEq + Copy,
    Approx<T>: PartialEq,
{
    fn eq(&self, other: &Approx<&[T]>) -> bool {
        self.0
            .iter()
            .zip(other.0)
            .fold(true, |acc, (&a, &b)| acc && Approx(a) == Approx(b))
    }
}

impl<T> Eq for Approx<&[T]>
where
    T: Eq + Copy,
    Approx<T>: Eq,
{
}

impl Approx<f64> {
    pub fn approx(&self) -> f64 {
        (self.0 / accuracy::F64_ABS).round() * accuracy::F64_ABS
    }
}

impl PartialOrd for Approx<f64> {
    fn partial_cmp(&self, other: &Approx<f64>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            // now: they are at least different by tolerance
            // -> compare normally
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Ord for Approx<f64> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect(&format!(
            "No comparison for {:?} and {:?} possible.",
            self, other
        ))
    }
}

impl PartialEq for Approx<f64> {
    fn eq(&self, other: &Approx<f64>) -> bool {
        Approx(self.0 - other.0).approx().abs() <= accuracy::F64_ABS
    }
}

impl Eq for Approx<f64> {}

impl Approx<Coordinate> {
    pub fn approx(&self) -> Coordinate {
        Coordinate {
            lat: Approx(self.0.lat).approx(),
            lon: Approx(self.0.lon).approx(),
        }
    }
}

impl PartialEq for Approx<Coordinate> {
    fn eq(&self, other: &Approx<Coordinate>) -> bool {
        Approx(self.0.lat) == Approx(other.0.lat) && Approx(self.0.lon) == Approx(other.0.lon)
    }
}

impl Eq for Approx<Coordinate> {}

impl<T, A> PartialOrd for Approx<&SmallVec<A>>
where
    T: PartialOrd + Copy,
    Approx<T>: PartialOrd,
    A: Array<Item = T>,
{
    fn partial_cmp(&self, other: &Approx<&SmallVec<A>>) -> Option<Ordering> {
        Approx(&self.0[..]).partial_cmp(&Approx(&other.0[..]))
    }
}

impl<T, A> PartialEq for Approx<&SmallVec<A>>
where
    T: PartialEq + Copy,
    Approx<T>: PartialEq,
    A: Array<Item = T>,
{
    fn eq(&self, other: &Approx<&SmallVec<A>>) -> bool {
        Approx(&self.0[..]) == Approx(&other.0[..])
    }
}

impl<T, A> Eq for Approx<&SmallVec<A>>
where
    T: PartialEq + Copy,
    Approx<T>: PartialEq,
    A: Array<Item = T>,
{
}
