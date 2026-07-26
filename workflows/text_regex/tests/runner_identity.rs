use lightflow::runner::{PROTOCOL, Request, WorkflowIdentity};
use lightflow::serde_json::{self, Map};
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn runner_rejects_forged_workflow_id_and_version() {
    for (id, version, expected) in [
        ("lightflow.forged", "0.1.0", "expected workflow id"),
        ("lightflow.text_regex", "9.9.9", "expected workflow version"),
    ] {
        let request = Request {
            protocol: PROTOCOL.to_owned(),
            workflow: WorkflowIdentity {
                id: id.to_owned(),
                version: version.to_owned(),
            },
            inputs: Map::new(),
            models: Default::default(),
        };
        let mut child = Command::new(env!("CARGO_BIN_EXE_lightflow-text-regex-runner"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("runner");
        serde_json::to_writer(child.stdin.as_mut().expect("stdin"), &request)
            .expect("write request");
        child
            .stdin
            .take()
            .expect("close stdin")
            .flush()
            .expect("flush");
        let output = child.wait_with_output().expect("runner output");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(expected), "unexpected stderr: {stderr}");
    }
}
