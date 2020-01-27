mod refcells {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    pub struct EdgeContainer {
        graph: RefCell<Weak<Graph>>,
    }

    impl EdgeContainer {
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

    #[derive(Debug)]
    pub struct Graph {
        value: u32,
        fwd_edges: RefCell<Rc<EdgeContainer>>,
    }

    impl Graph {
        pub fn fwd_edges(&self) -> Rc<EdgeContainer> {
            Rc::clone(&self.fwd_edges.borrow())
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }

    impl Graph {
        pub fn new() -> Rc<Graph> {
            // init fwd-edges with empty ref to graph
            let fwd_edges = Rc::new(EdgeContainer {
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

mod moves {
    #[derive(Debug)]
    pub struct Edge {
        treasure: String,
    }

    impl Edge {
        pub fn treasure(&self) -> &str {
            &self.treasure
        }
    }

    #[derive(Debug)]
    pub struct EdgeContainer {
        graph: Graph,
    }

    impl EdgeContainer {
        fn from(graph: Graph) -> EdgeContainer {
            EdgeContainer { graph }
        }

        pub fn graph(self) -> Graph {
            self.graph
        }

        pub fn edge_treasure(&self) -> &str {
            let edge_idx = 0;
            self.graph.edges[edge_idx].treasure()
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        edges: Vec<Edge>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                edges: vec![Edge {
                    treasure: String::from("Access this from EdgeContainer hehe"),
                }],
            }
        }

        pub fn fwd_edges(self) -> EdgeContainer {
            EdgeContainer::from(self)
        }
    }
}

fn main() {
    // using references
    {
        println!("Access graph's edges using RefCell");
        let graph = refcells::Graph::new();
        let fwd_edges = graph.fwd_edges();
        drop(graph);
        fwd_edges.try_using_graph();
    }

    println!();

    // using moves
    {
        println!("Access graph's edges using move-semantic");
        let graph = moves::Graph::new();
        let fwd_edges = graph.fwd_edges();
        println!("Treasure found: {}", fwd_edges.edge_treasure());
        // drop(graph); // doesn't compile due to previous move :)
        let _graph = fwd_edges.graph();
    }
}
