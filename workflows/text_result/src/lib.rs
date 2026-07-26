use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Text Result",
        description: "Normalize generated text into a final result.",
        input "text": "text" {
            description: "Generated text to expose through the final result port.",
            required: true,
            widget: "textarea",
        }
        output "result": "text" {
            description: "Final normalized text result.",
        }
    }
    .builtin_runtime("text_result", "lightflow.text.result", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.text_result");
