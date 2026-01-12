#![allow(unused)]
use crate::components::graph::Point;
use crate::components::{Edge, Node};
use crate::workflow::Workflow;
use dioxus::prelude::*;

#[component]
pub fn Flow(mut workflow: Signal<Workflow>) -> Element {
    rsx! {}
}
