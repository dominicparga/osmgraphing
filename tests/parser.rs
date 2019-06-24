mod parser {
    use osmgraphing::osm;
    use std::ffi::{OsString, OsStr};

    fn assert_support<S>(path: &S, expected: osm::Support) where S: AsRef<OsStr> + ?Sized {
        let path = OsString::from(&path);

        let support = match osm::Support::from_path(&path) {
            Ok(expected) => expected,
            _ => panic!(),
        };

        assert_eq!(support, expected);
    }

    mod support {
        use osmgraphing::osm;
        use super::assert_support;

        #[test]
        fn fmi() {
            assert_support("foo.fmi", osm::Support::FMI);
        }

        #[test]
        fn pbf() {
            assert_support("foo.pbf", osm::Support::PBF);
        }

        #[test]
        fn osm() {
            assert_support("foo.osm", osm::Support::XML);
        }
    }

    mod unsupport {
        use osmgraphing::osm;
        use super::assert_support;

        #[test]
        #[should_panic]
        fn any() {
            let filename = "foo.";
            assert_support(filename, osm::Support::FMI);
            assert_support(filename, osm::Support::PBF);
            assert_support(filename, osm::Support::XML);
        }
    }
}

// fn parse_fmi() { //     let path = OsString::from("resources/osm/small.fmi");

//     let graph = match osm::Support::from_path(&path) {
//         Ok(osm::Support::PBF) => {
//             let parser = osm::pbf::Parser;
//             parser.parse(&path)
//         }
//         Ok(osm::Support::FMI) => {
//             let parser = osm::fmi::Parser;
//             parser.parse(&path)
//         }
//         Ok(osm::Support::XML) => unimplemented!(),
//         Err(e) => panic!("{:}", e),
//     };

//     println!("{}", graph);
// }
