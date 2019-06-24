use std::fmt;

//--------------------------------------------------------------------------------------------------
// definitions

pub struct GraphBuilder {
    graph: Graph,
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

pub struct Node {
    id: usize,
    lat: f64,
    lon: f64,
}

pub struct Edge {
    id: usize,
    src: usize,
    dst: usize,
    weight: u64,
}

//--------------------------------------------------------------------------------------------------
// implementations

impl GraphBuilder {
    //----------------------------------------------------------------------------------------------
    // init self

    pub fn new() -> Self {
        GraphBuilder {
            graph: Graph::new(),
        }
    }

    pub fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        GraphBuilder {
            graph: Graph {
                nodes: Vec::with_capacity(node_capacity),
                edges: Vec::with_capacity(edge_capacity),
            },
        }
    }

    //----------------------------------------------------------------------------------------------
    // build graph

    pub fn new_graph(&mut self) -> &mut Self {
        self.graph = Graph::new();
        self
    }

    pub fn reserve(&mut self, additional_nodes: usize, additional_edges: usize) -> &mut Self {
        self.reserve_nodes(additional_nodes)
            .reserve_edges(additional_edges)
    }

    pub fn reserve_nodes(&mut self, additional: usize) -> &mut Self {
        self.graph.nodes.reserve(additional);
        self
    }

    pub fn reserve_edges(&mut self, additional: usize) -> &mut Self {
        self.graph.edges.reserve(additional);
        self
    }

    pub fn push_node(&mut self, id: usize, lat: f64, lon: f64) -> &mut Self {
        self.graph.nodes.push(Node { id, lat, lon });
        self
    }

    pub fn push_edge(&mut self, id: usize, src: usize, dst: usize, weight: u64) -> &mut Self {
        self.graph.edges.push(Edge {
            id,
            src,
            dst,
            weight,
        });
        self
    }

    pub fn finalize(self) -> Graph {
        // TODO compute offset array
        self.graph
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // TODO move into GraphBuilder::finalize
    pub fn set_edge_offset(&mut self) {
        // let mut i = 0;
        // let mut j = 0;
        // for edge in self.edges.iter() {
        //     if edge.src == self.nodes[j].id {
        //         self.nodes[j].edge_end = i;
        //     } else {
        //         j += 1;
        //         while j < self.node_count() && edge.src != self.nodes[j].id {
        //             self.nodes[j].edge_start = i - 1;
        //             self.nodes[j].edge_end = i - 1;
        //             j += 1;
        //         }
        //         if j < self.node_count() {
        //             self.nodes[j].edge_start = i;
        //             self.nodes[j].edge_end = i;
        //         }
        //     }
        //     i += 1;
        // }
    }

    //----------------------------------------------------------------------------------------------
    // getter

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn node(&self, id: usize) -> &Node {
        // TODO
        &self.nodes[0]
    }

    pub fn edge(&self, src: usize, dst: usize) -> &Edge {
        // TODO
        &self.edges[0]
    }

    pub fn leaving_edges(&self, node_id: usize) -> &[Edge] {
        // for edge_idx in node.edge_start..node.edge_end + 1 {
        //     let edge = &self.graph.edges[edge_idx];
        // TODO
        &self.edges[..]
    }
}

impl Node {}

impl Edge {
    pub fn src(&self) -> usize {
        self.src
    }
    pub fn dst(&self) -> usize {
        self.dst
    }
    pub fn weight(&self) -> u64 {
        self.weight
    }
}

//--------------------------------------------------------------------------------------------------
// fmt::Display

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph: {{ number of nodes: {}, number of edges: {} }}",
            self.node_count(),
            self.edge_count()
        )?;
        if self.node_count() + self.node_count() < 42 {
            writeln!(f, "")?;
            for node in &self.nodes {
                writeln!(f, "{}", node)?;
            }
            writeln!(f, "")?;
            for edge in &self.edges {
                writeln!(f, "{}", edge)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node: {{ id: {}, (lat, lon): ({:.6}, {:.6}) }}",
            self.id, self.lat, self.lon,
        )
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge: {{ id: {}, ({})-{}->({}) }}",
            self.id, self.src, self.weight, self.dst,
        )
    }
}
