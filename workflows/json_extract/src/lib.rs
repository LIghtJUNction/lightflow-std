use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "JSON Extract",
        description: "Extract a value from JSON by dot path.",
        input "value": "json" {
            description: "Source JSON value.",
            required: true,
            widget: "json",
        }
        input "path": "text" {
            description: "Dot path to extract, such as user.name or items.0.title.",
            required: true,
            widget: "text",
        }
        output "value": "json" {
            description: "Extracted JSON value, or null when missing.",
        }
        output "text": "text" {
            description: "Extracted value coerced to text.",
        }
        output "found": "boolean" {
            description: "Whether a non-null value was found.",
        }
    }
    .builtin_runtime("json_extract", "lightflow.json.extract", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.json_extract");
