use rumble_canvas_domain::{
    validate_actor_permissions, ActorReference, ActorType, MembershipStatus, PermissionPrimitive,
    RoleAssignment, WorkspaceMembership,
};

#[test]
fn test_workspace_membership_active_status() {
    let member = WorkspaceMembership {
        id: "member:1".to_string(),
        workspace_id: "workspace:test".to_string(),
        actor_ref: ActorReference {
            actor_id: "actor:alice".to_string(),
            actor_type: ActorType::Human,
            display_name: Some("Alice".to_string()),
            source: Some("local".to_string()),
        },
        status: MembershipStatus::Active,
        joined_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };
    assert!(member.is_active());
}

#[test]
fn test_role_assignment_empty_permissions_fails() {
    let role = RoleAssignment {
        id: "role:invalid".to_string(),
        workspace_id: "workspace:test".to_string(),
        actor_ref: ActorReference {
            actor_id: "actor:bob".to_string(),
            actor_type: ActorType::Human,
            display_name: Some("Bob".to_string()),
            source: Some("local".to_string()),
        },
        role: "viewer".to_string(),
        permissions: vec![], // INVALID: empty
        created_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };
    let result = validate_actor_permissions(&role);
    assert!(result.is_err(), "Empty permissions should fail validation");
}

#[test]
fn test_service_actor_cannot_hold_approve_permission() {
    let service = ActorReference {
        actor_id: "actor:ci-bot".to_string(),
        actor_type: ActorType::Service,
        display_name: Some("CI Bot".to_string()),
        source: Some("automation".to_string()),
    };
    let invalid_role = RoleAssignment {
        id: "role:invalid".to_string(),
        workspace_id: "workspace:test".to_string(),
        actor_ref: service,
        role: "reviewer".to_string(),
        permissions: vec![PermissionPrimitive::Approve], // INVALID for Service
        created_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };
    let result = validate_actor_permissions(&invalid_role);
    assert!(result.is_err(), "Service actors cannot hold Approve");
}

#[test]
fn test_human_actor_can_hold_all_permissions() {
    let human = ActorReference {
        actor_id: "actor:owner".to_string(),
        actor_type: ActorType::Human,
        display_name: Some("Owner".to_string()),
        source: Some("local".to_string()),
    };
    let valid_role = RoleAssignment {
        id: "role:owner".to_string(),
        workspace_id: "workspace:test".to_string(),
        actor_ref: human,
        role: "owner".to_string(),
        permissions: vec![
            PermissionPrimitive::Read,
            PermissionPrimitive::Write,
            PermissionPrimitive::Approve,
            PermissionPrimitive::Delegate,
        ],
        created_at: "2026-07-03T00:00:00Z".to_string(),
        revoked_at: None,
    };
    assert!(
        validate_actor_permissions(&valid_role).is_ok(),
        "Humans can hold all permissions"
    );
}

#[test]
fn test_actor_type_service_serializes_as_snake_case() {
    let actor = ActorReference {
        actor_id: "actor:test".to_string(),
        actor_type: ActorType::Service,
        display_name: None,
        source: None,
    };
    let json = serde_json::to_string(&actor).expect("serializes");
    assert!(
        json.contains("\"actor_type\":\"service\""),
        "Service serializes as lowercase 'service'"
    );
}
