#![allow(unused)]
//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

pub mod flow;
pub use flow::Flow;

mod graph;
pub use graph::Graph;

mod node;
pub use node::Node;

mod edge;
pub use edge::Edge;
