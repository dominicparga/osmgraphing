use crate::{
    helpers::{self, err},
    io::{self, SupportingFileExts},
};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct Parser;

impl Parser {
    pub fn new_reader<P: AsRef<Path>>(
        path: &P,
        is_file_with_header: bool,
    ) -> err::Result<impl Iterator<Item = String>> {
        let path = path.as_ref();

        io::network::edges::Parser::check_ext_support(&path)?;
        let file = OpenOptions::new()
            .read(true)
            .open(&path)
            .expect(&format!("Couldn't open {}", path.display()));
        let mut reader = BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .filter(helpers::is_line_functional);

        // consume header-line
        if is_file_with_header {
            reader.next();
        }

        Ok(reader)
    }
}

impl SupportingFileExts for Parser {
    fn supported_exts<'a>() -> &'a [&'a str] {
        &["csv"]
    }
}
