#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    /// A node belongs to one subgraph, because in a graph it could contains multiple graphs as subgraph.
    pub subgraph: String,
}
