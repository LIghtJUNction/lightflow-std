use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Model Lock Check",
        description: "Check whether a workflow model requirement is recorded in lfw.lock.",
        input "workflow_id": "text" {
            description: "Workflow id used in the lock key.",
            required: true,
            widget: "text",
        }
        input "requirement_id": "text" {
            description: "Model requirement id used in the lock key.",
            required: true,
            widget: "text",
        }
        output "locked": "boolean" {
            description: "Whether lfw.lock contains this model key.",
        }
        output "exists": "boolean" {
            description: "Whether the first locked local path exists.",
        }
        output "path": "path" {
            description: "First locked local model path, when present.",
        }
        output "entry": "json" {
            description: "Raw lfw.lock model entry.",
        }
    }
    .builtin_runtime(
        "model_lock_check",
        "lightflow.model.lock.check",
        "runner.v1",
    )
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.model_lock_check");
