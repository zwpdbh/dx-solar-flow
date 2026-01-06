use crate::components::graph::Point;
use dioxus::prelude::*;

#[component]
pub fn Edge(
    source_pos: Point,
    target_pos: Point,
    weight: i32,
    edge_idx: petgraph::graph::EdgeIndex,
    on_click: EventHandler<petgraph::graph::EdgeIndex>,
    is_selected: bool,
) -> Element {
    // Calculate direction vector for arrow
    let dx = target_pos.x - source_pos.x;
    let dy = target_pos.y - source_pos.y;
    let length = (dx * dx + dy * dy).sqrt();

    // Normalize and calculate arrow offset
    let unit_x = dx / length;
    let unit_y = dy / length;

    // Start from node border (not center)
    let start_offset = 25.0; // Radius of node
    let end_offset = 25.0; // Radius of node

    let start_x = source_pos.x + unit_x * start_offset;
    let start_y = source_pos.y + unit_y * start_offset;
    let end_x = target_pos.x - unit_x * end_offset;
    let end_y = target_pos.y - unit_y * end_offset;

    // Calculate arrowhead points
    let arrow_size = 10.0;
    let angle = dy.atan2(dx);
    let arrow_angle = std::f64::consts::PI / 6.0; // 30 degrees

    let arrow_x1 = end_x - arrow_size * (angle - arrow_angle).cos();
    let arrow_y1 = end_y - arrow_size * (angle - arrow_angle).sin();
    let arrow_x2 = end_x - arrow_size * (angle + arrow_angle).cos();
    let arrow_y2 = end_y - arrow_size * (angle + arrow_angle).sin();

    // Determine edge color based on selection state
    let stroke_color = if is_selected { "darkgreen" } else { "blue" };
    let stroke_width = if is_selected { "3" } else { "2" };

    let handle_edge_click = move |event: MouseEvent| {
        event.prevent_default();
        event.stop_propagation();
        on_click.call(edge_idx);
    };

    rsx! {
        g {
            // Invisible hit area for easier selection (wider line behind the visible edge)
            line {
                x1: "{start_x}",
                y1: "{start_y}",
                x2: "{end_x}",
                y2: "{end_y}",
                stroke: "transparent",
                stroke_width: "10", // Much wider for easier clicking
                cursor: "pointer",
                onclick: handle_edge_click,
            }
            // Edge line
            line {
                x1: "{start_x}",
                y1: "{start_y}",
                x2: "{end_x}",
                y2: "{end_y}",
                stroke: stroke_color,
                stroke_width,
                cursor: "pointer",
                onclick: handle_edge_click,
            }
            // Arrowhead
            line {
                x1: "{end_x}",
                y1: "{end_y}",
                x2: "{arrow_x1}",
                y2: "{arrow_y1}",
                stroke: stroke_color,
                stroke_width,
                cursor: "pointer",
                onclick: handle_edge_click,
            }
            line {
                x1: "{end_x}",
                y1: "{end_y}",
                x2: "{arrow_x2}",
                y2: "{arrow_y2}",
                stroke: stroke_color,
                stroke_width,
                cursor: "pointer",
                onclick: handle_edge_click,
            }
            // Edge weight label
            text {
                x: "{(start_x + end_x) / 2.0 + 10.0}",
                y: "{(start_y + end_y) / 2.0 - 10.0}",
                fill: "red",
                font_size: "12",
                font_weight: "bold",
                "{weight}"
            }
        }
    }
}
