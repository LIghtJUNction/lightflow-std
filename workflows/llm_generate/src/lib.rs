use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "prompt": "text" {
            description: "Prompt text.",
            required: true,
            widget: "prompt",
        }
        input "model": "text" {
            description: "Mock model name.",
            required: false,
            default: "mock",
            widget: "text",
        }
        output "text": "text" {
            description: "Generated mock text.",
        }
        output "response": "text" {
            description: "Generated mock response.",
        }
    }
    .name("LLM Generate")
    .description("Generate deterministic mock LLM text for offline workflow composition.")
    .builtin_runtime("llm_mock", "lightflow.llm.generate", "builtin.llm.mock.v1")
    .build()
}
