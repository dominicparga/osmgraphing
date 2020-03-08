mod refcells {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug)]
    pub struct EdgeAccessor {
        graph: RefCell<Weak<Graph>>,
    }

    impl EdgeAccessor {
        fn graph(&self) -> Weak<Graph> {
            Weak::clone(&self.graph.borrow())
        }

        pub fn try_using_graph(&self) {
            // Unwrapping is bad here because
            // if graph has already been dropped -> exception
            match self.graph().upgrade() {
                Some(graph) => println!("Using graph-value: {}", graph.value()),
                None => println!("nullpointer gg ez"),
            };
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug)]
    pub struct Graph {
        value: u32,
        fwd_edges: RefCell<Rc<EdgeAccessor>>,
    }

    impl Graph {
        pub fn fwd_edges(&self) -> Rc<EdgeAccessor> {
            Rc::clone(&self.fwd_edges.borrow())
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }

    impl Graph {
        pub fn new() -> Rc<Graph> {
            // init fwd-edges with empty ref to graph
            let fwd_edges = Rc::new(EdgeAccessor {
                graph: RefCell::new(Weak::new()),
            });
            // init graph with ref to fwd-edges
            let graph = Rc::new(Graph {
                value: 42,
                fwd_edges: RefCell::new(Rc::clone(&fwd_edges)),
            });
            // update fwd-edges' empty ref to graph
            *fwd_edges.graph.borrow_mut() = Rc::downgrade(&graph);
            // return
            graph
        }
    }
}

mod moving {
    #[derive(Debug)]
    pub struct Edge {
        treasure: String,
    }

    impl Edge {
        pub fn treasure(&self) -> &str {
            &self.treasure
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug)]
    pub struct EdgeAccessor {
        graph: Graph,
    }

    impl EdgeAccessor {
        fn from(graph: Graph) -> EdgeAccessor {
            EdgeAccessor { graph }
        }

        pub fn graph(self) -> Graph {
            self.graph
        }

        pub fn edge_treasure(&self) -> &str {
            let edge_idx = 0;
            self.graph.edges[edge_idx].treasure()
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug)]
    pub struct Graph {
        edges: Vec<Edge>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                edges: vec![Edge {
                    treasure: String::from("Access this from EdgeAccessor hehe"),
                }],
            }
        }

        pub fn fwd_edges(self) -> EdgeAccessor {
            EdgeAccessor::from(self)
        }
    }
}

mod borrowing {
    #[derive(Debug)]
    pub struct Edge {
        treasure: Option<String>,
    }

    impl Edge {
        pub fn treasure(&self) -> String {
            match &self.treasure {
                Some(treasure) => String::from(treasure),
                None => String::from("Nope, no treasure >.<"),
            }
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug)]
    pub struct EdgeAccessor<'a> {
        graph: &'a Graph,
        // general name since information is stored in memory, not in code/algorithm/...
        edges: &'a Vec<usize>,
        offsets: &'a Vec<usize>,
    }

    impl<'a> EdgeAccessor<'a> {
        fn from(
            graph: &'a Graph,
            edges: &'a Vec<usize>,
            offsets: &'a Vec<usize>,
        ) -> EdgeAccessor<'a> {
            EdgeAccessor {
                graph,
                edges,
                offsets,
            }
        }

        pub fn edge_treasure(&self) -> String {
            let edge_idx = 0;
            self.graph.edges[edge_idx].treasure()
        }
    }

    //--------------------------------------------------------------------------------------------//

    #[derive(Debug)]
    pub struct Graph {
        edges: Vec<Edge>,
        // important: store everything in here, but provide pointers to the EdgeAccessor
        fwd_edges: Vec<usize>,
        fwd_offsets: Vec<usize>,
        bwd_edges: Vec<usize>,
        bwd_offsets: Vec<usize>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                edges: vec![
                    Edge {
                        treasure: Some(String::from("Access this from EdgeAccessor hehe")),
                    },
                    Edge { treasure: None },
                ],
                fwd_edges: vec![0, 1],
                fwd_offsets: vec![0, 1],
                bwd_edges: vec![1, 0],
                bwd_offsets: vec![0, 1],
            }
        }

        pub fn fwd_edges<'a>(&'a self) -> EdgeAccessor<'a> {
            EdgeAccessor::from(self, &(self.fwd_edges), &(self.fwd_offsets))
        }

        pub fn bwd_edges<'a>(&'a self) -> EdgeAccessor<'a> {
            EdgeAccessor::from(self, &(self.bwd_edges), &(self.bwd_offsets))
        }
    }
}

//------------------------------------------------------------------------------------------------//

fn using_refcells() {
    println!("Access graph's edges using RefCell");
    let graph = refcells::Graph::new();
    let fwd_edges = graph.fwd_edges();
    drop(graph);
    fwd_edges.try_using_graph();
}

fn using_moving() {
    println!("Access graph's edges using move-semantic");
    let graph = moving::Graph::new();
    let fwd_edges = graph.fwd_edges();
    println!("Treasure found: {}", fwd_edges.edge_treasure());
    // drop(graph); // doesn't compile due to previous move
    let graph_after = fwd_edges.graph();
    drop(graph_after);
}

fn using_borrowing() {
    println!("Access graph's edges using borrow-semantic");
    let graph = borrowing::Graph::new();
    let fwd_edges = graph.fwd_edges();
    let bwd_edges = graph.bwd_edges();
    println!("Forward-treasure found: {}", fwd_edges.edge_treasure());
    println!("Backward-treasure found: {}", bwd_edges.edge_treasure());
    drop(graph);
    // doesn't compile due to previous move in drop(...)
    // println!("Forward-treasure found: {}", fwd_edges.edge_treasure());
    // println!("Backward-treasure found: {}", bwd_edges.edge_treasure());
}

fn main() {
    // test different access-methods
    using_refcells();
    println!();
    using_moving();
    println!();
    using_borrowing();
}
