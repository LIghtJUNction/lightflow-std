use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "value": "json" {
            description: "Source JSON value to split.",
            required: true,
            widget: "json",
        }
        output "first": "json" {
            description: "First array item, first object entry, or the scalar value.",
        }
        output "rest": "json" {
            description: "Remaining array items or object entries.",
        }
        output "items": "json" {
            description: "Array representation of the source value.",
        }
    }
    .name("Control Split")
    .description("Split a JSON array or object into first, rest, and items outputs.")
    .runtime("control_split", "lightflow.control.split")
    .build()
}
