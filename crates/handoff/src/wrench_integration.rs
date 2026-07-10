use rumble_canvas_package::SpecPackage;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WrenchError {
    #[error("wrench-inspect not found in PATH")]
    NotFound,
    #[error("wrench check failed: {0}")]
    CheckFailed(String),
    #[error("wrench output parsing failed: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrenchEvidence {
    pub check_type: String, // e.g., "contract_validation", "schema_validation", "traceability"
    pub status: String,     // "pass" | "fail" | "warn"
    pub findings: Vec<String>,
    pub checked_at: String,
}

/// Run wrench-inspect completeness checks on a package.
pub fn check_package_completeness(
    package: &SpecPackage,
) -> Result<Vec<WrenchEvidence>, WrenchError> {
    // Serialize package to JSON
    let package_json =
        serde_json::to_string(package).map_err(|e| WrenchError::ParseError(e.to_string()))?;

    // Invoke: wrench-inspect check canvas --json <stdin>
    let mut child = Command::new("wrench-inspect")
        .arg("check")
        .arg("canvas")
        .arg("--json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| WrenchError::NotFound)?;

    // Write package JSON to stdin
    {
        use std::io::Write;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| WrenchError::CheckFailed("could not open stdin".to_string()))?;
        stdin
            .write_all(package_json.as_bytes())
            .map_err(|e| WrenchError::CheckFailed(e.to_string()))?;
    }

    // Read output
    let output = child
        .wait_with_output()
        .map_err(|e| WrenchError::CheckFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(WrenchError::CheckFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Parse JSON output
    let evidence: Vec<WrenchEvidence> = serde_json::from_slice(&output.stdout)
        .map_err(|e| WrenchError::ParseError(e.to_string()))?;

    Ok(evidence)
}

/// Summarize wrench evidence: fail if any "fail" status, warn if any "warn".
pub fn summarize_evidence(evidence: &[WrenchEvidence]) -> (bool, Vec<String>) {
    let mut passed = true;
    let mut messages = vec![];

    for check in evidence {
        match check.status.as_str() {
            "fail" => {
                passed = false;
                messages.push(format!(
                    "{}: FAILED — {}",
                    check.check_type,
                    check.findings.join("; ")
                ));
            }
            "warn" => {
                messages.push(format!(
                    "{}: WARN — {}",
                    check.check_type,
                    check.findings.join("; ")
                ));
            }
            _ => {}
        }
    }

    (passed, messages)
}
