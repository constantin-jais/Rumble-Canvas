use crate::SpecPackage;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SchemaError {
    #[error("required field missing: {0}")]
    MissingField(String),
    #[error("invalid field value: {0}")]
    InvalidValue(String),
    #[error("schema validation failed: {0}")]
    Validation(String),
}

/// Validate a SpecPackage against the canonical schema.
pub fn validate_package(package: &SpecPackage) -> Result<(), SchemaError> {
    // Check required fields
    if package.package_id.is_empty() {
        return Err(SchemaError::MissingField("package_id".to_string()));
    }
    if package.workspace_id.is_empty() {
        return Err(SchemaError::MissingField("workspace_id".to_string()));
    }
    if package.items.is_empty() {
        return Err(SchemaError::MissingField(
            "items (must have ≥1 item)".to_string(),
        ));
    }

    // Check metadata
    if package.metadata.created_by.is_empty() {
        return Err(SchemaError::MissingField("metadata.created_by".to_string()));
    }
    if package.metadata.created_at.is_empty() {
        return Err(SchemaError::MissingField("metadata.created_at".to_string()));
    }

    // Check readiness details
    if package.package_readiness_details.traceability_density < 0.0
        || package.package_readiness_details.traceability_density > 1.0
    {
        return Err(SchemaError::InvalidValue(
            "traceability_density must be 0.0–1.0".to_string(),
        ));
    }

    // Check delivery target
    if package.delivery_target.handoff_format != "canvas.bolt_handoff.v0.1" {
        return Err(SchemaError::InvalidValue(
            "handoff_format must be \"canvas.bolt_handoff.v0.1\"".to_string(),
        ));
    }

    Ok(())
}
