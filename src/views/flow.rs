use crate::workflow::Workflow;
use dioxus::prelude::*;
use std::{fs, path::Path};

#[component]
pub fn FlowPage() -> Element {
    let mut workflow_file_path = use_signal(|| String::new());
    let mut file_info = use_signal(|| None::<Result<u64, String>>);
    let mut is_loading = use_signal(|| false);
    let mut workflow = use_signal(|| None);
    let mut workflow_err = use_signal(|| None);

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "workflow loader" }
            p {
                "/home/zw/code/rust_programming/reearth-flow/engine/runtime/examples/fixture/workflow/solar-radiation/solar-potential/workflow.yaml"
            }

            // Render workflow error if it exists
            {
                if let Some(err) = workflow_err.read().as_ref() {
                    rsx! {
                        div { class: "mb-4 p-3 bg-red-100 text-red-700 rounded", "Workflow Error: {err}" }
                    }
                } else {
                    rsx! {}
                }
            }

            div { class: "mb-4",
                input {
                    class: "border border-gray-300 rounded px-3 py-2 w-full max-w-md",
                    r#type: "text",
                    placeholder: "path to workflow file",
                    value: "{workflow_file_path}",
                    disabled: *is_loading.read(),
                    oninput: move |evt| workflow_file_path.set(evt.value().to_string()),
                }
            }

            div { class: "mb-4",
                button {
                    class: if *is_loading.read() { "bg-gray-400 text-white font-bold py-2 px-4 rounded cursor-not-allowed" } else { "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" },
                    disabled: *is_loading.read(),
                    onclick: move |_| {
                        let workflow_path = workflow_file_path.read().clone();
                        let workflow_path = Path::new(&workflow_path).to_path_buf();
                        if workflow_path.is_file() {
                            is_loading.set(true);

                            // Attempt to read file metadata
                            match fs::metadata(&workflow_path) {
                                Ok(metadata) => {
                                    let size = metadata.len();
                                    file_info.set(Some(Ok(size)));
                                    match Workflow::load_from_path(workflow_path) {
                                        Ok(flow) => {
                                            workflow.set(Some(flow));
                                            workflow_err.set(None); // Clear any previous error
                                        }
                                        Err(e) => {
                                            workflow_err.set(Some(e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    file_info.set(Some(Err(e.to_string())));
                                }
                            }
                            is_loading.set(false);
                        }

                    },
                    if *is_loading.read() {
                        "Loading..."
                    } else {
                        "Load"
                    }
                }
            }

            // Display file info or error
            {
                if *is_loading.read() {
                    rsx! {
                        div { class: "text-blue-600", "Loading file: {workflow_file_path.read()}" }
                    }
                } else if let Some(result) = file_info.read().as_ref() {
                    match result {
                        Ok(size) => {
                            // Show success message if workflow loaded successfully
                            if workflow.read().is_some() {
                                rsx! {
                                    div { class: "text-green-600", "Workflow loaded successfully" }
                                }
                            } else {
                                rsx! {
                                    div { class: "text-green-600", "File size: {size} bytes" }
                                }
                            }
                        }
                        Err(error_msg) => {
                            rsx! {
                                div { class: "text-red-600", "Error: {error_msg}" }
                            }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "text-gray-500", "Enter a file path and click Load" }
                    }
                }

            }

        }
    }
}
