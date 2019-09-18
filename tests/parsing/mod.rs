mod fmi;
mod pbf;

//------------------------------------------------------------------------------------------------//

use osmgraphing::Parser;

use super::parse;

//------------------------------------------------------------------------------------------------//
// tests

#[test]
fn wrong_extension() {
    assert!(
        Parser::parse("foo.asdf").is_err(),
        "File-extension 'asdf' should not be supported."
    );
}
