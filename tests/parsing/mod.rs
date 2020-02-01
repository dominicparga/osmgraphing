use super::parse;
use osmgraphing::Parser;

//------------------------------------------------------------------------------------------------//

mod fmi;
mod pbf;

//------------------------------------------------------------------------------------------------//
// tests

#[test]
fn wrong_extension() {
    assert!(
        Parser::parse("foo.asdf").is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
