pub mod geo;
pub mod length;
pub mod speed;
pub mod time;
use std::fmt::Debug;

pub trait Metric: Debug + Default + Clone + Copy {
    fn zero() -> Self;

    fn from<M: Into<Self>>(value: M) -> Self;

    fn neg_inf() -> Self;

    fn inf() -> Self;
}
