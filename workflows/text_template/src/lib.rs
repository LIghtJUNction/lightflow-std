use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "template": "text" {
            description: "Template text with placeholders such as {{topic}}.",
            required: true,
            widget: "textarea",
        }
        input "vars": "json" {
            description: "JSON object used to fill template placeholders.",
            required: false,
            default: {},
            widget: "json",
        }
        output "text": "text" {
            description: "Rendered template text.",
        }
    }
    .name("Text Template")
    .description("Render a text template with JSON variables.")
    .runtime("text_template", "lightflow.text.template")
    .build()
}
