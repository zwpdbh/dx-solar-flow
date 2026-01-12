use super::{Edge, Node};
use crate::{Error, Result};
use petgraph::graph::DiGraph;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub entry_graph_id: Option<String>,
    pub graph: DiGraph<Node, Edge>,
}

impl Workflow {
    pub fn load_from_path(path: PathBuf) -> Result<Self> {
        todo!()
    }
}
