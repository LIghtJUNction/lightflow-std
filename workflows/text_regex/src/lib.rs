use lightflow::preload::*;

pub fn define() -> WorkflowSpec {
    workflow! {
        input "text": "text" {
            description: "Source text.",
            required: true,
            widget: "textarea",
        }
        input "pattern": "text" {
            description: "Rust regex pattern.",
            required: true,
            widget: "text",
        }
        input "replacement": "text" {
            description: "Optional replacement text for all matches.",
            required: false,
            widget: "text",
        }
        output "text": "text" {
            description: "Replaced text, or original text when no replacement is set.",
        }
        output "matched": "boolean" {
            description: "Whether the pattern matched at least once.",
        }
        output "match_count": "integer" {
            description: "Number of matches.",
        }
        output "captures": "json" {
            description: "Array of capture groups for each match.",
        }
        output "first_match": "text" {
            description: "First matched substring.",
        }
    }
    .name("Text Regex")
    .description("Match or replace text with a regular expression.")
    .runtime("text_regex", "lightflow.text.regex")
    .build()
}
