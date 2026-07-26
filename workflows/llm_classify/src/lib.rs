use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "LLM Classify",
        description: "Classify text against a list of labels with deterministic offline matching.",
        input "text": "text" {
            description: "Text to classify.",
            required: true,
            widget: "textarea",
        }
        input "labels": "json" {
            description: "Array of candidate label strings.",
            required: true,
            widget: "json",
        }
        output "label": "text" {
            description: "Selected label.",
        }
        output "confidence": "number" {
            description: "Deterministic confidence score.",
        }
    }
    .builtin_runtime("llm_classify", "lightflow.llm.classify", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.llm_classify");
