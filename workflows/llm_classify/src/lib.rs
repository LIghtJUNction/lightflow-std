use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
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
    .name("LLM Classify")
    .description("Classify text against a list of labels with deterministic offline matching.")
    .runtime("llm_classify", "lightflow.llm.classify")
    .build()
}
