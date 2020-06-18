use std::{
    fmt::{self, Display},
    io, result,
};

pub type Feedback = result::Result<(), Msg>;
pub type Result<T> = result::Result<T, Msg>;

#[derive(Debug)]
pub struct Msg(String);

impl Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<io::Error> for Msg {
    fn from(e: io::Error) -> Msg {
        Msg(format!("{}", e))
    }
}

impl From<String> for Msg {
    fn from(s: String) -> Msg {
        Msg(s)
    }
}

impl From<&str> for Msg {
    fn from(s: &str) -> Msg {
        Msg(s.to_owned())
    }
}
