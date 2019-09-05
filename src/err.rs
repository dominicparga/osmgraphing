use std::fmt;
use std::io;

use quick_xml;

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub enum Error {
    Custom(String),
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error::Custom(String::from(msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg) => msg.fmt(f),
        }
    }
}

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub enum FileError {
    Custom(Error),
    UnsuppExt(String),
    Io(io::Error),
    InvalidUnicode(String),
    XmlIo(quick_xml::Error),
}

impl FileError {
    pub fn new(msg: &str) -> Self {
        FileError::Custom(Error::new(msg))
    }

    pub fn unsupported_extension(ext: &str, supported: &[&str]) -> Self {
        let mut msg = {
            if ext.is_empty() {
                String::from("The file has no extension.")
            } else {
                format!("Unsupported extension '{}' was given.", ext)
            }
        };

        msg = format!("{} Please use a valid path to the osm-file.", msg);
        msg = format!("{}\nSupported extensions are: {:?}", msg, supported);

        FileError::UnsuppExt(msg)
    }

    pub fn invalid_unicode() -> Self {
        FileError::InvalidUnicode(String::from("File name is invalid Unicode."))
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::Custom(e) => e.fmt(f),
            FileError::UnsuppExt(msg) => msg.fmt(f),
            FileError::Io(e) => e.fmt(f),
            FileError::XmlIo(e) => e.fmt(f),
            FileError::InvalidUnicode(msg) => msg.fmt(f),
        }
    }
}

impl From<io::Error> for FileError {
    fn from(e: io::Error) -> Self {
        FileError::Io(e)
    }
}

impl From<quick_xml::Error> for FileError {
    fn from(e: quick_xml::Error) -> Self {
        FileError::XmlIo(e)
    }
}
