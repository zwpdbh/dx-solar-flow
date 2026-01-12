use crate::components::graph::{EditingMode, Point, Selection, Tab};
use crate::components::{Edge as GraphEdge, Node as GraphNode};
use crate::workflow::{Edge, Node, Workflow};
use dioxus::prelude::*;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

#[component]
pub fn Flow(mut workflow: Signal<Workflow>) -> Element {
    // Store node positions in a signal for dragging, using default layout
    let mut node_positions = use_signal(move || {
        let workflow_ref = workflow.read();
        let node_count = workflow_ref.graph.node_count();
        let mut positions = HashMap::new();

        if node_count > 0 {
            let radius = 150.0;
            let center_x = 300.0;
            let center_y = 200.0;

            for (i, node_idx) in workflow_ref.graph.node_indices().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / node_count as f64;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();

                positions.insert(node_idx, Point { x, y });
            }
        }

        positions
    });

    // Track which node is currently being dragged
    let mut dragging_node = use_signal(|| None::<NodeIndex>);

    // Track the current editing mode
    let mut editing_mode = use_signal(|| EditingMode::Normal);

    // Track selected nodes for edge creation
    let mut selected_nodes = use_signal(|| Vec::<NodeIndex>::new());

    // Track current selection (for properties panel)
    let mut current_selection = use_signal(|| Selection::None);

    // Track current active tab
    let mut active_tab = use_signal(|| Tab::Node);

    let handle_mousemove = move |event: MouseEvent| {
        if let Some(node_idx) = *dragging_node.read() {
            let rect = event.data().element_coordinates();
            let x = rect.x as f64;
            let y = rect.y as f64;

            // Update the position of the dragged node
            node_positions.write().insert(node_idx, Point { x, y });
        }
    };

    let handle_mouseup = move |_| {
        *dragging_node.write() = None;
    };

    let handle_drag_start = move |node_idx: NodeIndex| {
        *dragging_node.write() = Some(node_idx);
    };

    let handle_node_click = move |node_idx: NodeIndex| {
        match *editing_mode.read() {
            EditingMode::Normal => {
                // Select the node for properties panel
                *current_selection.write() = Selection::Node(node_idx);
            }
            EditingMode::AddEdge => {
                // Add node to selection for edge creation
                let mut nodes = selected_nodes.write();
                if !nodes.contains(&node_idx) {
                    nodes.push(node_idx);
                }

                // If we have two nodes selected, create an edge
                if nodes.len() == 2 {
                    let source = nodes[0];
                    let target = nodes[1];

                    // In a real implementation, you would add an edge to the workflow
                    // For now, we'll just clear the selection
                    nodes.clear();
                }
            }
            EditingMode::DeleteEdge => {
                // In delete mode, clicking a node doesn't do anything
                // Edges are deleted by clicking on them directly
            }
            EditingMode::AddNode => {
                // In add node mode, clicking doesn't do anything
            }
            EditingMode::DeleteNode => {
                // In a real implementation, you would remove the node from the workflow
                // For now, we'll just clear selection
                *current_selection.write() = Selection::None;
            }
        }
    };

    let handle_canvas_click = move |event: MouseEvent| {
        if *editing_mode.read() == EditingMode::AddNode {
            // In a real implementation, you would add a new node to the workflow
        }
    };

    let handle_edge_click = move |_edge_idx| {
        match *editing_mode.read() {
            EditingMode::Normal => {
                // In a real implementation, you would select the edge for properties panel
            }
            EditingMode::AddEdge => {
                // Do nothing in add edge mode
            }
            EditingMode::DeleteEdge => {
                // In a real implementation, you would remove the edge from the workflow
            }
            EditingMode::AddNode => {
                // In add node mode, clicking doesn't do anything
            }
            EditingMode::DeleteNode => {
                // In delete node mode, clicking an edge doesn't do anything
            }
        }
    };

    let set_normal_mode = move |_| {
        *editing_mode.write() = EditingMode::Normal;
        selected_nodes.write().clear();
    };

    let set_add_edge_mode = move |_| {
        *editing_mode.write() = EditingMode::AddEdge;
        selected_nodes.write().clear();
    };

    let set_delete_edge_mode = move |_| {
        *editing_mode.write() = EditingMode::DeleteEdge;
        selected_nodes.write().clear();
    };

    let set_add_node_mode = move |_| {
        *editing_mode.write() = EditingMode::AddNode;
    };

    let set_delete_node_mode = move |_| {
        *editing_mode.write() = EditingMode::DeleteNode;
    };

    let switch_to_node_tab = move |_| {
        *active_tab.write() = Tab::Node;
        *editing_mode.write() = EditingMode::Normal;
    };

    let switch_to_edge_tab = move |_| {
        *active_tab.write() = Tab::Edge;
        *editing_mode.write() = EditingMode::Normal;
    };

    // Get the current selection info for display
    let selection_info = match &*current_selection.read() {
        Selection::Node(node_idx) => {
            let workflow_ref = workflow.read();
            if let Some(node_data) = workflow_ref.graph.node_weight(*node_idx) {
                format!("Selected Node: {} ({})", node_data.name, node_data.id)
            } else {
                "Selected Node: (unknown)".to_string()
            }
        },
        Selection::Edge(edge_idx) => {
            let workflow_ref = workflow.read();
            if let Some(edge_data) = workflow_ref.graph.edge_weight(*edge_idx) {
                format!("Selected Edge: {} ({})", edge_data.name, edge_data.id)
            } else {
                "Selected Edge: (unknown)".to_string()
            }
        },
        Selection::None => "No selection".to_string(),
    };

    rsx! {
        div { class: "flex flex-col h-screen",
            div { class: "p-4 bg-gray-100",
                h2 { class: "text-xl font-bold", "Workflow Visualization: {workflow.read().name}" }
                div { class: "mt-2 text-sm text-gray-600",
                    "Visualizing workflow: {workflow.read().id}. Drag nodes to reposition them."
                }

                // Tab navigation
                div { class: "flex border-b border-gray-200 mb-4",
                    {
                        let tab_class = if *active_tab.read() == Tab::Node {
                            "py-2 px-4 font-medium text-sm text-blue-600 border-b-2 border-blue-600"
                        } else {
                            "py-2 px-4 font-medium text-sm text-gray-500 hover:text-gray-700"
                        };
                        rsx! {
                            button {
                                class: "{tab_class}",
                                onclick: switch_to_node_tab,
                                "Nodes"
                            }
                        }
                    }
                    {
                        let tab_class = if *active_tab.read() == Tab::Edge {
                            "py-2 px-4 font-medium text-sm text-blue-600 border-b-2 border-blue-600"
                        } else {
                            "py-2 px-4 font-medium text-sm text-gray-500 hover:text-gray-700"
                        };
                        rsx! {
                            button {
                                class: "{tab_class}",
                                onclick: switch_to_edge_tab,
                                "Edges"
                            }
                        }
                    }
                }

                // Tab content
                if *active_tab.read() == Tab::Node {
                    // Node operations
                    div { class: "flex space-x-2 mt-2",
                        {
                            let btn_class = if *editing_mode.read() == EditingMode::Normal {
                                "px-3 py-1 rounded text-sm bg-blue-500 text-white"
                            } else {
                                "px-3 py-1 rounded text-sm bg-gray-200"
                            };
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    onclick: set_normal_mode,
                                    "Normal"
                                }
                            }
                        }
                        {
                            let btn_class = if *editing_mode.read() == EditingMode::AddNode {
                                "px-3 py-1 rounded text-sm bg-green-500 text-white"
                            } else {
                                "px-3 py-1 rounded text-sm bg-gray-200"
                            };
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    onclick: set_add_node_mode,
                                    "Add Node"
                                }
                            }
                        }
                        {
                            let btn_class = if *editing_mode.read() == EditingMode::DeleteNode {
                                "px-3 py-1 rounded text-sm bg-red-500 text-white"
                            } else {
                                "px-3 py-1 rounded text-sm bg-gray-200"
                            };
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    onclick: set_delete_node_mode,
                                    "Delete Node"
                                }
                            }
                        }
                    }
                } else {
                    // Edge operations
                    div { class: "flex space-x-2 mt-2",
                        {
                            let btn_class = if *editing_mode.read() == EditingMode::Normal {
                                "px-3 py-1 rounded text-sm bg-blue-500 text-white"
                            } else {
                                "px-3 py-1 rounded text-sm bg-gray-200"
                            };
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    onclick: set_normal_mode,
                                    "Normal"
                                }
                            }
                        }
                        {
                            let btn_class = if *editing_mode.read() == EditingMode::AddEdge {
                                "px-3 py-1 rounded text-sm bg-green-500 text-white"
                            } else {
                                "px-3 py-1 rounded text-sm bg-gray-200"
                            };
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    onclick: set_add_edge_mode,
                                    "Add Edge"
                                }
                            }
                        }
                        {
                            let btn_class = if *editing_mode.read() == EditingMode::DeleteEdge {
                                "px-3 py-1 rounded text-sm bg-red-500 text-white"
                            } else {
                                "px-3 py-1 rounded text-sm bg-gray-200"
                            };
                            rsx! {
                                button {
                                    class: "{btn_class}",
                                    onclick: set_delete_edge_mode,
                                    "Delete Edge"
                                }
                            }
                        }
                    }
                }

                // Selection info
                {
                    let mode_text = match *editing_mode.read() {
                        EditingMode::Normal => "Normal",
                        EditingMode::AddEdge => "Add Edge",
                        EditingMode::DeleteEdge => "Delete Edge",
                        EditingMode::AddNode => "Add Node",
                        EditingMode::DeleteNode => "Delete Node",
                    };
                    rsx! {
                        div { class: "mt-2 text-sm",
                            "Mode: {mode_text} | {selection_info}"
                        }
                    }
                }
                // Selected nodes for edge creation
                if *editing_mode.read() == EditingMode::AddEdge && !selected_nodes.read().is_empty() {
                    div { class: "text-sm",
                        "Selected nodes for edge: {selected_nodes.read().len()} selected"
                    }
                }
            }
            div { class: "flex-1 relative border-2 border-gray-300 rounded-lg overflow-hidden bg-white",
                svg {
                    class: "absolute top-0 left-0 w-full h-full",
                    onmousemove: handle_mousemove,
                    onmouseup: handle_mouseup,
                    onmouseleave: handle_mouseup,
                    onclick: handle_canvas_click,
                    // Draw edges with arrows (connecting nodes based on current positions)
                    for edge_idx in workflow.read().graph.edge_indices() {
                        {
                            let workflow_ref = workflow.read();
                            let positions_ref = node_positions.read();
                            let (source, target) = workflow_ref.graph.edge_endpoints(edge_idx).unwrap();
                            let source_pos = positions_ref.get(&source);
                            let target_pos = positions_ref.get(&target);

                            if let (Some(source_pos), Some(target_pos)) = (source_pos, target_pos) {
                                let edge_data = &workflow_ref.graph[edge_idx];
                                rsx! {
                                    GraphEdge {
                                        key: "{edge_idx.index()}",
                                        source_pos: source_pos.clone(),
                                        target_pos: target_pos.clone(),
                                        weight: 1, // Default weight for visualization
                                        edge_idx,
                                        on_click: handle_edge_click,
                                        is_selected: matches!(*current_selection.read(), Selection::Edge(selected_idx) if selected_idx == edge_idx),
                                    }
                                }
                            } else {
                                rsx! {
                                    g { key: "{edge_idx.index()}" }
                                }
                            }
                        }
                    }

                    // Draw nodes
                    for node_idx in workflow.read().graph.node_indices() {
                        {
                            let workflow_ref = workflow.read();
                            let positions_ref = node_positions.read();
                            if let Some(position) = positions_ref.get(&node_idx) {
                                let node_data = &workflow_ref.graph[node_idx];
                                rsx! {
                                    GraphNode {
                                        key: "{node_idx.index()}",
                                        position: position.clone(),
                                        label: node_data.name.clone(),
                                        node_idx,
                                        on_drag_start: handle_drag_start,
                                        on_click: handle_node_click,
                                        is_selected: matches!(*current_selection.read(), Selection::Node(selected_idx) if selected_idx == node_idx),
                                    }
                                }
                            } else {
                                rsx! {
                                    g { key: "{node_idx.index()}" }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "p-4 text-sm text-gray-600",
                "Workflow visualization with {workflow.read().graph.node_count()} nodes. Drag nodes to reposition them. Use tabs to switch between node and edge operations."
            }
        }
    }
}
