use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Text Concat",
        description: "Concatenate two text values or an array of text values.",
        input "a": "text" {
            description: "First text value when items is not provided.",
            required: false,
            widget: "textarea",
        }
        input "b": "text" {
            description: "Second text value when items is not provided.",
            required: false,
            widget: "textarea",
        }
        input "items": "json" {
            description: "Optional array of values to concatenate.",
            required: false,
            widget: "json",
        }
        input "separator": "text" {
            description: "Text inserted between concatenated values.",
            required: false,
            default: "",
            widget: "text",
        }
        output "text": "text" {
            description: "Concatenated text.",
        }
    }
    .builtin_runtime("text_concat", "lightflow.text.concat", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.text_concat");
