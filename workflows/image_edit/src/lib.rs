use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "image_path": "path" {
            description: "Source PNG image path.",
            required: true,
            widget: "image",
            artifact: "image",
        }
        input "prompt": "text" {
            description: "Edit prompt.",
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
            description: "Edited preview image artifact metadata.",
            artifact: "image",
        }
        output "image_path": "path" {
            description: "Path to the edited preview PNG image.",
            artifact: "image",
        }
        output "prompt": "text" {
            description: "Prompt used for the preview edit.",
        }
    }
    .name("Image Edit Preview")
    .description("Create a deterministic preview image edit from a source PNG and prompt.")
    .builtin_runtime(
        "image_edit_preview",
        "lightflow.image.edit",
        "builtin.preview.edit.v1",
    )
    .hf_model(
        "image_model",
        "flux-edit-preview",
        "text-to-image",
        "gguf",
        "lightflow/preview",
        "flux-edit-preview.gguf",
    )
    .build()
}
