use rumble_canvas_domain::{SpecWorkspace, SAMPLE_TS};
use rumble_canvas_package::SpecPackage;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub mod token_sealer;
pub use token_sealer::{seal_workspace_identity_token, verify_workspace_authorization};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HandoffError {
    #[error("execution is forbidden from Canvas MVP")]
    ExecutionForbidden,
    #[error("traceability is required before handoff")]
    MissingTraceability,
    #[error("blocking question requires an accepted waiver")]
    BlockingQuestionWithoutWaiver,
    #[error("high risk requires an accepted waiver")]
    HighRiskWithoutWaiver,
    #[error("waiver expired")]
    ExpiredWaiver,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub planning_only: bool,
    pub allow_execution: bool,
    pub requires_human_approval_for_execution: bool,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            planning_only: true,
            allow_execution: false,
            requires_human_approval_for_execution: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplementationHandoff {
    pub format: String,
    pub kind: String,
    pub bolt_target: String,
    pub source: Value,
    pub package: Value,
    pub planning_scope: Value,
    pub spec_context: Value,
    pub traceability_links: Value,
    pub active_waivers: Value,
    pub open_questions: Value,
    pub risks: Value,
    pub capability_candidates: Value,
    pub constraints: Value,
    pub requested_outputs: Vec<String>,
    pub execution_policy: ExecutionPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<String>,
}

impl ImplementationHandoff {
    pub fn canvas_warnings(&self) -> Vec<&'static str> {
        self.capability_candidates
            .as_array()
            .into_iter()
            .flatten()
            .filter(|candidate| {
                candidate
                    .get("proposed_owner_layer")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .unwrap_or_default()
                    .is_empty()
            })
            .map(|_| "capability_owner_missing")
            .collect()
    }

    pub fn validate_canvas_rules(&self) -> Result<(), HandoffError> {
        if !self.execution_policy.planning_only
            || self.execution_policy.allow_execution
            || !self.execution_policy.requires_human_approval_for_execution
        {
            return Err(HandoffError::ExecutionForbidden);
        }
        if self
            .traceability_links
            .as_array()
            .map(Vec::is_empty)
            .unwrap_or(true)
        {
            return Err(HandoffError::MissingTraceability);
        }
        if has_expired_waiver(&self.active_waivers) {
            return Err(HandoffError::ExpiredWaiver);
        }
        let has_waiver = has_accepted_waiver(&self.active_waivers);
        if has_blocking_question(&self.open_questions) && !has_waiver {
            return Err(HandoffError::BlockingQuestionWithoutWaiver);
        }
        if has_high_risk(&self.risks) && !has_waiver {
            return Err(HandoffError::HighRiskWithoutWaiver);
        }
        Ok(())
    }

    pub fn with_execution_policy(mut self, execution_policy: ExecutionPolicy) -> Self {
        self.execution_policy = execution_policy;
        self.payload_hash = Some(payload_hash(&self));
        self
    }
}

pub fn build_handoff(
    workspace: &SpecWorkspace,
    package: &SpecPackage,
) -> Result<ImplementationHandoff, HandoffError> {
    let mut handoff = ImplementationHandoff {
        format: "canvas.bolt_handoff.v0.1".to_string(),
        kind: "planning_request".to_string(),
        bolt_target: "cos-matic".to_string(),
        source: json!({
            "product": "rumble-canvas",
            "workspace_id": workspace.id,
            "handoff_id": "handoff:rumble-canvas-mvp:0.1.0",
            "created_by": workspace.owner.actor_id,
            "created_at": SAMPLE_TS
        }),
        package: json!({
            "package_id": package.package_id,
            "version": package.version,
            "package_hash": package.package_hash,
            "artifact_reference_id": package.artifact_reference_id,
            "items": package.items
        }),
        planning_scope: json!({
            "mode": "full_package",
            "target_objects": [],
            "excluded_objects": [],
            "goal": "Produce a planning-only implementation plan for the Canvas package/handoff MVP"
        }),
        spec_context: json!({
            "charter_summary": workspace.charter,
            "roles": workspace.roles,
            "journeys": workspace.journeys,
            "screens": workspace.screens,
            "actions": workspace.actions,
            "domain_entities": ["SpecWorkspace", "SpecSection", "SpecSectionRevision", "TraceabilityLink", "Waiver", "SpecPackage", "SpecPackageItem", "PackageReadinessSnapshot", "ImplementationHandoff", "ActorReference"],
            "acceptance_criteria": workspace.acceptance_criteria
        }),
        traceability_links: json!(workspace.traceability_links),
        active_waivers: json!(workspace.waivers),
        open_questions: json!(workspace.open_questions),
        risks: json!(workspace.risks),
        capability_candidates: json!(workspace.capability_candidates),
        constraints: json!({
            "sovereignty": "self-hostable; no hidden external dependency",
            "data_residency": "EU/local-first where applicable",
            "non_goals": ["No implementation execution from Canvas", "No provider AI dependency"]
        }),
        requested_outputs: vec![
            "implementation_plan".to_string(),
            "task_breakdown".to_string(),
            "risk_review".to_string(),
            "test_plan".to_string(),
            "shared_capability_extraction_review".to_string(),
        ],
        execution_policy: ExecutionPolicy::default(),
        payload_hash: None,
    };
    handoff.validate_canvas_rules()?;
    handoff.payload_hash = Some(payload_hash(&handoff));
    Ok(handoff)
}

