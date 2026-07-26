use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Model Select",
        description: "Select a model variant from a JSON variant list.",
        input "requirement_id": "text" {
            description: "Model requirement id.",
            required: false,
            widget: "text",
        }
        input "variants": "json" {
            description: "Array of model variants with id, format, repo, and file fields.",
            required: true,
            widget: "json",
        }
        input "preferred": "text" {
            description: "Preferred variant id or format.",
            required: false,
            widget: "text",
        }
        output "model": "json" {
            description: "Selected model variant object.",
        }
        output "variant_id": "text" {
            description: "Selected model variant id.",
        }
        output "requirement_id": "text" {
            description: "Model requirement id.",
        }
    }
    .builtin_runtime("model_select", "lightflow.model.select", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.model_select");
