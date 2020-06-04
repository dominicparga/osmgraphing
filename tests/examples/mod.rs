#[allow(dead_code)]
mod dijkstra {
    include!("../../examples/dijkstra.rs");

    pub fn test() {
        match run() {
            Ok(()) => (),
            Err(msg) => panic!("{}", msg),
        }
    }
}

#[test]
fn dijkstra() {
    dijkstra::test();
}

#[allow(dead_code)]
mod parser {
    include!("../../examples/parser.rs");

    pub fn test() {
        match run() {
            Ok(()) => (),
            Err(msg) => panic!("{}", msg),
        }
    }
}

#[test]
fn parser() {
    parser::test();
}
