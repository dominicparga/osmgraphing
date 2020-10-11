#[test]
fn dijkstra() {
    #[allow(dead_code)]
    mod example {
        include!("../../examples/dijkstra.rs");

        pub fn test() {
            match run() {
                Ok(()) => (),
                Err(msg) => panic!("{}", msg),
            }
        }
    }

    example::test();
}

#[test]
fn parser() {
    #[allow(dead_code)]
    mod example {
        include!("../../examples/parser.rs");

        pub fn test() {
            match run() {
                Ok(()) => (),
                Err(msg) => panic!("{}", msg),
            }
        }
    }

    example::test();
}

#[cfg(feature = "gpl")]
#[test]
fn exploration() {
    #[allow(dead_code)]
    mod example {
        include!("../../examples/exploration.rs");

        pub fn test() {
            match run() {
                Ok(()) => (),
                Err(msg) => panic!("{}", msg),
            }
        }
    }

    example::test();
}
