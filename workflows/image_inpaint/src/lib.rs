use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Image Inpaint Preview",
        description: "Create a deterministic preview inpaint from a source PNG, mask, and prompt.",
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "mask_path": "path" {
            description: "PNG mask path. White pixels receive the strongest preview edit.",
            required: true,
            widget: "image",
            artifact: "mask",
        }
        input "prompt": "text" {
            description: "Inpaint prompt.",
            required: true,
            widget: "prompt",
        }
        input "negative": "text" {
            description: "Optional negative prompt.",
            required: false,
            widget: "textarea",
        }
        input "seed": "integer" {
            description: "Optional deterministic seed.",
            required: false,
            widget: "number",
        }
        input "output_path": "path" {
            description: "Optional destination PNG path.",
            required: false,
            widget: "file_save",
            artifact: "image",
        }
        output "image": "artifact" {
            description: "Inpainted preview image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the inpainted preview PNG image.",
            artifact: "image",
        }
        output "prompt": "text" {
            description: "Prompt used for the preview inpaint.",
        }
    }
    .builtin_runtime(
        "image_inpaint_preview",
        "lightflow.image.inpaint",
        "runner.v1",
    )
    .hf_model(
        "image_model",
        "flux-inpaint-preview",
        "text-to-image",
        "gguf",
        "lightflow/preview",
        "flux-inpaint-preview.gguf",
    )
    .build()
}

lightflow_std_runtime::standard_runner!("lightflow.image_inpaint");
