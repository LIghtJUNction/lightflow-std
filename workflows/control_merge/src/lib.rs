use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "a": "json" {
            description: "First value.",
            required: false,
            widget: "json",
        }
        input "b": "json" {
            description: "Second value.",
            required: false,
            widget: "json",
        }
        input "mode": "text" {
            description: "Merge mode: first_non_null, object, or array.",
            required: false,
            default: "first_non_null",
            choices: ["first_non_null","object","array"],
            widget: "select",
        }
        output "value": "json" {
            description: "Merged value.",
        }
        output "selected": "text" {
            description: "Merge mode used.",
        }
    }
    .name("Control Merge")
    .description("Merge two JSON values using a selected merge mode.")
    .runtime("control_merge", "lightflow.control.merge")
    .build()
}
