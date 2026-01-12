use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct Node {
    pub id: String,
    pub name: String,
    /// A node belongs to one subgraph, because in a graph it could contains multiple graphs as subgraph.
    pub subgraph: String,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
