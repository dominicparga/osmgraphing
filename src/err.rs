use std::fmt;
use std::io;

use quick_xml;

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub enum Error {
    Custom(String),
    UnsuppExt(String),
    Io(io::Error),
    InvalidUnicode(String),
    XmlIo(quick_xml::Error),
}

impl Error {
    pub fn custom(msg: &str) -> Self {
        Error::Custom(String::from(msg))
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

        Error::UnsuppExt(msg)
    }

    pub fn invalid_unicode() -> Self {
        Error::InvalidUnicode(String::from("File name is invalid Unicode."))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg)         => msg.fmt(f),
            Error::UnsuppExt(msg)      => msg.fmt(f),
            Error::Io(e)               => e.fmt(f),
            Error::XmlIo(e)            => e.fmt(f),
            Error::InvalidUnicode(msg) => msg.fmt(f),
        }
    }
}

//------------------------------------------------------------------------------------------------//

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Error::XmlIo(e)
    }
}
