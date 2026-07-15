use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "mask_a_path": "path" {
            description: "First PNG mask path.",
            required: true,
            widget: "image",
            artifact: "mask",
        }
        input "mask_b_path": "path" {
            description: "Second PNG mask path. It is resized to the first mask dimensions when needed.",
            required: true,
            widget: "image",
            artifact: "mask",
        }
        input "mode": "text" {
            description: "Composition mode: max, add, multiply, min, subtract, union, or intersect.",
            required: false,
            default: "max",
            choices: ["max","add","multiply","min","subtract","union","intersect"],
            widget: "select",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "mask",
        }
        output "mask": "artifact" {
            description: "Composed mask artifact metadata.",
            artifact: "mask",
        }
        output "mask_path": "path" {
            description: "Path to the composed grayscale PNG mask.",
            artifact: "mask",
        }
        output "mode": "text" {
            description: "Composition mode used.",
        }
    }
        .name("Mask Compose")
        .description("Compose two PNG masks into one grayscale mask.")
        .runtime("mask_compose", "lightflow.mask.compose")
        .build()
}
