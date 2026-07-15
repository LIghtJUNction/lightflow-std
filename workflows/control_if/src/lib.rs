use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "condition": "boolean" {
            description: "Boolean condition used to choose the branch.",
            required: true,
            widget: "toggle",
        }
        input "then_value": "json" {
            description: "Value emitted when condition is true.",
            required: false,
            widget: "json",
        }
        input "else_value": "json" {
            description: "Value emitted when condition is false.",
            required: false,
            widget: "json",
        }
        output "value": "json" {
            description: "Selected branch value.",
        }
        output "selected": "text" {
            description: "Selected branch name: then or else.",
        }
    }
    .name("Control If")
    .description("Select one of two JSON values based on a boolean condition.")
    .runtime("control_if", "lightflow.control.if")
    .build()
}
