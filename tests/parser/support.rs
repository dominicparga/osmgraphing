use std::ffi::OsString;

use osmgraphing::osm;

#[test]
fn fmi() {
    let path = OsString::from("foo.fmi");

    let support = match osm::Support::from_path(&path) {
        Ok(filetype) => filetype,
        _ => panic!(),
    };

    assert_eq!(support, osm::Support::FMI);
    assert_ne!(support, osm::Support::PBF);
    assert_ne!(support, osm::Support::XML);
}

#[test]
fn pbf() {
    let path = OsString::from("foo.pbf");

    let support = match osm::Support::from_path(&path) {
        Ok(filetype) => filetype,
        _ => panic!(),
    };

    assert_ne!(support, osm::Support::FMI);
    assert_eq!(support, osm::Support::PBF);
    assert_ne!(support, osm::Support::XML);
}

#[test]
fn osm() {
    let path = OsString::from("foo.osm");

    let support = match osm::Support::from_path(&path) {
        Ok(filetype) => filetype,
        _ => panic!(),
    };

    assert_ne!(support, osm::Support::FMI);
    assert_ne!(support, osm::Support::PBF);
    assert_eq!(support, osm::Support::XML);
}

#[test]
#[should_panic]
fn any() {
    let path = OsString::from("foo.asdf");

    let _support = match osm::Support::from_path(&path) {
        Ok(filetype) => filetype,
        _ => panic!(),
    };
}
