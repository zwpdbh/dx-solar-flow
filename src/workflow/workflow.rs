use super::{Edge, Node};
use crate::{Error, Result};
use petgraph::graph::DiGraph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    #[serde(rename = "entryGraphId")]
    pub entry_graph_id: Option<String>,
    #[serde(rename = "with")]
    pub with_params: Option<HashMap<String, serde_yaml::Value>>,
    pub graphs: Vec<GraphDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphDefinition {
    pub id: String,
    pub name: String,
    pub nodes: Vec<NodeDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeDefinition {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub action: Option<String>,
    #[serde(rename = "with")]
    pub with_params: Option<HashMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub entry_graph_id: Option<String>,
    pub graph: DiGraph<Node, Edge>,
}

impl PartialEq for Workflow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.entry_graph_id == other.entry_graph_id &&
        self.graph.node_count() == other.graph.node_count() &&
        self.graph.edge_count() == other.graph.edge_count()
    }
}

impl Workflow {
    pub fn load_from_path(path: PathBuf) -> Result<Self> {
        let yaml_content = fs::read_to_string(path)?;
        let workflow_def: WorkflowDefinition = serde_yaml::from_str(&yaml_content)?;

        // Convert the workflow definition to our internal representation
        let mut graph = DiGraph::new();

        // Add all nodes from all graphs to the graph
        for graph_def in workflow_def.graphs {
            for node_def in graph_def.nodes {
                let node = Node {
                    id: node_def.id.clone(),
                    name: node_def.name.clone(),
                    subgraph: graph_def.id.clone(), // Assign the graph ID as the subgraph
                };

                graph.add_node(node);
            }
        }

        Ok(Workflow {
            id: workflow_def.id,
            name: workflow_def.name,
            entry_graph_id: workflow_def.entry_graph_id,
            graph,
        })
    }
}
