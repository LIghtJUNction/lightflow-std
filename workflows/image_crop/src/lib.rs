use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "x": "integer" {
            description: "Left crop coordinate in pixels.",
            required: false,
            default: 0,
            range: [0, 8192, 1],
            widget: "number",
        }
        input "y": "integer" {
            description: "Top crop coordinate in pixels.",
            required: false,
            default: 0,
            range: [0, 8192, 1],
            widget: "number",
        }
        input "width": "integer" {
            description: "Crop width in pixels.",
            required: true,
            range: [1, 8192, 1],
            widget: "number",
        }
        input "height": "integer" {
            description: "Crop height in pixels.",
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
            description: "Cropped image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the cropped PNG image.",
            artifact: "image",
        }
    }
    .name("Image Crop")
    .description("Crop a rectangular region from a PNG image.")
    .runtime("image_crop", "lightflow.image.crop")
    .build()
}
