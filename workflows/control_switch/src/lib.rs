use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Control Switch",
        description: "Select a JSON value from a cases object by selector.",
        input "selector": "text" {
            description: "Case key to select.",
            required: true,
            widget: "text",
        }
        input "cases": "json" {
            description: "JSON object mapping selector keys to values.",
            required: true,
            widget: "json",
        }
        input "default": "json" {
            description: "Fallback value when selector is missing.",
            required: false,
            widget: "json",
        }
        output "value": "json" {
            description: "Selected case value.",
        }
        output "selected": "text" {
            description: "Selected case key or default.",
        }
    }
    .builtin_runtime("control_switch", "lightflow.control.switch", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.control_switch");
