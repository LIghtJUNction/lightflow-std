use lightflow::runner::{read_request_from_stdin, write_response_to_stdout};
use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("text regex runner: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let request = read_request_from_stdin()?;
    request.validate_for(
        lightflow_text_regex::WORKFLOW_ID,
        lightflow_text_regex::WORKFLOW_VERSION,
    )?;
    let response = lightflow_text_regex::execute(&request.inputs)?;
    write_response_to_stdout(&response)?;
    Ok(())
}
