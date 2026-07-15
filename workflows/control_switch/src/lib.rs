use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
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
    .name("Control Switch")
    .description("Select a JSON value from a cases object by selector.")
    .runtime("control_switch", "lightflow.control.switch")
    .build()
}
