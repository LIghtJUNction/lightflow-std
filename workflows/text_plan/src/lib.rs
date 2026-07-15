use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "value": "json" {
            description: "JSON value passed into the example plan.",
            required: true,
            widget: "json",
        }
        output "result": "text" {
            description: "Final text result from the example plan.",
        }
    }
    .name("Text Plan")
    .description("Example composite workflow built from workflow nodes.")
    .depends_on("lightflow.text_prompt", "0.1.0")
    .depends_on("lightflow.text_result", "0.1.0")
    .node("prompt", "lightflow.text_prompt")
    .node("result", "lightflow.text_result")
    .edge("prompt", "prompt", "result", "text")
    .build()
}
