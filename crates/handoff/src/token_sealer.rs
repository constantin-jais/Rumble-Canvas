use rumble_canvas_domain::{PermissionPrimitive, RoleAssignment};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenSealerError {
    #[error("sealing failed: {0}")]
    SealingFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SealedToken {
    pub token_hex: String,
    pub algorithm: String,
}

/// Seal a WorkspaceIdentity fact set into a deterministic test token.
///
/// This is a fixture sealer, not production Biscuit verification. Real Biscuit
/// mint/verify remains demand-driven by the M2 authorization increment.
pub fn seal_workspace_identity_token(
    tenant_id: &str,
    workspace_id: &str,
    role: &RoleAssignment,
    key_material: &[u8],
) -> Result<SealedToken, TokenSealerError> {
    if tenant_id.trim().is_empty() {
        return Err(TokenSealerError::SealingFailed(
            "tenant_id is required".to_string(),
        ));
    }
    if workspace_id.trim().is_empty() {
        return Err(TokenSealerError::SealingFailed(
            "workspace_id is required".to_string(),
        ));
    }
    if role.workspace_id != workspace_id {
        return Err(TokenSealerError::SealingFailed(
            "role workspace_id does not match token workspace_id".to_string(),
        ));
    }

    Ok(seal_with_mock(tenant_id, workspace_id, role, key_material))
}

fn seal_with_mock(
    tenant_id: &str,
    workspace_id: &str,
    role: &RoleAssignment,
    key_material: &[u8],
) -> SealedToken {
    // Deterministic mock: bind the token to tenant + workspace + key material,
    // then append permissions for local authorization tests.
    // Format: "scope_hash|actor_hash|permissions_list"
    let scope_hash = mock_scope_hash(tenant_id, workspace_id, key_material);
    let actor_hash = mock_actor_hash(&role.actor_ref.actor_id, &role.role);
    let perms_str = role
        .permissions
        .iter()
        .map(perm_to_string)
        .collect::<Vec<_>>()
        .join("|");

    SealedToken {
        token_hex: format!("{}|{}|{}", scope_hash, actor_hash, perms_str),
        algorithm: "mock".to_string(),
    }
}

/// Verify a deterministic fixture token and check if it grants the required permission.
///
/// The mock fails closed on tenant, workspace, or key-material mismatch before
/// checking permissions, preserving the tenant isolation invariant in tests.
pub fn verify_workspace_authorization(
    token: &SealedToken,
    tenant_id: &str,
    workspace_id: &str,
    required_permission: &PermissionPrimitive,
    key_material: &[u8],
) -> bool {
    if token.algorithm != "mock" || token.token_hex.is_empty() {
        return false;
    }

    let parts: Vec<&str> = token.token_hex.split('|').collect();
    if parts.len() < 3 {
        return false;
    }

    let expected_scope_hash = mock_scope_hash(tenant_id, workspace_id, key_material);
    if parts[0] != expected_scope_hash {
        return false;
    }

    let required_perm_str = perm_to_string(required_permission);
    parts[2..].contains(&required_perm_str)
}

fn mock_scope_hash(tenant_id: &str, workspace_id: &str, key_material: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let key_hex = key_material
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    let facts = format!("{tenant_id}|{workspace_id}|{key_hex}");
    let mut hasher = Sha256::new();
    hasher.update(facts.as_bytes());
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

fn mock_actor_hash(actor_id: &str, role: &str) -> String {
    use sha2::{Digest, Sha256};

    let facts = format!("{actor_id}|{role}");
    let mut hasher = Sha256::new();
    hasher.update(facts.as_bytes());
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

fn perm_to_string(perm: &PermissionPrimitive) -> &'static str {
    match perm {
        PermissionPrimitive::Read => "read",
        PermissionPrimitive::Comment => "comment",
        PermissionPrimitive::Write => "write",
        PermissionPrimitive::Approve => "approve",
        PermissionPrimitive::Invite => "invite",
        PermissionPrimitive::Administer => "administer",
        PermissionPrimitive::Delegate => "delegate",
    }
}
