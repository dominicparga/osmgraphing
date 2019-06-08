use std::ffi::{OsStr};

pub trait Read {
    fn from_str(s: &str) -> Self;
    fn from_os_str<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self;
}

pub mod pbf;
// pub mod xml; // not finished yet
