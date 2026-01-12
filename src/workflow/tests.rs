use super::{Edge, Node, Workflow};
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_workflow_yaml() {
        // Define the path to the workflow YAML file
        let workflow_path =
            PathBuf::from("documents/solar-radiation/calculate-cloud-correction/workflow.yaml");

        // Check if the file exists before attempting to parse
        if !workflow_path.exists() {
            // If the relative path doesn't work, try constructing the full path
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let full_path = current_dir.join("documents").join("workflow.yaml");

            if !full_path.exists() {
                panic!(
                    "Workflow YAML file not found at {:?} or {:?}",
                    workflow_path, full_path
                );
            }

            // Load the workflow from the full path
            let result = Workflow::load_from_path(full_path);
            assert!(
                result.is_ok(),
                "Failed to parse workflow YAML: {:?}",
                result.err()
            );

            let workflow = result.unwrap();

            // Validate basic properties
            assert_eq!(workflow.id, "2d4e1c6a-eb70-11f0-9b6c-7c70db10a7e3");
            assert_eq!(workflow.name, "CalculateCloudCorrectionFactor");
            assert_eq!(
                workflow.entry_graph_id,
                Some("a2df1dee-eba2-11f0-b126-7c70db10a7e3".to_string())
            );

            // Check that the workflow graph has nodes
            assert!(
                workflow.graph.node_count() > 0,
                "Workflow should have at least one node"
            );

            // Verify that nodes have been loaded with correct properties
            for node_idx in workflow.graph.node_indices() {
                let node = &workflow.graph[node_idx];
                assert!(!node.id.is_empty(), "Node ID should not be empty");
                assert!(!node.name.is_empty(), "Node name should not be empty");
                assert!(
                    !node.subgraph.is_empty(),
                    "Node subgraph should not be empty"
                );
            }

            println!(
                "Successfully parsed workflow with {} nodes",
                workflow.graph.node_count()
            );
        } else {
            // Load the workflow from the path
            let result = Workflow::load_from_path(workflow_path);
            assert!(
                result.is_ok(),
                "Failed to parse workflow YAML: {:?}",
                result.err()
            );

            let workflow = result.unwrap();

            // Validate basic properties
            assert_eq!(workflow.id, "2d4e1c6a-eb70-11f0-9b6c-7c70db10a7e3");
            assert_eq!(workflow.name, "CalculateCloudCorrectionFactor");
            assert_eq!(
                workflow.entry_graph_id,
                Some("a2df1dee-eba2-11f0-b126-7c70db10a7e3".to_string())
            );

            // Check that the workflow graph has nodes
            assert!(
                workflow.graph.node_count() > 0,
                "Workflow should have at least one node"
            );

            // Verify that nodes have been loaded with correct properties
            for node_idx in workflow.graph.node_indices() {
                let node = &workflow.graph[node_idx];
                assert!(!node.id.is_empty(), "Node ID should not be empty");
                assert!(!node.name.is_empty(), "Node name should not be empty");
                assert!(
                    !node.subgraph.is_empty(),
                    "Node subgraph should not be empty"
                );
            }

            println!(
                "Successfully parsed workflow with {} nodes",
                workflow.graph.node_count()
            );
        }
    }

    #[test]
    fn test_parse_simple_workflow_yaml_structure() {
        // Test that the workflow contains expected nodes from the YAML
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let workflow_path = current_dir
            .join("documents")
            .join("solar-radiation")
            .join("calculate-cloud-correction")
            .join("workflow.yaml");

        if !workflow_path.exists() {
            panic!("Workflow YAML file not found at {:?}", workflow_path);
        }

        let result = Workflow::load_from_path(workflow_path);
        assert!(
            result.is_ok(),
            "Failed to parse workflow YAML: {:?}",
            result.err()
        );

        let workflow = result.unwrap();

        // Count nodes with specific names that we know exist in the YAML
        let mut csv_reader_found = false;
        let mut rename_attributes_found = false;
        let mut prepare_extra_attr_found = false;

        for node_idx in workflow.graph.node_indices() {
            let node = &workflow.graph[node_idx];
            match node.name.as_str() {
                "CsvReader" => csv_reader_found = true,
                "RenameAttributes" => rename_attributes_found = true,
                "PrepareExtraAttribute" => prepare_extra_attr_found = true,
                _ => {}
            }
        }

        assert!(
            csv_reader_found,
            "CsvReader node should be present in the workflow"
        );
        assert!(
            rename_attributes_found,
            "RenameAttributes node should be present in the workflow"
        );
        assert!(
            prepare_extra_attr_found,
            "PrepareExtraAttribute node should be present in the workflow"
        );

        println!(
            "Found expected nodes: CsvReader={}, RenameAttributes={}, PrepareExtraAttribute={}",
            csv_reader_found, rename_attributes_found, prepare_extra_attr_found
        );
    }

    #[test]
    fn test_parse_complex_workflow_yaml_structure() {
        // Test that the workflow contains expected nodes from the YAML
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let workflow_path = current_dir
            .join("documents")
            .join("solar-radiation")
            .join("solar-potential")
            .join("workflow.yaml");

        if !workflow_path.exists() {
            panic!("Workflow YAML file not found at {:?}", workflow_path);
        }

        let result = Workflow::load_from_path(workflow_path);
        assert!(
            result.is_ok(),
            "Failed to parse workflow YAML: {:?}",
            result.err()
        );

        let workflow = result.unwrap();
    }
}
