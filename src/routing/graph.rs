use std::fmt;

//--------------------------------------------------------------------------------------------------
// definitions

pub struct GraphBuilder {
    graph: Graph,
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    offsets: Vec<usize>,
}

pub struct Node {
    id: usize,
    osm_id: Option<usize>,
    lat: f64,
    lon: f64,
}

pub struct Edge {
    id: usize,
    osm_id: Option<usize>,
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
        // offsets -> n+1 due to method `leaving_edges(...)`
        GraphBuilder {
            graph: Graph {
                nodes: Vec::with_capacity(node_capacity),
                edges: Vec::with_capacity(edge_capacity),
                offsets: Vec::with_capacity(node_capacity + 1),
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
        self.graph.offsets.reserve(additional);
        self
    }

    pub fn reserve_edges(&mut self, additional: usize) -> &mut Self {
        self.graph.edges.reserve(additional);
        self
    }

    pub fn push_node(&mut self, id: usize, osm_id: Option<usize>, lat: f64, lon: f64) -> &mut Self {
        self.graph.nodes.push(Node {
            id,
            osm_id,
            lat,
            lon,
        });
        self
    }

    pub fn push_edge(
        &mut self,
        id: usize,
        osm_id: Option<usize>,
        src: usize,
        dst: usize,
        weight: u64,
    ) -> &mut Self {
        self.graph.edges.push(Edge {
            id,
            osm_id,
            src,
            dst,
            weight,
        });
        self
    }

    pub fn finalize(mut self) -> Graph {
        //------------------------------------------------------------------------------------------
        // sort edges by ascending src, then by ascending dst

        self.graph
            .edges
            .sort_by(|e0, e1| e0.src.cmp(&e1.src).then_with(|| e0.dst.cmp(&e1.dst)));

        //------------------------------------------------------------------------------------------
        // build offset-array

        // i is node_idx, j is edge_idx
        let mut i = 0;
        let mut offset = 0;
        self.graph.offsets.push(0);
        for j in 0..self.graph.edge_count() {
            let edge = &self.graph.edges[j];
            // if coming edges have new src
            // then update offset of new src
            if i != edge.src {
                i += 1;
                self.graph.offsets.push(offset);
            }
            offset += 1;
        }
        // last node needs an upper bound as well for `leaving_edges(...)`
        self.graph.offsets.push(offset);

        self.graph
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            offsets: Vec::new(),
        }
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
        &self.nodes[id]
    }

    pub fn edge(&self, src: usize, dst: usize) -> &Edge {
        let edges = self.leaving_edges(src);
        let j = match edges.binary_search_by(|edge| edge.dst.cmp(&dst)) {
            Ok(j) => j,
            Err(_) => panic!("Edge (({})->({})) doesn't exist in the graph.", src, dst),
        };
        &self.edges[j]
    }

    pub fn leaving_edges(&self, node_id: usize) -> &[Edge] {
        let i0 = self.offsets[node_id];
        let i1 = self.offsets[node_id + 1]; // guaranteed by array-length
        &self.edges[i0..i1]
    }
}

impl Node {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn osm_id(&self) -> Option<usize> {
        self.osm_id
    }
    pub fn lat(&self) -> f64 {
        self.lat
    }
    pub fn lon(&self) -> f64 {
        self.lon
    }
}

impl Edge {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn osm_id(&self) -> Option<usize> {
        self.osm_id
    }
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

        writeln!(f, "")?;

        let n = 20;
        let m = 20;

        // print nodes
        for mut i in 0..n {
            // if enough nodes are in the graph
            if i < self.node_count() {
                if i == n - 1 {
                    // if at least 2 nodes are missing -> print `...`
                    if i + 1 < self.node_count() {
                        writeln!(f, "...")?;
                    }
                    // print last node
                    i = self.node_count() - 1;
                }
                writeln!(f, "{}", self.nodes[i])?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        // print edges
        for mut j in 0..m {
            // if enough edges are in the graph
            if j < self.edge_count() {
                if j == m - 1 {
                    // if at least 2 edges are missing -> print `...`
                    if j + 1 < self.edge_count() {
                        writeln!(f, "...")?;
                    }
                    // print last edge
                    j = self.edge_count() - 1;
                }
                writeln!(f, "{}", self.edges[j])?;
            } else {
                break;
            }
        }

        writeln!(f, "")?;

        // print offsets
        for mut i in 0..n {
            // if enough offsets are in the graph
            if i < self.node_count() {
                if i == n - 1 {
                    // if at least 2 offsets are missing -> print `...`
                    if i + 1 < self.node_count() {
                        writeln!(f, "...")?;
                    }
                    // print last offset
                    i = self.node_count() - 1;
                }
                writeln!(f, "{{ id: {}, offset: {} }}", i, self.offsets[i])?;
            } else {
                break;
            }
        }
        // offset has n+1 entries due to `leaving_edges(...)`
        let i = self.offsets.len() - 1;
        writeln!(f, "{{ __: {}, offset: {} }}", i, self.offsets[i])?;

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
