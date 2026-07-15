use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "prompt": "text" {
            description: "Positive text prompt used for image generation.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt.",
            required: false,
            default: "",
            widget: "textarea",
        }
        input "width": "integer" {
            description: "Output image width in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "height": "integer" {
            description: "Output image height in pixels.",
            required: false,
            default: 512,
            range: [64, 2048, 8],
            widget: "number",
        }
        input "seed": "integer" {
            description: "Optional deterministic generation seed.",
            required: false,
            widget: "seed",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        input "model": "text" {
            description: "Optional model variant id for the image_model requirement.",
            required: false,
            choices: ["sdxl-gguf-q4","sdxl-safetensors"],
            widget: "model_select",
            model: "image_model",
        }
        output "image": "artifact" {
            description: "Generated image artifact metadata.",
            artifact: "image",
            model: "image_model",
        }
        output "image_path": "path" {
            description: "Path to the generated PNG image.",
            artifact: "image",
            model: "image_model",
        }
    }
    .name("Text To Image")
    .description("Generate an image artifact from a text prompt through a pluggable image runtime.")
    .builtin_runtime(
        "image_runtime",
        "lightflow.image.generate",
        "builtin.preview.v1",
    )
    .hf_model(
        "image_model",
        "sdxl-gguf-q4",
        "text-to-image",
        "gguf",
        "city96/stable-diffusion-xl-base-1.0-gguf",
        "sd_xl_base_1.0-q4_k_m.gguf",
    )
    .hf_model(
        "image_model",
        "sdxl-safetensors",
        "text-to-image",
        "safetensors",
        "stabilityai/stable-diffusion-xl-base-1.0",
        "sd_xl_base_1.0.safetensors",
    )
    .build()
}
