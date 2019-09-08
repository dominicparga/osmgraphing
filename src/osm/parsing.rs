use std::ffi::OsStr;

trait Parser {
    type Reader;

    fn open_reader<S: AsRef<OsStr> + ?Sized>(path: &S) -> Self::Reader;
}
