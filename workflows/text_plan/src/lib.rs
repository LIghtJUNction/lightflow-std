use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Text Plan",
        description: "Example composite workflow built from workflow nodes.",
        input "value": "json" {
            description: "JSON value passed into the example plan.",
            required: true,
            widget: "json",
        }
        output "result": "text" {
            description: "Final text result from the example plan.",
        }
        node prompt: "lightflow.text_prompt",
        node result: "lightflow.text_result",
        edge prompt.prompt -> result.text,
    }
    .build()
}
