pub mod geo;
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
