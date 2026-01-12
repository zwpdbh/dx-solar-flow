use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct Edge {
    pub id: String,
    pub name: String,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
