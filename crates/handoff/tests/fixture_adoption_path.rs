use rumble_canvas_domain::{
    ActorReference, ActorType, MembershipStatus, PermissionPrimitive, RoleAssignment,
    WorkspaceMembership,
};
use rumble_canvas_handoff::token_sealer::{
    seal_workspace_identity_token, verify_workspace_authorization,
};

#[test]
fn test_d11_adoption_path_canvas_to_handoff_authorization() {
    let tenant_id = "tenant:test";
    let workspace_id = "workspace:test";
    let key_material = b"test-key-32-bytes-long-12345678"; // 32 bytes

    // Given: An owner actor with Approve permission
    let owner = ActorReference {
        actor_id: "actor:alice".to_string(),
        actor_type: ActorType::Human,
        display_name: Some("Alice Owner".to_string()),
        source: Some("local_profile".to_string()),
    };

    let _owner_membership = WorkspaceMembership {
        id: "member:alice".to_string(),
        workspace_id: workspace_id.to_string(),
        actor_ref: owner.clone(),
        status: MembershipStatus::Active,
        joined_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };

    let owner_role = RoleAssignment {
        id: "role:owner".to_string(),
        workspace_id: workspace_id.to_string(),
        actor_ref: owner,
        role: "owner".to_string(),
        permissions: vec![
            PermissionPrimitive::Read,
            PermissionPrimitive::Write,
            PermissionPrimitive::Approve,
        ],
        created_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };

    // When: Canvas seals a token for the owner
    let owner_token =
        seal_workspace_identity_token(tenant_id, workspace_id, &owner_role, key_material)
            .expect("owner token seals");

    // Then: Owner can authorize a handoff approval (has Approve permission)
    let result = verify_workspace_authorization(
        &owner_token,
        tenant_id,
        workspace_id,
        &PermissionPrimitive::Approve,
        key_material,
    );
    assert!(result, "Owner token grants Approve permission");

    // --- Reviewer (no Approve) ---
    let reviewer = ActorReference {
        actor_id: "actor:bob".to_string(),
        actor_type: ActorType::Human,
        display_name: Some("Bob Reviewer".to_string()),
        source: Some("local_profile".to_string()),
    };

    let _reviewer_membership = WorkspaceMembership {
        id: "member:bob".to_string(),
        workspace_id: workspace_id.to_string(),
        actor_ref: reviewer.clone(),
        status: MembershipStatus::Active,
        joined_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };

    let reviewer_role = RoleAssignment {
        id: "role:reviewer".to_string(),
        workspace_id: workspace_id.to_string(),
        actor_ref: reviewer,
        role: "reviewer".to_string(),
        permissions: vec![PermissionPrimitive::Read, PermissionPrimitive::Comment],
        created_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };

    // When: Canvas seals a token for the reviewer
    let reviewer_token =
        seal_workspace_identity_token(tenant_id, workspace_id, &reviewer_role, key_material)
            .expect("reviewer token seals");

    // Then: Reviewer cannot authorize a handoff approval (no Approve permission)
    let result = verify_workspace_authorization(
        &reviewer_token,
        tenant_id,
        workspace_id,
        &PermissionPrimitive::Approve,
        key_material,
    );
    assert!(!result, "Reviewer token does NOT grant Approve permission");

    // And: Reviewer CAN read (has Read permission)
    let result = verify_workspace_authorization(
        &reviewer_token,
        tenant_id,
        workspace_id,
        &PermissionPrimitive::Read,
        key_material,
    );
    assert!(result, "Reviewer token grants Read permission");

    // And: tenant/workspace/key mismatches fail closed even when permission exists.
    let wrong_tenant = verify_workspace_authorization(
        &reviewer_token,
        "tenant:other",
        workspace_id,
        &PermissionPrimitive::Read,
        key_material,
    );
    assert!(!wrong_tenant, "Token is tenant-bound");

    let wrong_workspace = verify_workspace_authorization(
        &reviewer_token,
        tenant_id,
        "workspace:other",
        &PermissionPrimitive::Read,
        key_material,
    );
    assert!(!wrong_workspace, "Token is workspace-bound");

    let wrong_key = verify_workspace_authorization(
        &reviewer_token,
        tenant_id,
        workspace_id,
        &PermissionPrimitive::Read,
        b"other-key-32-bytes-long-1234567",
    );
    assert!(!wrong_key, "Token is key-bound");

    let mismatched_role = RoleAssignment {
        workspace_id: "workspace:other".to_string(),
        ..reviewer_role
    };
    assert!(
        seal_workspace_identity_token(tenant_id, workspace_id, &mismatched_role, key_material)
            .is_err(),
        "role workspace_id must match token workspace_id"
    );
}
