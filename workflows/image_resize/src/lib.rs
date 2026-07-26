use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Image Resize",
        description: "Resize a PNG image with nearest-neighbor sampling.",
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "width": "integer" {
            description: "Output width in pixels.",
            required: true,
            range: [1, 8192, 1],
            widget: "number",
        }
        input "height": "integer" {
            description: "Output height in pixels.",
            required: true,
            range: [1, 8192, 1],
            widget: "number",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Resized image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the resized PNG image.",
            artifact: "image",
        }
    }
    .builtin_runtime("image_resize", "lightflow.image.resize", "runner.v1")
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.image_resize");
