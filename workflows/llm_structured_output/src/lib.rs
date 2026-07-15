use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "text": "text" {
            description: "LLM text or JSON string.",
            required: true,
            widget: "textarea",
        }
        input "schema": "json" {
            description: "Optional expected schema metadata for editor tooling.",
            required: false,
            widget: "json",
        }
        output "object": "json" {
            description: "Parsed JSON value or object containing the original text.",
        }
        output "json": "text" {
            description: "Serialized JSON output.",
        }
    }
    .name("LLM Structured Output")
    .description("Parse LLM text into JSON, or wrap plain text as a JSON object.")
    .runtime("llm_structured_output", "lightflow.llm.structured_output")
    .build()
}
