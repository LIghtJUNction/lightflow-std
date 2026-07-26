use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Text Prompt",
        description: "Build a text prompt from structured input.",
        input "value": "json" {
            description: "Structured source value to convert into prompt text.",
            required: true,
            widget: "json",
        }
        output "prompt": "text" {
            description: "Prompt text generated from the source value.",
        }
    }
    .builtin_runtime("text_prompt", "lightflow.text.prompt", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.text_prompt");