pub fn payload_hash(handoff: &ImplementationHandoff) -> String {
    let mut clone = handoff.clone();
    clone.payload_hash = None;
    let bytes = serde_json::to_vec(&clone).expect("handoff serializes");
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn has_accepted_waiver(waivers: &Value) -> bool {
    waivers.as_array().into_iter().flatten().any(|w| {
        matches!(
            w.get("status").and_then(Value::as_str),
            Some("accepted") | Some("active")
        )
    })
}

fn has_expired_waiver(waivers: &Value) -> bool {
    waivers.as_array().into_iter().flatten().any(|w| {
        w.get("expires_at")
            .and_then(Value::as_str)
            .is_some_and(|ts| ts < SAMPLE_TS)
    })
}

fn has_blocking_question(open_questions: &Value) -> bool {
    open_questions.as_array().into_iter().flatten().any(|q| {
        q.get("impact").and_then(Value::as_str) == Some("blocking")
            && q.get("status").and_then(Value::as_str) == Some("open")
    })
}

fn has_high_risk(risks: &Value) -> bool {
    risks.as_array().into_iter().flatten().any(|r| {
        matches!(
            r.get("severity").and_then(Value::as_str),
            Some("high") | Some("critical") | Some("blocking")
        ) && r.get("status").and_then(Value::as_str) == Some("open")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumble_canvas_domain::{sample_workspace, OpenQuestion, RiskFlag, Waiver};
    use rumble_canvas_package::build_package;

    fn handoff_for(mut workspace: SpecWorkspace) -> Result<ImplementationHandoff, HandoffError> {
        let package = build_package(&workspace).unwrap_or_else(|_| {
            workspace.traceability_links = sample_workspace().traceability_links;
            build_package(&workspace).unwrap()
        });
        build_handoff(&workspace, &package)
    }

    #[test]
    fn generates_valid_handoff() {
        let workspace = sample_workspace();
        let package = build_package(&workspace).unwrap();
        let handoff = build_handoff(&workspace, &package).unwrap();
        assert_eq!(handoff.format, "canvas.bolt_handoff.v0.1");
        assert!(!handoff.execution_policy.allow_execution);
        assert!(handoff.payload_hash.unwrap().starts_with("sha256:"));
    }

    #[test]
    fn refuses_allow_execution_true() {
        let workspace = sample_workspace();
        let package = build_package(&workspace).unwrap();
        let handoff = build_handoff(&workspace, &package)
            .unwrap()
            .with_execution_policy(ExecutionPolicy {
                allow_execution: true,
                ..ExecutionPolicy::default()
            });
        assert_eq!(
            handoff.validate_canvas_rules().unwrap_err(),
            HandoffError::ExecutionForbidden
        );
    }

    #[test]
    fn refuses_missing_traceability() {
        let mut workspace = sample_workspace();
        workspace.traceability_links.clear();
        let package = build_package(&sample_workspace()).unwrap();
        assert_eq!(
            build_handoff(&workspace, &package).unwrap_err(),
            HandoffError::MissingTraceability
        );
    }

    #[test]
    fn refuses_blocking_question_without_waiver() {
        let mut workspace = sample_workspace();
        workspace.open_questions.push(OpenQuestion {
            id: "q".into(),
            question: "Who approves?".into(),
            impact: "blocking".into(),
            status: "open".into(),
        });
        assert_eq!(
            handoff_for(workspace).unwrap_err(),
            HandoffError::BlockingQuestionWithoutWaiver
        );
    }

    #[test]
    fn refuses_high_risk_without_waiver() {
        let mut workspace = sample_workspace();
        workspace.risks.push(RiskFlag {
            id: "r".into(),
            category: "security".into(),
            severity: "high".into(),
            description: "Unsafe execution".into(),
            status: "open".into(),
        });
        assert_eq!(
            handoff_for(workspace).unwrap_err(),
            HandoffError::HighRiskWithoutWaiver
        );
    }

    #[test]
    fn refuses_expired_waiver() {
        let mut workspace = sample_workspace();
        workspace.waivers.push(Waiver {
            id: "w".into(),
            workspace_id: workspace.id.clone(),
            target_type: "risk_flag".into(),
            target_id: "r".into(),
            status: "accepted".into(),
            reason: "temporary".into(),
            risk_level: "low".into(),
            risk_category: "quality".into(),
            owner_id: "actor:owner".into(),
            approver_id: "actor:reviewer".into(),
            expires_at: Some("2026-06-29T00:00:00Z".into()),
            created_at: SAMPLE_TS.into(),
            decided_at: Some(SAMPLE_TS.into()),
        });
        assert_eq!(
            handoff_for(workspace).unwrap_err(),
            HandoffError::ExpiredWaiver
        );
    }

    #[test]
    fn capability_without_owner_warns_but_is_non_blocking_locally() {
        let mut workspace = sample_workspace();
        workspace.capability_candidates[0].proposed_owner_layer = None;
        let handoff = handoff_for(workspace).unwrap();
        assert_eq!(handoff.canvas_warnings(), vec!["capability_owner_missing"]);
    }
}
