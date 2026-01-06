#![allow(unused)]
use super::node::Node;
use super::parser::{determine_format, from_str, SerdeFormat};
use super::uri::Uri;
use crate::workflow::parser::expand_yaml_includes;
use crate::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::env;
use uuid::Uuid;

pub type Id = Uuid;

pub type Parameter = Map<String, Value>;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: Id,
    pub from: Id,
    pub to: Id,
    pub from_port: String,
    pub to_port: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Graph {
    pub id: Id,
    pub name: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub entry_graph_id: Id,
    pub with: Option<Parameter>,
    pub graphs: Vec<Graph>,
}

static ENVIRONMENT_PREFIX: &str = "FLOW_VAR_";

impl TryFrom<&str> for Workflow {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut workflow: Self = from_str(value).map_err(crate::error::Error::input)?;
        workflow.load_variables_from_environment()?;
        Ok(workflow)
    }
}

impl Workflow {
    pub async fn load_from_path(workflow_path: String) -> Result<Workflow> {
        let path = Uri::for_test(&workflow_path);

        // Extract base directory for !include resolution
        let base_dir = path.path().parent().map(|p| p.to_path_buf());
        let path_buf = path.as_path();
        let yaml_content = std::fs::read_to_string(&path_buf).map_err(|e| {
            crate::error::Error::Input(format!("Failed to read workflow file: {}", e))
        })?;
        let workflow_yaml = if let Some(base) = base_dir {
            expand_yaml_includes(&yaml_content, Some(&base))?
        } else {
            expand_yaml_includes(&yaml_content, None)?
        };

        let workflow = Workflow::try_from(workflow_yaml.as_str())?;
        Ok(workflow)
    }

    fn load_variables_from_environment(&mut self) -> Result<()> {
        let environment_vars: Vec<(String, String)> = env::vars()
            .filter(|(key, _)| key.starts_with(ENVIRONMENT_PREFIX))
            .map(|(key, value)| (key[ENVIRONMENT_PREFIX.len()..].to_string(), value))
            .filter(|(key, _)| {
                self.with
                    .as_ref()
                    .unwrap_or(&serde_json::Map::new())
                    .contains_key(key)
            })
            .collect();
        if environment_vars.is_empty() {
            return Ok(());
        }
        let mut with = if let Some(with) = self.with.clone() {
            with
        } else {
            serde_json::Map::<String, Value>::new()
        };
        with.extend(
            environment_vars
                .into_iter()
                .map(|(key, value)| {
                    tracing::info!("Loading environment variable: {}", key);
                    let value = match determine_format(value.as_str()) {
                        SerdeFormat::Json | SerdeFormat::Yaml => {
                            from_str(value.as_str()).map_err(crate::error::Error::input)?
                        }
                        SerdeFormat::Unknown => {
                            serde_json::to_value(value).map_err(crate::error::Error::input)?
                        }
                    };
                    Ok((key, value))
                })
                .collect::<Result<Vec<_>>>()?,
        );
        self.with = Some(with);
        Ok(())
    }
}
