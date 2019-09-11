mod fmi;
mod pbf;

//------------------------------------------------------------------------------------------------//

use std::ffi::OsString;

use osmgraphing::Parser;

#[test]
fn wrong_extension() {
    assert!(
        Parser::parse(&OsString::from("foo.asdf")).is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
