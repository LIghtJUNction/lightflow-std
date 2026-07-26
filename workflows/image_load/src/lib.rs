use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Image Load",
        description: "Load a PNG image path into an image artifact handle.",
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Loaded image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Source PNG image path.",
            artifact: "image",
        }
    }
    .builtin_runtime("image_load", "lightflow.image.load", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.image_load");
