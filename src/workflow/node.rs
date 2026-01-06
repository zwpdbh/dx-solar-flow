#![allow(unused)]
use super::workflow::Id;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt;

pub static ROUTING_PARAM_KEY: &str = "routingPort";
pub static INPUT_ROUTING_ACTION: &str = "InputRouter";
pub static OUTPUT_ROUTING_ACTION: &str = "OutputRouter";

pub type NodeProperty = Map<String, Value>;
pub type NodeAction = String;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct NodeEntity {
    pub id: Id,
    pub name: String,
    pub with: Option<NodeProperty>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(String);

impl Port {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl NodeId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = &self.0;
        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeHandle {
    pub id: NodeId,
}
impl NodeHandle {
    pub fn new(id: NodeId) -> Self {
        Self { id }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "action")]
    Action {
        #[serde(flatten)]
        entity: NodeEntity,
        action: NodeAction,
    },
    #[serde(rename = "subGraph")]
    SubGraph {
        #[serde(flatten)]
        entity: NodeEntity,
        #[serde(rename = "subGraphId")]
        sub_graph_id: Id,
    },
}

impl Node {
    pub fn id(&self) -> Id {
        match self {
            Node::Action { entity, action: _ } => entity.id,
            Node::SubGraph {
                entity,
                sub_graph_id: _,
            } => entity.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Node::Action { entity, action: _ } => &entity.name,
            Node::SubGraph {
                entity,
                sub_graph_id: _,
            } => &entity.name,
        }
    }

    pub fn action(&self) -> &str {
        match self {
            Node::Action { entity: _, action } => action.as_str(),
            Node::SubGraph {
                entity: _,
                sub_graph_id: _,
            } => "subGraph",
        }
    }

    pub fn with(&self) -> &Option<NodeProperty> {
        match self {
            Node::Action { entity, action: _ } => &entity.with,
            Node::SubGraph {
                entity,
                sub_graph_id: _,
            } => &entity.with,
        }
    }
}
