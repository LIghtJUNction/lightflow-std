use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "LLM Generate",
        description: "Generate deterministic mock LLM text for offline workflow composition.",
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
    .builtin_runtime("llm_mock", "lightflow.llm.generate", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.llm_generate");
