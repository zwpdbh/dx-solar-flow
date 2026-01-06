use crate::components::Graph;
use dioxus::prelude::*;
use petgraph::Graph as PetGraph;

#[component]
pub fn GraphPage() -> Element {
    let graph = use_signal(|| {
        let mut g = PetGraph::<String, i32>::new();

        // Add some example nodes (cities)
        let nyc = g.add_node("New York".to_string());
        let la = g.add_node("Los Angeles".to_string());
        let chicago = g.add_node("Chicago".to_string());
        let houston = g.add_node("Houston".to_string());

        // Add some example edges (connections between cities)
        g.add_edge(nyc, la, 100);
        g.add_edge(nyc, chicago, 50);
        g.add_edge(chicago, houston, 75);
        g.add_edge(la, houston, 120);
        g.add_edge(houston, la, 110);

        g
    });

    rsx! {
        Graph {
            graph: graph,
            initial_positions: None,
        }
    }
}
