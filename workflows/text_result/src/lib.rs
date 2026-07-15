use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "text": "text" {
            description: "Generated text to expose through the final result port.",
            required: true,
            widget: "textarea",
        }
        output "result": "text" {
            description: "Final normalized text result.",
        }
    }
    .name("Text Result")
    .description("Normalize generated text into a final result.")
    .build()
}
