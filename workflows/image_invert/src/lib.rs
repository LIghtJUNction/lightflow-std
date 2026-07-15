use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
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
            description: "Image artifact metadata for the inverted PNG.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the inverted PNG.",
            artifact: "image",
        }
    }
    .name("Image Invert")
    .description("Invert the colors of an input PNG image.")
    .builtin_runtime(
        "image_runtime",
        "lightflow.image.invert",
        "builtin.image.invert.v1",
    )
    .build()
}
