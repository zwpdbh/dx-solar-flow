use crate::components::graph::Point;
use dioxus::prelude::*;

#[component]
pub fn Node(
    position: Point,
    label: String,
    node_idx: petgraph::graph::NodeIndex,
    on_drag_start: EventHandler<petgraph::graph::NodeIndex>,
    on_click: EventHandler<petgraph::graph::NodeIndex>,
    is_selected: bool,
) -> Element {
    let handle_node_mousedown = move |event: MouseEvent| {
        event.prevent_default();
        event.stop_propagation();
        on_drag_start.call(node_idx);
    };

    let handle_node_click = move |event: MouseEvent| {
        event.prevent_default();
        event.stop_propagation();
        on_click.call(node_idx);
    };

    // Determine node color based on selection state
    let fill_color = if is_selected { "lightgreen" } else { "lightblue" };
    let stroke_color = if is_selected { "darkgreen" } else { "black" };

    rsx! {
        g {
            // Draggable node circle
            circle {
                cx: "{position.x}",
                cy: "{position.y}",
                r: "25",
                fill: fill_color,
                stroke: stroke_color,
                stroke_width: "2",
                cursor: "move",
                onmousedown: handle_node_mousedown,
                onclick: handle_node_click,
            }
            // Node label
            text {
                x: "{position.x}",
                y: "{position.y}",
                text_anchor: "middle",
                dominant_baseline: "middle",
                font_size: "10",
                font_weight: "bold",
                fill: "black",
                pointer_events: "none", // So clicks go through to the circle
                "{label}"
            }
        }
    }
}