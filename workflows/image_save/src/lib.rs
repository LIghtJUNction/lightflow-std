use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Image Save",
        description: "Copy a PNG image to a selected output path.",
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Saved image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Saved PNG image path.",
            artifact: "image",
        }
    }
    .builtin_runtime("image_save", "lightflow.image.save", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.image_save");
