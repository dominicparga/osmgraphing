use std::ffi::OsString;

use osmgraphing::osm;

#[test]
#[ignore]
fn parsing() {
    let path = OsString::from("resources/osm/small.pbf"); // file missing
    let parser = osm::pbf::Parser;
    let _graph = parser.parse(&path);

    // check graph structure
    unimplemented!()
}

#[test]
fn file_support() {
    assert!(
        osm::Support::from_path(&OsString::from("foo.asdf")).is_err(),
        "File-extension 'asdf' should not be supported."
    );

    let support = osm::Support::from_path(&OsString::from("foo.pbf"));
    assert!(support.is_ok(), "File-extension 'pbf' is not supported.");
    let support = support.unwrap();

    assert_ne!(support, osm::Support::FMI);
    assert_eq!(support, osm::Support::PBF);
    assert_ne!(support, osm::Support::XML);
}
