use std::ffi::{OsStr};

pub trait Read {
    fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self;
}

pub mod pbf;
// pub mod xml; // not finished yet
