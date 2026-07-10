use crate::{build_handoff, HandoffError, ImplementationHandoff};
use rumble_canvas_domain::SpecWorkspace;
use rumble_canvas_package::SpecPackage;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WrenchError {
    #[error("wrench-inspect not found in PATH")]
    NotFound,
    #[error("wrench check failed: {0}")]
    CheckFailed(String),
    #[error("wrench output parsing failed: {0}")]
    ParseError(String),
    #[error("handoff build failed: {0}")]
    HandoffBuild(String),
}

/// Mirror of the `wrench-inspect handoff inspect --json` report
/// (wrench-inspect `handoff::Report`). Fields we do not consume
/// (`coverage`) are ignored on deserialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrenchReport {
    pub valid: bool,
    pub summary: WrenchSummary,
    #[serde(default)]
    pub findings: Vec<WrenchFinding>,
    #[serde(default)]
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrenchSummary {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrenchFinding {
    /// snake_case severity as emitted by wrench-inspect: "error" | "warning" | "info".
    pub severity: String,
    pub code: String,
    pub path: String,
    pub message: String,
    pub recommendation: String,
}

/// Run wrench-inspect completeness checks on a package by building the
/// `canvas.bolt_handoff.v0.1` planning_request document and inspecting it.
///
/// wrench-inspect only understands `kind: planning_request` handoff
/// documents, so the raw SpecPackage is never sent directly.
pub fn check_package_completeness(
    workspace: &SpecWorkspace,
    package: &SpecPackage,
) -> Result<WrenchReport, WrenchError> {
    let handoff = build_handoff(workspace, package)
        .map_err(|e: HandoffError| WrenchError::HandoffBuild(e.to_string()))?;
    check_handoff_completeness(&handoff)
}

/// Inspect an already-built handoff document with
/// `wrench-inspect handoff inspect --json <file>`.
pub fn check_handoff_completeness(
    handoff: &ImplementationHandoff,
) -> Result<WrenchReport, WrenchError> {
    let file = tempfile::NamedTempFile::new()
        .map_err(|e| WrenchError::CheckFailed(format!("temp file: {e}")))?;
    serde_json::to_writer(&file, handoff).map_err(|e| WrenchError::ParseError(e.to_string()))?;

    let output = Command::new("wrench-inspect")
        .args(["handoff", "inspect", "--json"])
        .arg(file.path())
        .output()
        .map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                WrenchError::NotFound
            } else {
                WrenchError::CheckFailed(e.to_string())
            }
        })?;

    // The CLI prints the JSON report on stdout, then exits 1 when the
    // handoff is invalid — an invalid handoff is a report, not an error.
    if output.stdout.is_empty() {
        return Err(WrenchError::CheckFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|e| WrenchError::ParseError(e.to_string()))
}

/// Summarize a wrench report: overall pass/fail plus one message per
/// non-info finding.
pub fn summarize_report(report: &WrenchReport) -> (bool, Vec<String>) {
    let messages = report
        .findings
        .iter()
        .filter(|f| f.severity != "info")
        .map(|f| {
            format!(
                "{} {} at {}: {} ({})",
                f.severity.to_uppercase(),
                f.code,
                f.path,
                f.message,
                f.recommendation
            )
        })
        .collect();
    (report.valid, messages)
}
