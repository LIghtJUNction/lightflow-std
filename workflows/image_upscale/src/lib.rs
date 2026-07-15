use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "scale": "integer" {
            description: "Integer upscale factor.",
            required: false,
            default: 2,
            range: [1, 16, 1],
            widget: "number",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Upscaled image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the upscaled PNG image.",
            artifact: "image",
        }
    }
    .name("Image Upscale")
    .description("Upscale a PNG image by an integer scale factor.")
    .runtime("image_upscale", "lightflow.image.upscale")
    .build()
}
