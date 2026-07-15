use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "value": "json" {
            description: "Structured source value to convert into prompt text.",
            required: true,
            widget: "json",
        }
        output "prompt": "text" {
            description: "Prompt text generated from the source value.",
        }
    }
    .name("Text Prompt")
    .description("Build a text prompt from structured input.")
    .build()
}
