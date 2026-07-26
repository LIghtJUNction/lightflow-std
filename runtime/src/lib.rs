//! Package-owned implementations shared by LightFlow standard workflow crates.

mod control;
mod image;
mod input;
mod text;

use lightflow::runner::Response;
use lightflow::serde_json::{Map, Value};
use std::error::Error;
use std::fmt;

const RUNTIME_SOURCE_DIGEST: u64 = hash_source(
    hash_source(
        hash_source(
            hash_source(
                hash_source(0xcbf2_9ce4_8422_2325, include_bytes!("lib.rs")),
                include_bytes!("input.rs"),
            ),
            include_bytes!("text.rs"),
        ),
        include_bytes!("control.rs"),
    ),
    include_bytes!("image.rs"),
);

/// Execute one standard workflow without invoking LightFlow core business logic.
pub fn execute(
    workflow_id: &str,
    version: &str,
    implementation: &str,
    inputs: &Map<String, Value>,
) -> Result<Response, RuntimeError> {
    let (outputs, artifacts) = match workflow_id {
        "lightflow.text_prompt" => text::prompt(inputs)?,
        "lightflow.text_result" => text::result(inputs)?,
        "lightflow.text_concat" => text::concat(inputs)?,
        "lightflow.text_template" => text::template(inputs)?,
        "lightflow.json_extract" => text::json_extract(inputs)?,
        "lightflow.llm_generate" => text::llm_generate(inputs)?,
        "lightflow.llm_classify" => text::llm_classify(inputs)?,
        "lightflow.llm_structured_output" => text::llm_structured_output(inputs)?,
        "lightflow.control_if" => control::control_if(inputs)?,
        "lightflow.control_switch" => control::control_switch(inputs)?,
        "lightflow.control_merge" => control::control_merge(inputs)?,
        "lightflow.control_split" => control::control_split(inputs)?,
        "lightflow.model_select" => control::model_select(inputs)?,
        "lightflow.model_lock_check" => control::model_lock_check(inputs)?,
        "lightflow.image_load"
        | "lightflow.image_save"
        | "lightflow.image_resize"
        | "lightflow.image_crop"
        | "lightflow.image_upscale"
        | "lightflow.image_invert"
        | "lightflow.mask_compose"
        | "lightflow.text_to_image"
        | "lightflow.image_edit"
        | "lightflow.image_inpaint" => image::execute(workflow_id, inputs)?,
        other => {
            return Err(RuntimeError::new(format!(
                "unsupported standard workflow: {other}"
            )));
        }
    };

    Ok(Response {
        outputs,
        artifacts,
        replay_fingerprint: Map::from_iter([
            (
                "implementation".to_owned(),
                Value::String(implementation.to_owned()),
            ),
            (
                "crate_version".to_owned(),
                Value::String(version.to_owned()),
            ),
        ]),
    })
}

/// Run one package-owned executor over the stable JSON-over-stdio protocol.
pub fn run_stdio<E>(
    workflow_id: &str,
    version: &str,
    execute: impl FnOnce(&Map<String, Value>) -> Result<Response, E>,
) -> Result<(), Box<dyn Error>>
where
    E: Error + 'static,
{
    let request = lightflow::runner::read_request_from_stdin()?;
    request.validate_for(workflow_id, version)?;
    let response = execute(&request.inputs)?;
    lightflow::runner::write_response_to_stdout(&response)?;
    Ok(())
}

/// Combine package-local and shared algorithm sources into one replay identity.
pub fn implementation_identity(workflow_id: &str, leaf_source_digest: u64) -> String {
    format!(
        "{workflow_id}.leaf.fnv1a64:{leaf_source_digest:016x}.runtime.fnv1a64:{RUNTIME_SOURCE_DIGEST:016x}"
    )
}

/// Declare the public execution entrypoint owned by one standard workflow crate.
#[macro_export]
macro_rules! standard_runner {
    ($workflow_id:literal) => {
        pub const WORKFLOW_ID: &str = $workflow_id;
        pub const WORKFLOW_VERSION: &str = env!("CARGO_PKG_VERSION");

        /// Execute this workflow's package-owned implementation.
        pub fn execute(
            inputs: &lightflow::serde_json::Map<String, lightflow::serde_json::Value>,
        ) -> Result<lightflow::runner::Response, $crate::RuntimeError> {
            $crate::execute(
                WORKFLOW_ID,
                WORKFLOW_VERSION,
                implementation_identity().as_str(),
                inputs,
            )
        }

        /// Stable identity used to make replay provenance explicit.
        pub fn implementation_identity() -> String {
            const LEAF_SOURCE_DIGEST: u64 =
                $crate::hash_source(0xcbf2_9ce4_8422_2325, include_bytes!("lib.rs"));
            $crate::implementation_identity(WORKFLOW_ID, LEAF_SOURCE_DIGEST)
        }
    };
}

#[doc(hidden)]
pub const fn hash_source(mut digest: u64, source: &[u8]) -> u64 {
    let mut index = 0;
    while index < source.len() {
        digest ^= source[index] as u64;
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    digest
}

/// Define a small binary that exposes a workflow crate's `execute` entrypoint.
#[macro_export]
macro_rules! standard_runner_binary {
    ($workflow_crate:ident) => {
        fn main() -> std::process::ExitCode {
            use $workflow_crate as workflow_crate;
            match $crate::run_stdio(
                workflow_crate::WORKFLOW_ID,
                workflow_crate::WORKFLOW_VERSION,
                workflow_crate::execute,
            ) {
                Ok(()) => std::process::ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("package workflow runner: {error}");
                    std::process::ExitCode::FAILURE
                }
            }
        }
    };
}

/// Error returned by a standard package implementation.
#[derive(Debug)]
pub struct RuntimeError(String);

impl RuntimeError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implementation_identity_is_stable_and_includes_both_source_layers() {
        let identity = implementation_identity("lightflow.example", 42);
        assert_eq!(identity, implementation_identity("lightflow.example", 42));
        assert!(identity.contains(".leaf.fnv1a64:000000000000002a"));
        assert!(identity.contains(".runtime.fnv1a64:"));
        assert_ne!(
            identity,
            implementation_identity("lightflow.example", 43),
            "leaf source changes must alter replay identity"
        );
    }
}
