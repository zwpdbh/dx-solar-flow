use super::{Edge, Node};
use crate::{Error, Result};
use petgraph::graph::DiGraph;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    #[serde(rename = "entryGraphId")]
    pub entry_graph_id: Option<String>,
    #[serde(rename = "with")]
    pub with_params: Option<HashMap<String, serde_yaml::Value>>,
    pub graphs: Vec<Value>,
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
        let yaml_content = Self::resolve_includes(&path)?;
        let workflow_def: WorkflowDefinition = serde_yaml::from_str(&yaml_content)?;

        // Convert the workflow definition to our internal representation
        let mut graph = DiGraph::new();

        // Process each graph in the workflow definition
        for graph_value in workflow_def.graphs {
            // Convert the Value to a GraphDefinition
            if let Ok(graph_def) = serde_yaml::from_value::<GraphDefinition>(graph_value) {
                for node_def in graph_def.nodes {
                    let node = Node {
                        id: node_def.id.clone(),
                        name: node_def.name.clone(),
                        subgraph: graph_def.id.clone(), // Assign the graph ID as the subgraph
                    };
                    graph.add_node(node);
                }
            }
        }

        Ok(Workflow {
            id: workflow_def.id,
            name: workflow_def.name,
            entry_graph_id: workflow_def.entry_graph_id,
            graph,
        })
    }

    // Helper function to resolve !include directives in YAML
    fn resolve_includes(file_path: &PathBuf) -> Result<String> {
        let content = fs::read_to_string(file_path)?;
        let parent_dir = file_path.parent().unwrap_or(Path::new("."));

        // More sophisticated regex to match !include directives with their context
        // This regex captures the indentation before the !include directive
        let re = Regex::new(r"(?m)^(\s*)-\s*!\s*include\s+([^\n]+)").unwrap();

        let mut result = content.clone();

        // Process all matches from last to first to maintain correct indices
        let matches: Vec<_> = re.captures_iter(&content).collect();
        let mut substitutions = Vec::new();

        for cap in matches {
            let full_match = &cap[0]; // Full match like "  - !include ../path/to/file.yml"
            let indent = &cap[1]; // The indentation part (e.g., "  ")
            let file_path_str = cap[2].trim(); // Just the file path part

            // Resolve the included file path relative to the parent directory
            let included_path = parent_dir.join(file_path_str);

            // Recursively resolve includes in the included file
            let mut included_content = Self::resolve_includes(&included_path)?;

            // The included content needs to be properly formatted as an array element
            // If the included file starts with indentation, we need to adjust it
            let lines: Vec<&str> = included_content.lines().collect();
            let mut formatted_lines = Vec::new();

            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    // First line should have the same indentation as the original "- !include" line
                    formatted_lines.push(format!("{}{}", indent, line));
                } else {
                    // Subsequent lines should have additional indentation
                    if line.trim().is_empty() {
                        formatted_lines.push(line.to_string());
                    } else {
                        formatted_lines.push(format!("{}  {}", indent, line));
                    }
                }
            }

            let formatted_content = formatted_lines.join("\n");
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();
            substitutions.push((start, end, formatted_content));
        }

        // Apply substitutions in reverse order to maintain correct indices
        substitutions.sort_by_key(|(start, _, _)| std::cmp::Reverse(*start));
        for (start, end, replacement) in substitutions {
            result = format!("{}{}{}",
                &result[..start],
                replacement,
                &result[end..]
            );
        }

        Ok(result)
    }
}