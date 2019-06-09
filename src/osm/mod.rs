//------------------------------------------------------------------------------------------------//
// reader

use std::ffi::{OsStr};

trait Read {
    fn from_path<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self;
}

mod pbf;
// pub mod xml; // not finished yet

//------------------------------------------------------------------------------------------------//
// parser

pub trait Parse {
    fn parse<S: AsRef<OsStr> + ?Sized>(&self, path: &S);
}

mod parser;
pub use parser::Parser;
