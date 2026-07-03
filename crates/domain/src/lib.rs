use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod role_assignment;
pub mod workspace_membership;

pub use role_assignment::{validate_actor_permissions, RoleAssignment, RoleValidationError};
pub use workspace_membership::{MembershipStatus, WorkspaceMembership};

pub const SAMPLE_TS: &str = "2026-06-30T00:00:00Z";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorReference {
    pub actor_id: String,
    pub actor_type: ActorType,
    pub display_name: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Human,
    Agent,
    Service,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionPrimitive {
    Read,
    Comment,
    Write,
    Approve,
    Invite,
    Administer,
    Delegate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecSectionRevision {
    pub id: String,
    pub section_id: String,
    pub revision_number: u32,
    pub content_format: String,
    pub structured_content: Value,
    pub markdown_content: Option<String>,
    pub created_by: ActorReference,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecSection {
    pub id: String,
    pub workspace_id: String,
    pub key: String,
    pub title: String,
    pub status: String,
    pub current_revision_id: String,
    pub approved_revision_id: String,
    pub required_for_package: bool,
    pub revision: SpecSectionRevision,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceabilityLink {
    pub id: String,
    pub workspace_id: String,
    pub source_type: String,
    pub source_id: String,
    pub target_type: String,
    pub target_id: String,
    pub relation_type: String,
    pub rationale: Option<String>,
    pub confidence: String,
    pub status: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Waiver {
    pub id: String,
    pub workspace_id: String,
    pub target_type: String,
    pub target_id: String,
    pub status: String,
    pub reason: String,
    pub risk_level: String,
    pub risk_category: String,
    pub owner_id: String,
    pub approver_id: String,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub decided_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductCharter {
    pub mission: String,
    pub target_users: Vec<String>,
    pub jobs_to_be_done: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleDefinition {
    pub id: String,
    pub name: String,
    pub actor_type: String,
    pub permissions: Vec<PermissionPrimitive>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JourneyDefinition {
    pub id: String,
    pub name: String,
    pub primary_actor_role_id: String,
    pub trigger: String,
    pub happy_path: Vec<String>,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenDefinition {
    pub id: String,
    pub name: String,
    pub route_or_entry: String,
    pub purpose: String,
    pub allowed_role_ids: Vec<String>,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionDefinition {
    pub id: String,
    pub screen_id: String,
    pub name: String,
    pub actor_role_id: String,
    pub intent: String,
    pub business_rules: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub audit_required: bool,
    pub destructive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityCandidate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub needed_by: Vec<String>,
    pub proposed_owner_layer: Option<String>,
    pub status: String,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskFlag {
    pub id: String,
    pub category: String,
    pub severity: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenQuestion {
    pub id: String,
    pub question: String,
    pub impact: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecWorkspace {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub status: String,
    pub owner: ActorReference,
    pub created_at: String,
    pub updated_at: String,
    pub charter: ProductCharter,
    pub roles: Vec<RoleDefinition>,
    pub journeys: Vec<JourneyDefinition>,
    pub screens: Vec<ScreenDefinition>,
    pub actions: Vec<ActionDefinition>,
    pub acceptance_criteria: Vec<String>,
    pub traceability_links: Vec<TraceabilityLink>,
    pub waivers: Vec<Waiver>,
    pub open_questions: Vec<OpenQuestion>,
    pub risks: Vec<RiskFlag>,
    pub capability_candidates: Vec<CapabilityCandidate>,
    pub memberships: Vec<WorkspaceMembership>,
    pub sections: Vec<SpecSection>,
}

pub fn sample_actor() -> ActorReference {
    ActorReference {
        actor_id: "actor:owner".to_string(),
        actor_type: ActorType::Human,
        display_name: Some("Canvas Owner".to_string()),
        source: Some("local_profile".to_string()),
    }
}

pub fn sample_workspace() -> SpecWorkspace {
    let actor = sample_actor();
    let charter = ProductCharter {
        mission: "Produce a planning-only implementation handoff from a structured spec package"
            .to_string(),
        target_users: vec!["Product builder".to_string(), "Tech lead".to_string()],
        jobs_to_be_done: vec!["Validate product intent before agentic implementation".to_string()],
        non_goals: vec!["Execute implementation work from Canvas".to_string()],
    };
    let role = RoleDefinition {
        id: "role:owner".to_string(),
        name: "Owner".to_string(),
        actor_type: "human".to_string(),
        permissions: vec![
            PermissionPrimitive::Read,
            PermissionPrimitive::Write,
            PermissionPrimitive::Approve,
        ],
    };
    let journey = JourneyDefinition {
        id: "journey:package-to-handoff".to_string(),
        name: "Approve package and request Bolt planning".to_string(),
        primary_actor_role_id: role.id.clone(),
        trigger: "A spec slice is ready for implementation planning".to_string(),
        happy_path: vec![
            "Approve immutable package".to_string(),
            "Generate handoff".to_string(),
            "Validate via cosmatic".to_string(),
        ],
        acceptance_criteria: vec!["Validation succeeds without allowing execution".to_string()],
    };
    let screen = ScreenDefinition {
        id: "screen:handoff".to_string(),
        name: "Handoff Review".to_string(),
        route_or_entry: "/handoff".to_string(),
        purpose: "Review package readiness and produce a planning-only handoff".to_string(),
        allowed_role_ids: vec![role.id.clone()],
        acceptance_criteria: vec!["Findings and dry-run plan are visible to the owner".to_string()],
    };
    let action = ActionDefinition {
        id: "action:validate-handoff".to_string(),
        screen_id: screen.id.clone(),
        name: "Validate handoff".to_string(),
        actor_role_id: role.id.clone(),
        intent: "Ask Bolt to validate the handoff payload without execution".to_string(),
        business_rules: vec!["execution_policy.allow_execution must be false".to_string()],
        acceptance_criteria: vec!["cosmatic handoff validate --json returns no errors".to_string()],
        audit_required: true,
        destructive: false,
    };
    let criteria = vec![
        "Given a valid package, handoff validation succeeds".to_string(),
        "Given allow_execution true, handoff validation fails".to_string(),
    ];
    let memberships = vec![
        WorkspaceMembership {
            id: "member:owner".to_string(),
            workspace_id: "workspace:rumble-canvas-mvp".to_string(),
            actor_ref: actor.clone(),
            status: MembershipStatus::Active,
            joined_at: SAMPLE_TS.to_string(),
            revoked_at: None,
        },
        WorkspaceMembership {
            id: "member:contributor".to_string(),
            workspace_id: "workspace:rumble-canvas-mvp".to_string(),
            actor_ref: ActorReference {
                actor_id: "actor:contributor".to_string(),
                actor_type: ActorType::Human,
                display_name: Some("Contributor Alice".to_string()),
                source: Some("local_profile".to_string()),
            },
            status: MembershipStatus::Active,
            joined_at: SAMPLE_TS.to_string(),
            revoked_at: None,
        },
    ];
    let sections = vec![section(
        "section:charter",
        "product-charter",
        "Product Charter",
        json!(charter),
        &actor,
    )];
    SpecWorkspace {
        id: "workspace:rumble-canvas-mvp".to_string(),
        name: "Rumble Canvas MVP".to_string(),
        slug: "rumble-canvas-mvp".to_string(),
        status: "approved".to_string(),
        owner: actor,
        created_at: SAMPLE_TS.to_string(),
        updated_at: SAMPLE_TS.to_string(),
        charter,
        roles: vec![role],
        journeys: vec![journey],
        screens: vec![screen],
        actions: vec![action],
        acceptance_criteria: criteria,
        traceability_links: vec![TraceabilityLink {
            id: "trace:journey-action".to_string(),
            workspace_id: "workspace:rumble-canvas-mvp".to_string(),
            source_type: "journey".to_string(),
            source_id: "journey:package-to-handoff".to_string(),
            target_type: "action".to_string(),
            target_id: "action:validate-handoff".to_string(),
            relation_type: "implements".to_string(),
            rationale: Some("The action implements the package-to-handoff journey".to_string()),
            confidence: "manual".to_string(),
            status: "active".to_string(),
            created_by: "actor:owner".to_string(),
            created_at: SAMPLE_TS.to_string(),
        }],
        waivers: vec![],
        open_questions: vec![],
        risks: vec![],
        capability_candidates: vec![CapabilityCandidate {
            id: "cap:implementation-handoff".to_string(),
            name: "ImplementationHandoff".to_string(),
            description: "Planning-only Rumble-to-Bolt boundary object".to_string(),
            needed_by: vec!["rumble-canvas".to_string(), "cos-matic".to_string()],
            proposed_owner_layer: Some("bolt".to_string()),
            status: "accepted".to_string(),
            rationale: "Needed to connect product packages to harness planning safely".to_string(),
        }],
        memberships,
        sections,
    }
}

fn section(
    id: &str,
    key: &str,
    title: &str,
    structured_content: Value,
    actor: &ActorReference,
) -> SpecSection {
    let revision_id = format!("revision:{key}:1");
    SpecSection {
        id: id.to_string(),
        workspace_id: "workspace:rumble-canvas-mvp".to_string(),
        key: key.to_string(),
        title: title.to_string(),
        status: "approved".to_string(),
        current_revision_id: revision_id.clone(),
        approved_revision_id: revision_id.clone(),
        required_for_package: true,
        revision: SpecSectionRevision {
            id: revision_id,
            section_id: id.to_string(),
            revision_number: 1,
            content_format: "dual".to_string(),
            structured_content,
            markdown_content: Some(format!("# {title}\n\nSample approved section.")),
            created_by: actor.clone(),
            created_at: SAMPLE_TS.to_string(),
        },
        created_at: SAMPLE_TS.to_string(),
        updated_at: SAMPLE_TS.to_string(),
    }
}
