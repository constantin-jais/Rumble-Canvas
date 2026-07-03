use super::{ActorReference, ActorType, PermissionPrimitive};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub id: String,
    pub workspace_id: String,
    pub actor_ref: ActorReference,
    pub role: String, // product-named bundle (e.g., "owner", "editor")
    pub permissions: Vec<PermissionPrimitive>,
    pub created_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Debug, Error)]
pub enum RoleValidationError {
    #[error("actor type {0} cannot hold approval permission")]
    ApprovalForbiddenForActorType(String),
    #[error("permissions cannot be empty")]
    EmptyPermissions,
}

/// Validate that an actor type can hold the assigned permissions.
pub fn validate_actor_permissions(role: &RoleAssignment) -> Result<(), RoleValidationError> {
    if role.permissions.is_empty() {
        return Err(RoleValidationError::EmptyPermissions);
    }

    // Agent, Service, External actors cannot hold Approve or Delegate
    if matches!(
        role.actor_ref.actor_type,
        ActorType::Agent | ActorType::Service | ActorType::External
    ) {
        for perm in &role.permissions {
            if matches!(
                perm,
                PermissionPrimitive::Approve | PermissionPrimitive::Delegate
            ) {
                return Err(RoleValidationError::ApprovalForbiddenForActorType(format!(
                    "{:?}",
                    role.actor_ref.actor_type
                )));
            }
        }
    }

    Ok(())
}
