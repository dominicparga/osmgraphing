use std::fmt;
use std::io;

//------------------------------------------------------------------------------------------------//

#[derive(Debug)]
pub enum Error {
    Custom(String),
    UnsuppExt(String),
    Io(io::Error),
    InvalidUnicode(String),
}

impl Error {
    pub fn from(msg: &str) -> Self {
        Error::Custom(String::from(msg))
    }

    pub fn from_unsupported_extension(ext: &str, supported: &[&str]) -> Self {
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

    pub fn from_invalid_unicode() -> Self {
        Error::InvalidUnicode(String::from("File name is invalid Unicode."))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg)         => msg.fmt(f),
            Error::UnsuppExt(msg)      => msg.fmt(f),
            Error::Io(e)               => e.fmt(f),
            Error::InvalidUnicode(msg) => msg.fmt(f),
        }
    }
}
