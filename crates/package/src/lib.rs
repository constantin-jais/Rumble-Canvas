use rumble_canvas_domain::{SpecSection, SpecWorkspace, SAMPLE_TS};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PackageError {
    #[error("approved package is immutable")]
    ImmutableAfterApproval,
    #[error("package requires at least one traceability link")]
    MissingTraceability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecPackageItem {
    pub id: String,
    pub package_id: String,
    pub section_id: String,
    pub revision_id: String,
    pub section_key: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageReadinessSnapshot {
    pub status: String,
    pub traceability_links: usize,
    pub blocking_questions: usize,
    pub high_risks: usize,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecPackage {
    pub package_id: String,
    pub workspace_id: String,
    pub version: String,
    pub status: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub package_hash: String,
    pub artifact_reference_id: Option<String>,
    pub items: Vec<SpecPackageItem>,
    pub readiness: PackageReadinessSnapshot,
}

impl SpecPackage {
    pub fn add_item(&mut self, item: SpecPackageItem) -> Result<(), PackageError> {
        if self.status == "approved"
            || self.status == "exported"
            || self.status == "handoff_submitted"
        {
            return Err(PackageError::ImmutableAfterApproval);
        }
        self.items.push(item);
        self.package_hash = compute_package_hash(self);
        Ok(())
    }

    pub fn approve(mut self, actor_id: &str) -> Self {
        self.status = "approved".to_string();
        self.approved_by = Some(actor_id.to_string());
        self.approved_at = Some(SAMPLE_TS.to_string());
        self.package_hash = compute_package_hash(&self);
        self
    }
}

pub fn build_package(workspace: &SpecWorkspace) -> Result<SpecPackage, PackageError> {
    if workspace.traceability_links.is_empty() {
        return Err(PackageError::MissingTraceability);
    }
    let mut package = SpecPackage {
        package_id: "package:rumble-canvas-mvp:0.1.0".to_string(),
        workspace_id: workspace.id.clone(),
        version: "0.1.0".to_string(),
        status: "draft".to_string(),
        approved_by: None,
        approved_at: None,
        package_hash: String::new(),
        artifact_reference_id: Some("artifact:sample-package".to_string()),
        items: workspace.sections.iter().map(package_item).collect(),
        readiness: PackageReadinessSnapshot {
            status: "ready".to_string(),
            traceability_links: workspace.traceability_links.len(),
            blocking_questions: workspace
                .open_questions
                .iter()
                .filter(|q| q.impact == "blocking" && q.status == "open")
                .count(),
            high_risks: workspace
                .risks
                .iter()
                .filter(|r| {
                    matches!(r.severity.as_str(), "high" | "critical" | "blocking")
                        && r.status == "open"
                })
                .count(),
            created_at: SAMPLE_TS.to_string(),
        },
    };
    package.package_hash = compute_package_hash(&package);
    Ok(package.approve(&workspace.owner.actor_id))
}

fn package_item(section: &SpecSection) -> SpecPackageItem {
    SpecPackageItem {
        id: format!("pkgitem:{}", section.key),
        package_id: "package:rumble-canvas-mvp:0.1.0".to_string(),
        section_id: section.id.clone(),
        revision_id: section.approved_revision_id.clone(),
        section_key: section.key.clone(),
        required: section.required_for_package,
    }
}

pub fn compute_package_hash(package: &SpecPackage) -> String {
    #[derive(Serialize)]
    struct HashView<'a> {
        package_id: &'a str,
        workspace_id: &'a str,
        version: &'a str,
        status: &'a str,
        approved_by: &'a Option<String>,
        approved_at: &'a Option<String>,
        artifact_reference_id: &'a Option<String>,
        items: &'a [SpecPackageItem],
        readiness: &'a PackageReadinessSnapshot,
    }
    let view = HashView {
        package_id: &package.package_id,
        workspace_id: &package.workspace_id,
        version: &package.version,
        status: &package.status,
        approved_by: &package.approved_by,
        approved_at: &package.approved_at,
        artifact_reference_id: &package.artifact_reference_id,
        items: &package.items,
        readiness: &package.readiness,
    };
    let bytes = serde_json::to_vec(&view).expect("hash view serializes");
    let digest = Sha256::digest(&bytes);
    let hex: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
    format!("sha256:{}", hex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumble_canvas_domain::sample_workspace;

    #[test]
    fn package_hash_is_stable_for_identical_workspace() {
        let a = build_package(&sample_workspace()).unwrap();
        let b = build_package(&sample_workspace()).unwrap();
        assert_eq!(a.package_hash, b.package_hash);
    }

    #[test]
    fn package_requires_traceability() {
        let mut workspace = sample_workspace();
        workspace.traceability_links.clear();
        assert_eq!(
            build_package(&workspace).unwrap_err(),
            PackageError::MissingTraceability
        );
    }

    #[test]
    fn approved_package_is_immutable() {
        let mut package = build_package(&sample_workspace()).unwrap();
        let item = package.items[0].clone();
        assert_eq!(
            package.add_item(item).unwrap_err(),
            PackageError::ImmutableAfterApproval
        );
    }

    #[test]
    fn sha256_hash_format_regression_golden() {
        // Golden test: verify SHA256 digest format is stable.
        // This prevents regressions in the sha2 0.11 migration where
        // output type no longer implements LowerHex.
        // Expected output for "golden test payload" is verified via:
        //   echo -n "golden test payload" | sha256sum
        let payload = b"golden test payload";
        let digest = Sha256::digest(payload);
        let hex: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
        let result = format!("sha256:{}", hex);
        assert_eq!(
            result,
            "sha256:68f2e7eb43975fb21deeee23a698c5c2ec1d9f6989b904344c9920da42538650"
        );
    }
}
