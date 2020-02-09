pub mod geo;
pub mod length;
pub mod speed;
pub mod time;
use std::fmt::{Debug, Display};

pub trait Metric: Debug + Default + Display + Clone + Copy {
    fn new<M: Into<Self>>(value: M) -> Self;

    fn zero() -> Self;

    fn neg_inf() -> Self;

    fn inf() -> Self;
}
