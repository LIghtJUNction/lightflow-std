use lightflow::preload::*;
use lightflow::runner::Response;
use lightflow::serde_json::{Map, Value, json};
use regex::Regex;
use std::error::Error;
use std::fmt;

pub const WORKFLOW_ID: &str = "lightflow.text_regex";
pub const WORKFLOW_VERSION: &str = env!("CARGO_PKG_VERSION");
const SOURCE_DIGEST: u64 = source_digest(include_bytes!("lib.rs"));

pub fn define() -> WorkflowSpec {
    workflow! {
        name: "Text Regex",
        description: "Match or replace text with a regular expression.",
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
    .builtin_runtime("text_regex", "lightflow.text.regex", "runner.v1")
    .build()
}

/// Execute the package-owned regex implementation.
///
/// The adjacent runner binary exposes this function through LightFlow's stable
/// JSON-over-stdio runner protocol.
pub fn execute(inputs: &Map<String, Value>) -> Result<Response, TextRegexError> {
    let text = required_string(inputs, "text")?;
    let pattern = required_string(inputs, "pattern")?;
    let replacement = optional_string(inputs, "replacement")?;
    let _operation = optional_string(inputs, "operation")?;
    let _flags = optional_string(inputs, "flags")?;
    let regex = Regex::new(pattern).map_err(TextRegexError::Regex)?;

    let captures = regex
        .captures_iter(text)
        .map(|captures| {
            Value::Array(
                captures
                    .iter()
                    .map(|capture| {
                        capture
                            .map(|capture| Value::String(capture.as_str().to_owned()))
                            .unwrap_or(Value::Null)
                    })
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    let match_count = captures.len();
    let result = replacement
        .map(|replacement| regex.replace_all(text, replacement).into_owned())
        .unwrap_or_else(|| text.to_owned());
    let first_match = regex
        .find(text)
        .map(|found| found.as_str().to_owned())
        .unwrap_or_default();

    Ok(Response {
        outputs: Map::from_iter([
            ("text".to_owned(), Value::String(result)),
            ("matched".to_owned(), Value::Bool(match_count > 0)),
            ("match_count".to_owned(), json!(match_count)),
            ("captures".to_owned(), Value::Array(captures)),
            ("first_match".to_owned(), Value::String(first_match)),
        ]),
        artifacts: Vec::new(),
        replay_fingerprint: Map::from_iter([
            ("crate_version".to_owned(), json!(WORKFLOW_VERSION)),
            (
                "implementation".to_owned(),
                json!(implementation_identity()),
            ),
        ]),
    })
}

pub fn implementation_identity() -> String {
    format!("lightflow.text_regex.source.fnv1a64:{SOURCE_DIGEST:016x}")
}

#[derive(Debug)]
pub enum TextRegexError {
    MissingInput(&'static str),
    InvalidInputType(&'static str),
    Regex(regex::Error),
}

impl fmt::Display for TextRegexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput(name) => write!(formatter, "required input `{name}` is missing"),
            Self::InvalidInputType(name) => {
                write!(formatter, "input `{name}` must be a JSON string")
            }
            Self::Regex(error) => write!(formatter, "invalid regex pattern: {error}"),
        }
    }
}

impl Error for TextRegexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Regex(error) => Some(error),
            Self::MissingInput(_) | Self::InvalidInputType(_) => None,
        }
    }
}

fn required_string<'a>(
    inputs: &'a Map<String, Value>,
    name: &'static str,
) -> Result<&'a str, TextRegexError> {
    inputs
        .get(name)
        .ok_or(TextRegexError::MissingInput(name))?
        .as_str()
        .ok_or(TextRegexError::InvalidInputType(name))
}

fn optional_string<'a>(
    inputs: &'a Map<String, Value>,
    name: &'static str,
) -> Result<Option<&'a str>, TextRegexError> {
    inputs
        .get(name)
        .map(|value| value.as_str().ok_or(TextRegexError::InvalidInputType(name)))
        .transpose()
}

const fn source_digest(source: &[u8]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    let mut index = 0;
    while index < source.len() {
        digest ^= source[index] as u64;
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_matches_replaces_counts_and_captures() {
        let inputs = Map::from_iter([
            ("text".to_owned(), json!("cat 42 and 7")),
            ("pattern".to_owned(), json!(r"(\d+)")),
            ("replacement".to_owned(), json!("id:$1")),
        ]);
        let response = execute(&inputs).expect("regex execution");

        assert_eq!(response.outputs["text"], "cat id:42 and id:7");
        assert_eq!(response.outputs["matched"], true);
        assert_eq!(response.outputs["match_count"], 2);
        assert_eq!(
            response.outputs["captures"],
            json!([["42", "42"], ["7", "7"]])
        );
        assert_eq!(response.outputs["first_match"], "42");
    }

    #[test]
    fn execute_without_replacement_preserves_text_and_reports_no_match() {
        let inputs = Map::from_iter([
            ("text".to_owned(), json!("cat")),
            ("pattern".to_owned(), json!(r"\d+")),
        ]);
        let response = execute(&inputs).expect("regex execution");

        assert_eq!(response.outputs["text"], "cat");
        assert_eq!(response.outputs["matched"], false);
        assert_eq!(response.outputs["match_count"], 0);
        assert_eq!(response.outputs["captures"], json!([]));
        assert_eq!(response.outputs["first_match"], "");
    }

    #[test]
    fn execute_requires_string_text_and_pattern() {
        for inputs in [
            Map::from_iter([("pattern".to_owned(), json!("x"))]),
            Map::from_iter([
                ("text".to_owned(), json!(42)),
                ("pattern".to_owned(), json!("x")),
            ]),
            Map::from_iter([("text".to_owned(), json!("text"))]),
            Map::from_iter([
                ("text".to_owned(), json!("text")),
                ("pattern".to_owned(), json!(false)),
            ]),
        ] {
            let error = execute(&inputs).expect_err("invalid required input");
            assert!(
                error.to_string().contains("required input")
                    || error.to_string().contains("must be a JSON string")
            );
        }
    }

    #[test]
    fn execute_rejects_non_string_optional_inputs() {
        for name in ["replacement", "operation", "flags"] {
            let inputs = Map::from_iter([
                ("text".to_owned(), json!("text")),
                ("pattern".to_owned(), json!("x")),
                (name.to_owned(), json!({"invalid": true})),
            ]);
            let error = execute(&inputs).expect_err("invalid optional input");
            assert!(error.to_string().contains(name));
            assert!(error.to_string().contains("must be a JSON string"));
        }
    }

    #[test]
    fn implementation_identity_is_stable_and_non_empty() {
        let first = implementation_identity();
        assert!(!first.trim().is_empty());
        assert_eq!(first, implementation_identity());
        assert!(first.starts_with("lightflow.text_regex.source.fnv1a64:"));
    }
}
