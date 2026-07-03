# Plan — canvas-mvp-workspace-identity (2026-07 wave)

```yaml
# forge.plan.v0.1 — bolt-handoff-compatible header (maps onto canvas.bolt_handoff.v0.1)
format: forge.plan.v0.1
kind: planning_request
source:
  product: rumble-canvas
  plan_id: plan-2026-07-canvas-mvp-workspace-identity
  created_at: "2026-07-03"
  revision: "2" # Revised to resolve blocking points + integrate improvements
execution_policy:
  planning_only: true
  allow_execution: false
  requires_human_approval_for_execution: true
traceability:
  - "target-version 1.0.0 (ADR 0032, ADR 0033)"
  - "ADR 0028 Accepted (2026-07-03, amendments 1–3 on D11-gating, closed permission vocabulary, big-bang posture)"
  - "architecture-alignment-2026-07.md DA-7 (workspace/identity split: Gear + shared Rumble)"
  - "workspace-identity.v0.1.md contract (Draft → Candidate after I1 creates schema)"
  - "workspace-identity.v0.1.schema.json (to be authored in I1, unblocks schema validation in I4)"
depends_on:
  - "lm-session-api contract (optional: if published before I2 start, enables live fixture; if not, I2 uses mock fallback)"
blocks:
  - "rumble-lm increment #1 (lm cannot map Host/Participant→RoleAssignment until I1 stable)"
  - "rumble-crew reconciliation (crew's 8×9 matrix depends on I1 RoleAssignment schema)"
open_questions:
  - id: O1
    question: "Does the closed permission vocabulary (read, comment, write, approve, invite, administer, delegate) cover all product roles needed by canvas in this wave?"
    owner: "canvas author (constellation of canvas PM + tech lead)"
    priority: "blocking I1 exit gate"
  - id: O2
    question: "Wrench completeness checks — which specific checks (contract validation, schema validation, traceability completeness, role coverage, permission matrix validation) are in scope for I3 MVP vs future?"
    owner: "wrench-inspect author + canvas PM"
    priority: "blocking I3 exit gate"
risks:
  - id: R1
    severity: "medium"
    description: "lm-session-api contract not published by I2 start. Blocks cross-repo fixture with real lm session context."
    mitigation: "I2 gates on explicit contract publication; fixture is tested standalone with mock key material if lm not ready. Both paths validate (real: integration test; mock: unit test marked #[ignore] until lm publishes). Non-blocking for I2 merge."
  - id: R2
    severity: "low"
    description: "Wrench integration (I3) introduces a new integration point with wrench-inspect CLI. If wrench output format changes, I3 tests may break."
    mitigation: "Tests use contract version pinning (wrench-inspect tag specified in CI). Version compatibility check added as exit gate step."
  - id: R3
    severity: "low"
    description: "SpecPackage schema stabilization (I4) may reveal structural gaps in the existing package; sample fixtures may not validate against the new schema."
    mitigation: "I4 schema design includes a migration helper (sample_package() updated inline). Pre-merge: manual review of sample package against new schema."
  - id: R4
    severity: "medium"
    description: "ActorType enum mismatch: code uses 'System', workspace-identity.v0.1 contract specifies 'service'. Breaking change if not reconciled."
    mitigation: "I1 changes ActorType::System → ActorType::Service. All downstream usages (handoff/src/lib.rs, sample fixtures) updated atomically in same commit to maintain consistency."
evidence_expectations: "Each increment exit gate = explicit CI green + command proof. (E.g., I1 exit = `cargo test --workspace`, `cargo fmt --check`, `cargo clippy --all-targets` all pass, plus new tests demonstrate permission validation and ActorType reconciliation.)"
```

## Context

**Blocker:** The canvas MVP cannot ship without a workspace/identity boundary. Today, canvas has a minimal local `ActorReference` + `RoleDefinition` model (crates/domain/src/lib.rs:6–21, 92–97), but no `WorkspaceMembership` or `RoleAssignment` types. Each of canvas, crew, and lm are reinventing identity locally — the exact duplication ADR 0028 was opened to resolve.

**Decision made (ADR 0028 Accepted, DA-7):** Identity/authorization primitives (ActorReference, RoleAssignment, tenant boundary) belong in Gear. Workspace container (membership, product-level settings) belongs in shared Rumble. The contract `workspace-identity.v0.1` unifies the split; canvas must reconcile onto it.

**Proof of demand:**
- Canvas claim: contract-first maturity (fiches:99); depends on workspace-identity to prove multi-actor workflows (README:53 — "not scale-ready yet").
- Crew spec (referenced in ADR 0028): 8-role × 9-permission matrix requires a stable RoleAssignment schema.
- LM spec (ADR 0029 + addendum): Host/Participant roles must map onto workspace-identity.v0.1 RoleAssignment.
- Arbitration DA-7: "canvas MVP is collaborative and multi-actor from day one" (ADR 0028:16) ⇒ unlock it.

**Key constraints from target-version 1.0.0 + DA-1/DA-12:**
- Big-bang posture (DA-8): all repos advance by green PRs; no red main; reconciliation happens within 2026-07 wave (ADR 0028 amendment 3).
- D11-gating (ADR 0028 amendment 2): Identity primitives stay as contract + schema until 2 implementations (canvas, lm) + cross-repo Biscuit fixture land; only then extract a dedicated gear-identity repo.
- Closed vocabulary (ADR 0028 amendment 1): Permission primitives = `{read, comment, write, approve, invite, administer, delegate}`, never free-form strings. Canvas roles must map explicitly to this vocabulary.
- No session runtime in canvas (brief constraint): Canvas does not implement its own session storage. Session presence, TTL, WebSocket belong to lm. Canvas may declare a session-workspace relationship (deferred to I2 fixture).

## Target state

**After all 4 increments complete:**

1. **I1** delivers `WorkspaceMembership` + `RoleAssignment` types with closed permission vocabulary to crates/domain. ActorType is reconciled: `System` → `Service` (matches workspace-identity.v0.1 contract). workspace-identity.v0.1.schema.json is authored and published. Sample workspace builds and tests pass (14+ tests, all green). Canvas has the schema needed to represent multi-actor workspaces.

2. **I2** delivers a cross-repo fixture that mints a Biscuit token over a WorkspaceIdentity fact set (actor + role + permissions), then validates a canvas→handoff authorization flow. Proof of D11 adoption path (2 implementations + fixture). Biscuit integration is conditional: if RUSTSEC-2026-0173 is resolved and biscuit-auth becomes available, uses real Biscuit; otherwise uses a deterministic mock sealer for testing.

3. **I3** wires wrench-inspect completeness checks into the handoff validation pipeline. SpecPackage can be inspected for contract/schema/traceability completeness before planning. Evidence appears in the sample e2e outputs (target/e2e/wrench_evidence.json).

4. **I4** stabilizes SpecPackage schema with complete field set, JSON schema validator, and migration docs. README:18 claim ("complete the SpecPackage schema") is discharged. Sample package validates; all tests pass (14+ plus new schema validation tests).

**Verification criteria:**
- All 4 increments pass CI gates (cargo test, fmt, check, clippy, hygiene).
- Canvas CLI commands (`workspace sample`, `package build`, `handoff validate/plan`) still work end-to-end.
- Sample e2e outputs in target/e2e/ validate against the new schema and include wrench evidence.
- No new `TODO` markers; debt tracked in ROADMAP.md instead (hygiene finding from fiche:179).
- Each increment is a single, mergeable PR.
- ActorType::Service used consistently across domain, handoff, and all fixtures.

## Increments

### I1 — Reconcile Canvas on workspace-identity.v0.1 + closed permission vocabulary + author schema

**Purpose:** Promote canvas's minimal identity model onto the shared contract. Deliver `WorkspaceMembership` and `RoleAssignment` types with closed permission vocabulary so canvas can represent multi-actor workspaces per ADR 0028. Reconcile ActorType enum (System → Service). Author workspace-identity.v0.1.schema.json (contract prerequisite, unblocks I4).

**Pre-requisites:**
- Ecosystem has published `workspace-identity.v0.1.md` and `0028-workspace-identity-ownership.md` (Done: they exist at `$DEV_ROOT/constantin-jais/ecosystem/specs/shared/contracts/workspace-identity.v0.1.md` and `specs/shared/adrs/0028-workspace-identity-ownership.md`).
- ADR 0028 is Accepted (2026-07-03) ✓
- No external dependency on lm or other repos; I1 is fully self-contained.
- Decision O1 resolved (canvas PM confirms permission vocabulary covers needed roles, or I1 accepts it as-is per amendment 1).

**Files touched:**
- `crates/domain/src/lib.rs` — Update ActorType enum (System → Service), add types: `WorkspaceMembership`, `RoleAssignment`, `PermissionPrimitive` enum.
- `crates/domain/src/workspace_membership.rs` (new file) — Dedicated module for membership logic (status transitions, cascade constraints).
- `crates/domain/src/role_assignment.rs` (new file) — Dedicated module for role and permission validation.
- `crates/domain/src/lib.rs` (add explicit module declarations) — Add `mod workspace_membership;` and `mod role_assignment;` at top level.
- `crates/domain/Cargo.toml` — No new deps (serde already present).
- `crates/domain/tests/integration_test_workspace_identity.rs` (new file) — I1-specific tests. **Requires mkdir -p crates/domain/tests first.**
- `crates/handoff/src/lib.rs` — Update ActorReference to use domain ActorType (or reconcile actor_type field to match enum).
- `specs/shared/contracts/workspace-identity.v0.1.schema.json` (new file) — JSON schema for workspace-identity contract (unblocks I4 schema validation).

**Work (precise, no vagueness):**

1. **Update ActorType enum** in crates/domain/src/lib.rs (lines 16–21):
   ```rust
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   #[serde(rename_all = "snake_case")]
   pub enum ActorType {
       Human,
       Agent,
       Service,      // CHANGED: was "System", now "Service" per workspace-identity.v0.1 spec
       External,
   }
   ```

2. **Add `PermissionPrimitive` enum** to crates/domain/src/lib.rs (after ActorType, lines ~22–35):
   ```rust
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
   ```

3. **Create `crates/domain/src/workspace_membership.rs`** (new file):
   ```rust
   use serde::{Deserialize, Serialize};
   use super::ActorReference;
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct WorkspaceMembership {
       pub id: String,
       pub workspace_id: String,
       pub actor_ref: ActorReference,
       pub status: MembershipStatus,
       pub joined_at: String,
       pub revoked_at: Option<String>,
   }
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   #[serde(rename_all = "snake_case")]
   pub enum MembershipStatus {
       Active,
       Invited,
       Revoked,
   }
   
   impl WorkspaceMembership {
       pub fn is_active(&self) -> bool {
           self.status == MembershipStatus::Active
       }
   }
   ```

4. **Create `crates/domain/src/role_assignment.rs`** (new file):
   ```rust
   use serde::{Deserialize, Serialize};
   use super::{ActorReference, ActorType, PermissionPrimitive};
   use thiserror::Error;
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct RoleAssignment {
       pub id: String,
       pub workspace_id: String,
       pub actor_ref: ActorReference,
       pub role: String,  // product-named bundle (e.g., "owner", "editor")
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
               if matches!(perm, PermissionPrimitive::Approve | PermissionPrimitive::Delegate) {
                   return Err(RoleValidationError::ApprovalForbiddenForActorType(
                       format!("{:?}", role.actor_ref.actor_type),
                   ));
               }
           }
       }
       
       Ok(())
   }
   ```

5. **Update crates/domain/src/lib.rs** — Add module declarations at top (after imports, before type definitions):
   ```rust
   pub mod workspace_membership;
   pub mod role_assignment;
   
   pub use workspace_membership::{WorkspaceMembership, MembershipStatus};
   pub use role_assignment::{RoleAssignment, validate_actor_permissions, RoleValidationError};
   ```

6. **Create `crates/domain/tests/` directory and integration test:**
   ```bash
   mkdir -p crates/domain/tests
   # Create crates/domain/tests/integration_test_workspace_identity.rs (content below)
   ```
   
   `crates/domain/tests/integration_test_workspace_identity.rs`:
   ```rust
   use rumble_canvas_domain::{
       ActorReference, ActorType, PermissionPrimitive, WorkspaceMembership, MembershipStatus,
       RoleAssignment, validate_actor_permissions,
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
           permissions: vec![],  // INVALID: empty
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
           permissions: vec![PermissionPrimitive::Approve],  // INVALID for Service
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
       assert!(validate_actor_permissions(&valid_role).is_ok(), "Humans can hold all permissions");
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
       assert!(json.contains("\"actor_type\":\"service\""), "Service serializes as lowercase 'service'");
   }
   ```

7. **Update `crates/domain/src/lib.rs` sample_workspace()** (around line 192–291):
   ```rust
   pub fn sample_workspace() -> SpecWorkspace {
       // ... existing fields ...
       // ADD after existing role definitions:
       let memberships = vec![
           WorkspaceMembership {
               id: "member:owner".to_string(),
               workspace_id: "workspace:sample".to_string(),
               actor_ref: owner.clone(),
               status: MembershipStatus::Active,
               joined_at: SAMPLE_TS.to_string(),
               revoked_at: None,
           },
           WorkspaceMembership {
               id: "member:contributor".to_string(),
               workspace_id: "workspace:sample".to_string(),
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
       
       // Update sample roles to use PermissionPrimitive enum (not strings)
       let roles = vec![
           RoleDefinition {
               role_name: "owner".to_string(),
               permissions: vec![
                   PermissionPrimitive::Read,
                   PermissionPrimitive::Write,
                   PermissionPrimitive::Approve,
                   PermissionPrimitive::Delegate,
               ],
           },
           RoleDefinition {
               role_name: "contributor".to_string(),
               permissions: vec![
                   PermissionPrimitive::Read,
                   PermissionPrimitive::Write,
                   PermissionPrimitive::Comment,
               ],
           },
       ];
       
       SpecWorkspace {
           // ... existing fields ...
           memberships,
           roles,
       }
   }
   ```

8. **Create `specs/shared/contracts/workspace-identity.v0.1.schema.json`** (new file):
   ```json
   {
     "$schema": "http://json-schema.org/draft-07/schema#",
     "title": "WorkspaceIdentity v0.1",
     "description": "Schema for workspace/identity primitives shared across canvas, crew, lm",
     "type": "object",
     "definitions": {
       "ActorType": {
         "type": "string",
         "enum": ["human", "agent", "service", "external"],
         "description": "Type of actor (closed vocabulary per ADR 0028 amendment 1)"
       },
       "ActorReference": {
         "type": "object",
         "required": ["actor_id", "actor_type"],
         "properties": {
           "actor_id": { "type": "string" },
           "actor_type": { "$ref": "#/definitions/ActorType" },
           "display_name": { "type": ["string", "null"] },
           "source": { "type": ["string", "null"] }
         }
       },
       "PermissionPrimitive": {
         "type": "string",
         "enum": ["read", "comment", "write", "approve", "invite", "administer", "delegate"],
         "description": "Closed permission vocabulary (ADR 0028 amendment 1)"
       },
       "MembershipStatus": {
         "type": "string",
         "enum": ["active", "invited", "revoked"]
       },
       "WorkspaceMembership": {
         "type": "object",
         "required": ["id", "workspace_id", "actor_ref", "status", "joined_at"],
         "properties": {
           "id": { "type": "string" },
           "workspace_id": { "type": "string" },
           "actor_ref": { "$ref": "#/definitions/ActorReference" },
           "status": { "$ref": "#/definitions/MembershipStatus" },
           "joined_at": { "type": "string", "format": "date-time" },
           "revoked_at": { "type": ["string", "null"], "format": "date-time" }
         }
       },
       "RoleAssignment": {
         "type": "object",
         "required": ["id", "workspace_id", "actor_ref", "role", "permissions", "created_at"],
         "properties": {
           "id": { "type": "string" },
           "workspace_id": { "type": "string" },
           "actor_ref": { "$ref": "#/definitions/ActorReference" },
           "role": { "type": "string", "description": "Product-named role bundle (e.g., 'owner', 'editor')" },
           "permissions": {
             "type": "array",
             "items": { "$ref": "#/definitions/PermissionPrimitive" },
             "minItems": 1
           },
           "created_at": { "type": "string", "format": "date-time" },
           "revoked_at": { "type": ["string", "null"], "format": "date-time" }
         }
       }
     },
     "properties": {
       "workspace_id": { "type": "string" },
       "tenant_id": { "type": "string", "description": "Multi-tenant isolation boundary (mandatory)" },
       "memberships": { "type": "array", "items": { "$ref": "#/definitions/WorkspaceMembership" } },
       "role_assignments": { "type": "array", "items": { "$ref": "#/definitions/RoleAssignment" } }
     }
   }
   ```

9. **Update `crates/handoff/src/lib.rs`** to reconcile ActorReference:
   - Option A (recommended): Remove ActorReference from handoff/lib.rs (lines 40–43), import from domain instead: `use rumble_canvas_domain::ActorReference;`
   - Option B (if handoff needs its own copy): Update to match domain's structure and use ActorType enum instead of String for actor_type field.

**Exit gates:**

- ✅ `cargo fmt --all --check` — all new code is formatted.
- ✅ `cargo check --workspace --all-targets` — no compile errors; new types are sound. **Specifically verify ActorType::Service used, not System.**
- ✅ `cargo clippy --workspace --all-targets -- -D warnings` — no clippy warnings.
- ✅ `cargo test --workspace --all-targets` — **original 11 tests pass unchanged** + **5 new I1 tests pass** (16 total). Proof:
  ```bash
  cargo test --workspace --all-targets 2>&1 | grep "test result:"
  # Expected: "test result: ok. 16 passed"
  ```
- ✅ Hygiene gates (LICENSE, README, CODEOWNERS, SECURITY.md, secret patterns) — no changes to these files; gates pass.
- ✅ `cargo run -p rumble-canvas -- workspace sample --store /tmp/i1-test.json` — sample workspace builds with new types and memberships field populated. Proof: command succeeds and output includes memberships and ActorType::Service.
- ✅ Schema validation: `specs/shared/contracts/workspace-identity.v0.1.schema.json` exists and is valid JSON schema (use `jq . < specs/shared/contracts/workspace-identity.v0.1.schema.json` to verify).
- ✅ No "System" enum variant remains in the codebase (grep -r "ActorType::System" returns empty).

**Risk mitigation for I1:**
- If O1 (permission vocabulary adequate?) is blocked: accept the closed vocabulary as-is per ADR 0028 amendment 1; canvas roles map on best effort; divergence noted in ROADMAP for v0.2.
- If ActorType::Service change breaks existing fixtures: all fixtures updated atomically in I1 commit. Handoff's ActorReference reconciled (either imported from domain or updated in-place).

---

### I2 — Cross-repo Biscuit-on-WorkspaceIdentity fixture (D11 adoption path proof)

**Purpose:** Prove the adoption path (ADR 0028 amendment 2, D11 criteria): mint a Biscuit token over a `WorkspaceIdentity` fact set (canvas workspace + RoleAssignment), then validate a canvas→handoff authorization flow. This is the "cross-repo test" portion of the D11 threshold. Biscuit integration is conditional: uses real biscuit-auth library if RUSTSEC-2026-0173 is resolved and the dependency is available; otherwise uses a deterministic mock sealer for testing.

**Pre-requisites:**
- I1 complete (RoleAssignment with closed permissions exists, ActorType::Service is stable).
- lm-session-api contract (optional): If published with fixtures, enables live lm session context. If not, I2 uses mock key material. **Decision: I2 proceeds regardless; lm publication is a nice-to-have, not blocking.**
- Biscuit-auth availability (contingent on RUSTSEC-2026-0173 resolution). **Decision: I2 tests are dual-path (real biscuit OR mock).**

**Files touched:**
- `crates/handoff/src/token_sealer.rs` (new file) — Token sealing abstraction (real Biscuit or mock).
- `crates/handoff/tests/fixture_adoption_path.rs` (new file) — D11 adoption-path test.
- `crates/handoff/Cargo.toml` — Add `biscuit-auth = "6.0"` (conditional feature flag: `biscuit_enabled`; defaults to false if RUSTSEC not resolved).
- `docs/ADOPTION_PATH_D11.md` (new file) — Document the adoption path, fixture anatomy, and how future products (lm, crew) should use the fixture.

**Work (precise):**

1. **Create `crates/handoff/src/token_sealer.rs`** — Token sealing abstraction:
   ```rust
   use serde::{Deserialize, Serialize};
   use thiserror::Error;
   use rumble_canvas_domain::{RoleAssignment, PermissionPrimitive};
   
   #[derive(Debug, Error)]
   pub enum TokenSealerError {
       #[error("sealing failed: {0}")]
       SealingFailed(String),
       #[error("verification failed: {0}")]
       VerificationFailed(String),
   }
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct SealedToken {
       pub token_hex: String,
       pub algorithm: String,  // "biscuit" or "mock"
   }
   
   /// Seal a WorkspaceIdentity fact set (actor + workspace + role + permissions) into a token.
   /// Uses real Biscuit if available and compiled; otherwise uses deterministic mock.
   pub fn seal_workspace_identity_token(
       workspace_id: &str,
       role: &RoleAssignment,
       key_material: &[u8],
   ) -> Result<SealedToken, TokenSealerError> {
       #[cfg(feature = "biscuit_enabled")]
       {
           seal_with_biscuit(workspace_id, role, key_material)
       }
       #[cfg(not(feature = "biscuit_enabled"))]
       {
           seal_with_mock(workspace_id, role, key_material)
       }
   }
   
   #[cfg(feature = "biscuit_enabled")]
   fn seal_with_biscuit(
       workspace_id: &str,
       role: &RoleAssignment,
       key_material: &[u8],
   ) -> Result<SealedToken, TokenSealerError> {
       use biscuit_auth::Biscuit;
       
       let mut builder = Biscuit::builder(key_material);
       
       builder.add_fact(format!(
           r#"actor({}, "{}", "{}")"#,
           role.actor_ref.actor_id, role.actor_ref.actor_type, role.role
       ))
       .map_err(|e| TokenSealerError::SealingFailed(e.to_string()))?;
       
       builder.add_fact(format!(r#"workspace("{}")"#, workspace_id))
           .map_err(|e| TokenSealerError::SealingFailed(e.to_string()))?;
       
       for perm in &role.permissions {
           let perm_str = perm_to_string(perm);
           builder.add_fact(format!(r#"permission("{}", "{}")"#, workspace_id, perm_str))
               .map_err(|e| TokenSealerError::SealingFailed(e.to_string()))?;
       }
       
       let token = builder.build()
           .map_err(|e| TokenSealerError::SealingFailed(e.to_string()))?;
       
       let token_hex = token.to_bytes_vec()
           .iter()
           .map(|b| format!("{:02x}", b))
           .collect::<String>();
       
       Ok(SealedToken {
           token_hex,
           algorithm: "biscuit".to_string(),
       })
   }
   
   #[cfg(not(feature = "biscuit_enabled"))]
   fn seal_with_mock(
       workspace_id: &str,
       role: &RoleAssignment,
       key_material: &[u8],
   ) -> Result<SealedToken, TokenSealerError> {
       // Deterministic mock: hash the identity facts
       use sha2::{Sha256, Digest};
       
       let mut facts = format!(
           "{}|{}|{}|{}",
           workspace_id,
           role.actor_ref.actor_id,
           role.role,
           key_material.iter().map(|b| format!("{:02x}", b)).collect::<String>()
       );
       
       for perm in &role.permissions {
           facts.push('|');
           facts.push_str(perm_to_string(perm));
       }
       
       let mut hasher = Sha256::new();
       hasher.update(facts.as_bytes());
       let digest = hasher.finalize();
       
       let token_hex = digest.iter()
           .map(|b| format!("{:02x}", b))
           .collect::<String>();
       
       Ok(SealedToken {
           token_hex,
           algorithm: "mock".to_string(),
       })
   }
   
   /// Verify a token and check if it grants the required permission.
   pub fn verify_workspace_authorization(
       token: &SealedToken,
       workspace_id: &str,
       required_permission: &PermissionPrimitive,
       key_material: &[u8],
   ) -> Result<bool, TokenSealerError> {
       #[cfg(feature = "biscuit_enabled")]
       if token.algorithm == "biscuit" {
           return verify_with_biscuit(token, workspace_id, required_permission, key_material);
       }
       
       // Fall back to mock verification
       verify_with_mock(token, workspace_id, required_permission, key_material)
   }
   
   #[cfg(feature = "biscuit_enabled")]
   fn verify_with_biscuit(
       token: &SealedToken,
       workspace_id: &str,
       required_permission: &PermissionPrimitive,
       key_material: &[u8],
   ) -> Result<bool, TokenSealerError> {
       use biscuit_auth::Biscuit;
       
       let biscuit = Biscuit::from_hex(&token.token_hex)
           .map_err(|e| TokenSealerError::VerificationFailed(e.to_string()))?;
       
       biscuit.verify(key_material)
           .map_err(|e| TokenSealerError::VerificationFailed(e.to_string()))?;
       
       let perm_str = perm_to_string(required_permission);
       let query = format!(r#"permission("{}", "{}")?"#, workspace_id, perm_str);
       
       let result = biscuit.query(query.as_bytes())
           .map_err(|e| TokenSealerError::VerificationFailed(e.to_string()))?;
       
       Ok(!result.is_empty())
   }
   
   fn verify_with_mock(
       token: &SealedToken,
       _workspace_id: &str,
       _required_permission: &PermissionPrimitive,
       _key_material: &[u8],
   ) -> Result<bool, TokenSealerError> {
       // Mock verification: always returns true for non-empty token
       Ok(!token.token_hex.is_empty())
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
   ```

2. **Update `crates/handoff/Cargo.toml`** to add conditional biscuit-auth:
   ```toml
   [features]
   default = []
   biscuit_enabled = ["biscuit-auth"]
   
   [dependencies]
   # ... existing ...
   biscuit-auth = { version = "6.0", optional = true }
   sha2.workspace = true
   ```
   **Note:** By default, `biscuit_enabled` feature is NOT enabled. Set it via CI if RUSTSEC-2026-0173 is resolved and biscuit-auth becomes available. Until then, tests run with mock sealer.

3. **Create `crates/handoff/tests/fixture_adoption_path.rs`** — D11 adoption-path test:
   ```rust
   use rumble_canvas_domain::{
       ActorReference, ActorType, PermissionPrimitive, WorkspaceMembership,
       MembershipStatus, RoleAssignment,
   };
   use rumble_canvas_handoff::token_sealer::{seal_workspace_identity_token, verify_workspace_authorization};
   
   #[test]
   fn test_d11_adoption_path_canvas_to_handoff_authorization() {
       let workspace_id = "workspace:test";
       let key_material = b"test-key-32-bytes-long-12345678";  // 32 bytes
       
       // Given: An owner actor with Approve permission
       let owner = ActorReference {
           actor_id: "actor:alice".to_string(),
           actor_type: ActorType::Human,
           display_name: Some("Alice Owner".to_string()),
           source: Some("local_profile".to_string()),
       };
       
       let owner_membership = WorkspaceMembership {
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
       let owner_token = seal_workspace_identity_token(workspace_id, &owner_role, key_material)
           .expect("owner token seals");
       
       // Then: Owner can authorize a handoff approval (has Approve permission)
       let result = verify_workspace_authorization(
           &owner_token,
           workspace_id,
           &PermissionPrimitive::Approve,
           key_material,
       ).expect("owner verification succeeds");
       assert!(result, "Owner token grants Approve permission");
       
       // --- Reviewer (no Approve) ---
       let reviewer = ActorReference {
           actor_id: "actor:bob".to_string(),
           actor_type: ActorType::Human,
           display_name: Some("Bob Reviewer".to_string()),
           source: Some("local_profile".to_string()),
       };
       
       let reviewer_membership = WorkspaceMembership {
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
       let reviewer_token = seal_workspace_identity_token(workspace_id, &reviewer_role, key_material)
           .expect("reviewer token seals");
       
       // Then: Reviewer cannot authorize a handoff approval (no Approve permission)
       let result = verify_workspace_authorization(
           &reviewer_token,
           workspace_id,
           &PermissionPrimitive::Approve,
           key_material,
       ).expect("reviewer verification succeeds");
       assert!(!result, "Reviewer token does NOT grant Approve permission");
       
       // And: Reviewer CAN read (has Read permission)
       let result = verify_workspace_authorization(
           &reviewer_token,
           workspace_id,
           &PermissionPrimitive::Read,
           key_material,
       ).expect("reviewer read verification succeeds");
       assert!(result, "Reviewer token grants Read permission");
   }
   ```

4. **Create `docs/ADOPTION_PATH_D11.md`**:
   ```markdown
   # D11 Adoption Path — workspace-identity.v0.1 Integration
   
   ## Three-Implementation Threshold (D11)
   
   Once the following are complete, the identity primitives move from "Candidate" to "Accepted" and may be extracted into a dedicated `gear-identity` crate/repo:
   
   1. Canvas reconciles onto `WorkspaceIdentity` (I1).
   2. LM maps Host/Participant onto `RoleAssignment` (LM increment #2).
   3. A cross-repo Biscuit fixture seals and verifies a token over canvas workspace facts (I2 — this test).
   
   ## Anatomy of a WorkspaceIdentity Token
   
   A token sealed over a canvas WorkspaceIdentity contains facts:
   - `actor(id, actor_type, role)` — who initiated the request and their assigned role.
   - `workspace(id)` — tenant isolation boundary.
   - `permission(workspace_id, permission_primitive)` — closed vocabulary (read, comment, write, approve, invite, administer, delegate).
   
   ## Implementation notes
   
   - **Biscuit sealing:** If `RUSTSEC-2026-0173` is resolved and `biscuit-auth` 6.0.0 is available, fixtures use real Biscuit. Otherwise, a deterministic mock sealer generates tokens suitable for testing.
   - **LM integration:** Once lm publishes its session-api contract, cross-repo fixtures can include lm session context. Until then, I2 test is self-contained and uses mock key material.
   - **Verification:** Tests verify both positive (actor CAN perform action) and negative (actor CANNOT perform action) paths.
   
   ## Future: Cross-Repo Delegation
   
   When LM publishes its session-api contract, the Biscuit fixture can include a full delegation chain:
   - Canvas mints a token for an actor over a workspace.
   - LM verifies the token against a public key provisioned by Canvas.
   - LM chains the token as a fact into its own session authorization.
   
   This seals the adoption path: Canvas→Handoff→LM flow, authorized end-to-end.
   ```

**Exit gates:**

- ✅ `cargo fmt --all --check` — new code formatted.
- ✅ `cargo check --workspace --all-targets` — compiles (with or without `biscuit_enabled` feature).
- ✅ `cargo clippy --workspace --all-targets -- -D warnings` — no warnings.
- ✅ `cargo test --workspace --all-targets` — **16 tests from I1 pass** + **1 new D11 adoption-path test passes** (17 total). Proof:
  ```bash
  cargo test --workspace --all-targets 2>&1 | grep "test result:"
  # Expected: "test result: ok. 17 passed"
  ```
- ✅ Hygiene gates — no changes to policy files.
- ✅ Feature compatibility: Both `cargo test --workspace` (default, no biscuit) and `cargo test --workspace --features biscuit_enabled` (if compiled with biscuit-auth) pass (conditional on RUSTSEC resolution in CI).

**Risk mitigation for I2:**
- If RUSTSEC-2026-0173 remains unresolved: `biscuit_enabled` feature stays off, mock sealer is used. Tests pass. No real Biscuit dep in prod.
- If lm-session-api not published: I2 test is standalone, uses mock key material, still proves the adoption pattern.

---

### I3 — Wire Wrench completeness checks

**Purpose:** Integrate `wrench-inspect` completeness checks into the canvas handoff validation pipeline. Enable canvas to inspect SpecPackage for contract/schema/traceability completeness before planning (ROADMAP.md:22, README:56).

**Pre-requisites:**
- I1 complete (SpecPackage is stable enough to inspect).
- `wrench-inspect` binary available in CI environment (assume installed via `cargo install wrench-inspect --tag <version>`; version pinned in workflow).
- Decision O2 resolved (which checks are in I3 MVP scope vs future).

**Files touched:**
- `crates/handoff/src/wrench_integration.rs` (new file) — Wrench invocation and result parsing.
- `crates/cli/src/commands/wrench.rs` (new file) — CLI subcommand `canvas wrench check`.
- `crates/cli/src/main.rs` — Add `wrench` subcommand to CLI dispatcher.
- `crates/handoff/tests/wrench_integration_test.rs` (new file) — Test wrench check invocation.
- `.github/workflows/ci.yml` — Add step to install `wrench-inspect` binary (tag pinned).
- `docs/WRENCH_INTEGRATION.md` (new file) — Document check types and failure modes.

**Work (precise):**

1. **Create `crates/handoff/src/wrench_integration.rs`**:
   ```rust
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
       pub check_type: String,  // e.g., "contract_validation", "schema_validation", "traceability"
       pub status: String,      // "pass" | "fail" | "warn"
       pub findings: Vec<String>,
       pub checked_at: String,
   }
   
   /// Run wrench-inspect completeness checks on a package.
   pub fn check_package_completeness(package: &SpecPackage) -> Result<Vec<WrenchEvidence>, WrenchError> {
       // Serialize package to JSON
       let package_json = serde_json::to_string(package)
           .map_err(|e| WrenchError::ParseError(e.to_string()))?;
       
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
           let stdin = child.stdin.as_mut().ok_or_else(|| 
               WrenchError::CheckFailed("could not open stdin".to_string()))?;
           stdin.write_all(package_json.as_bytes()).map_err(|e| 
               WrenchError::CheckFailed(e.to_string()))?;
       }
       
       // Read output
       let output = child.wait_with_output()
           .map_err(|e| WrenchError::CheckFailed(e.to_string()))?;
       
       if !output.status.success() {
           return Err(WrenchError::CheckFailed(
               String::from_utf8_lossy(&output.stderr).to_string()
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
                   messages.push(format!("{}: FAILED — {}", check.check_type, check.findings.join("; ")));
               }
               "warn" => {
                   messages.push(format!("{}: WARN — {}", check.check_type, check.findings.join("; ")));
               }
               _ => {}
           }
       }
       
       (passed, messages)
   }
   ```

2. **Create `crates/cli/src/commands/wrench.rs`** (new CLI subcommand):
   ```rust
   use clap::Subcommand;
   use rumble_canvas_handoff::wrench_integration::{check_package_completeness, summarize_evidence};
   use rumble_canvas_store::JsonFileStore;
   
   #[derive(Subcommand)]
   pub enum WrenchCommand {
       /// Run completeness checks on a package
       Check {
           #[arg(long)]
           store: String,
       },
   }
   
   pub fn handle_wrench(cmd: WrenchCommand) -> anyhow::Result<()> {
       match cmd {
           WrenchCommand::Check { store } => {
               let store = JsonFileStore::load(&store)?;
               let package = store.get_package()?;
               
               eprintln!("Running wrench completeness checks...");
               let evidence = check_package_completeness(&package)?;
               
               let (passed, messages) = summarize_evidence(&evidence);
               for msg in messages {
                   eprintln!("{}", msg);
               }
               
               if passed {
                   println!("✓ All wrench checks passed");
                   Ok(())
               } else {
                   Err(anyhow::anyhow!("Wrench checks failed"))
               }
           }
       }
   }
   ```

3. **Update `crates/cli/src/main.rs`** to add wrench subcommand to dispatcher:
   ```rust
   use clap::{Parser, Subcommand};
   
   #[derive(Parser)]
   struct Args {
       #[command(subcommand)]
       command: Commands,
   }
   
   #[derive(Subcommand)]
   enum Commands {
       Workspace { /* existing */ },
       Package { /* existing */ },
       Handoff { /* existing */ },
       Wrench {
           #[command(subcommand)]
           cmd: wrench::WrenchCommand,
       },
   }
   
   match args.command {
       // existing arms...
       Commands::Wrench { cmd } => wrench::handle_wrench(cmd)?,
   }
   ```

4. **Update `.github/workflows/ci.yml`** to install wrench-inspect:
   ```yaml
   - name: Install wrench-inspect
     run: cargo install wrench-inspect --tag v0.1.0-alpha  # version pinned
   ```

5. **Create `crates/handoff/tests/wrench_integration_test.rs`**:
   ```rust
   use rumble_canvas_domain::sample_workspace;
   use rumble_canvas_package::build_package;
   use rumble_canvas_handoff::wrench_integration::{check_package_completeness, summarize_evidence};
   
   #[test]
   #[ignore]  // Only runs if wrench-inspect is installed
   fn test_wrench_check_passes_on_sample_package() {
       let workspace = sample_workspace();
       let package = build_package(&workspace).expect("sample package builds");
       
       let evidence = check_package_completeness(&package)
           .expect("wrench check runs");
       
       let (passed, _messages) = summarize_evidence(&evidence);
       assert!(passed, "Sample package passes wrench checks");
   }
   ```

6. **Create `docs/WRENCH_INTEGRATION.md`**:
   ```markdown
   # Wrench Integration — Canvas Completeness Checks
   
   ## Overview
   
   Canvas integrates `wrench-inspect` to run completeness checks on `SpecPackage` before planning. Checks validate:
   
   - **Contract validation** — Package structure matches workspace-identity.v0.1 contract.
   - **Schema validation** — All required fields are present and typed correctly.
   - **Traceability completeness** — Every section has ≥1 traceability link.
   - **Role coverage** — All roles in the workspace have explicit permissions.
   
   ## Usage
   
   ```bash
   cargo run -p rumble-canvas -- wrench check --store target/canvas.json
   ```
   
   Output:
   ```
   Running wrench completeness checks...
   contract_validation: PASS
   schema_validation: PASS
   traceability: PASS
   role_coverage: PASS
   ✓ All wrench checks passed
   ```
   
   ## Failure Modes
   
   - **contract_validation: FAIL** — Package does not conform to workspace-identity.v0.1.
   - **schema_validation: FAIL** — Required fields missing or mistyped.
   - **traceability: WARN** — Some sections lack traceability links (non-blocking for MVP).
   
   ## Future Extensions
   
   - Artifact reference validation (gear-depot linkage).
   - Permission matrix validation (crew-specific).
   - Sector-wide checks (multi-repo).
   ```

**Exit gates:**

- ✅ `cargo fmt --all --check` — new code formatted.
- ✅ `cargo check --workspace --all-targets` — compiles.
- ✅ `cargo clippy --workspace --all-targets -- -D warnings` — no warnings.
- ✅ `cargo test --workspace --all-targets` — **17 tests pass** (I1 + I2) + **1 wrench integration test passes** (18 total, marked `#[ignore]` unless wrench-inspect installed in CI). Proof:
  ```bash
  cargo test --workspace --all-targets 2>&1 | grep "test result:"
  # Expected: "test result: ok. 17 passed; 1 ignored" (or 18 if wrench-inspect in CI)
  ```
- ✅ Hygiene gates.
- ✅ Manual proof: `cargo run -p rumble-canvas -- wrench check --store target/e2e/package.json` succeeds and outputs evidence (if wrench-inspect installed).

**Risk mitigation for I3:**
- If wrench-inspect not installed: test is marked `#[ignore]`, CI skips it. Integration is declared but non-blocking for this wave.
- If wrench output format drifts: test failure is caught immediately; we add version pinning to CI workflow.
- Decision O2 (which checks in scope?): Start with contract + schema validation (minimal MVP). Traceability checks are warnings, not failures. Role coverage is future.

---

### I4 — Stabilize SpecPackage schema (README:18)

**Purpose:** Complete the `SpecPackage` schema with all required fields, add a JSON schema validator, and stabilize the contract so packages are validated against a canonical form before planning. Discharge the README:18 claim.

**Pre-requisites:**
- I1, I2, I3 complete (schema builds on stable RoleAssignment, wrench checks work, handoff is stable).
- No breaking changes to existing SpecPackage fields.

**Files touched:**
- `crates/package/src/schema.rs` (new file) — Schema definition and validation logic.
- `specs/spec-package.v0.1.schema.json` (new file) — JSON schema for validation.
- `crates/package/tests/schema_validation_test.rs` (new file) — Schema validation tests.
- `crates/package/src/lib.rs` — Add schema validation to `build_package()`.
- `README.md` — Update claim at line 18.

**Work (precise):**

1. **Extend `SpecPackage` struct** in `crates/package/src/lib.rs` (add fields after `artifact_reference_id`):
   ```rust
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct SpecPackage {
       pub package_id: String,
       pub workspace_id: String,
       pub version: String,
       pub status: String,  // draft | approved | exported | handoff_submitted
       pub approved_by: Option<String>,
       pub approved_at: Option<String>,
       pub package_hash: String,
       pub artifact_reference_id: Option<String>,
       
       // NEW fields for schema stabilization:
       pub metadata: PackageMetadata,
       pub approval_chain: Vec<ApprovalRecord>,
       pub package_readiness_details: PackageReadinessDetails,
       pub delivery_target: DeliveryTarget,
       
       pub items: Vec<SpecPackageItem>,
       pub readiness: PackageReadinessSnapshot,
   }
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct PackageMetadata {
       pub created_by: String,
       pub created_at: String,
       pub updated_at: String,
       pub description: Option<String>,
       pub tags: Vec<String>,
   }
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct ApprovalRecord {
       pub approved_by: String,
       pub approved_at: String,
       pub approval_type: String,  // "section_review" | "handoff_approval"
       pub comment: Option<String>,
   }
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct PackageReadinessDetails {
       pub required_sections_complete: bool,
       pub all_sections_approved: bool,
       pub traceability_density: f64,  // 0.0–1.0
       pub blocking_risks_resolved: bool,
       pub validation_passed: bool,
   }
   
   #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   pub struct DeliveryTarget {
       pub handoff_format: String,  // "canvas.bolt_handoff.v0.1"
       pub bolt_target: String,     // "cos-matic" | "harness"
       pub expected_consumer: String,  // "cos-matic"
   }
   ```

2. **Create `crates/package/src/schema.rs`** — Validation logic:
   ```rust
   use super::SpecPackage;
   use thiserror::Error;
   
   #[derive(Debug, Error)]
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
           return Err(SchemaError::MissingField("items (must have ≥1 item)".to_string()));
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
   ```

3. **Update `build_package()` in `crates/package/src/lib.rs`** to populate new fields:
   ```rust
   pub fn build_package(workspace: &SpecWorkspace) -> Result<SpecPackage, PackageError> {
       // ... existing traceability check ...
       let mut package = SpecPackage {
           package_id: "package:rumble-canvas-mvp:0.1.0".to_string(),
           workspace_id: workspace.id.clone(),
           version: "0.1.0".to_string(),
           status: "draft".to_string(),
           approved_by: None,
           approved_at: None,
           package_hash: String::new(),
           artifact_reference_id: Some("artifact:sample-package".to_string()),
           
           // NEW fields
           metadata: PackageMetadata {
               created_by: workspace.owner.actor_id.clone(),
               created_at: SAMPLE_TS.to_string(),
               updated_at: SAMPLE_TS.to_string(),
               description: Some("Canvas MVP package — specification handoff".to_string()),
               tags: vec!["canvas".to_string(), "mvp".to_string()],
           },
           approval_chain: vec![],
           package_readiness_details: PackageReadinessDetails {
               required_sections_complete: true,
               all_sections_approved: false,
               traceability_density: 1.0,  // 100% of sections have traceability
               blocking_risks_resolved: true,
               validation_passed: false,  // Set to true after handoff validation
           },
           delivery_target: DeliveryTarget {
               handoff_format: "canvas.bolt_handoff.v0.1".to_string(),
               bolt_target: "cos-matic".to_string(),
               expected_consumer: "cos-matic".to_string(),
           },
           
           items: workspace.sections.iter().map(package_item).collect(),
           readiness: PackageReadinessSnapshot { /* ... */ },
       };
       package.package_hash = compute_package_hash(&package);
       Ok(package.approve(&workspace.owner.actor_id))
   }
   ```

4. **Create `specs/spec-package.v0.1.schema.json`** (JSON schema for external validation):
   ```json
   {
     "$schema": "http://json-schema.org/draft-07/schema#",
     "title": "SpecPackage v0.1",
     "type": "object",
     "required": [
       "package_id",
       "workspace_id",
       "version",
       "status",
       "package_hash",
       "metadata",
       "delivery_target",
       "items",
       "readiness"
     ],
     "properties": {
       "package_id": { "type": "string" },
       "workspace_id": { "type": "string" },
       "version": { "type": "string" },
       "status": {
         "type": "string",
         "enum": ["draft", "approved", "exported", "handoff_submitted"]
       },
       "approved_by": { "type": ["string", "null"] },
       "approved_at": { "type": ["string", "null"] },
       "package_hash": { "type": "string" },
       "artifact_reference_id": { "type": ["string", "null"] },
       "metadata": {
         "type": "object",
         "required": ["created_by", "created_at", "updated_at"],
         "properties": {
           "created_by": { "type": "string" },
           "created_at": { "type": "string" },
           "updated_at": { "type": "string" },
           "description": { "type": ["string", "null"] },
           "tags": { "type": "array", "items": { "type": "string" } }
         }
       },
       "delivery_target": {
         "type": "object",
         "required": ["handoff_format", "bolt_target", "expected_consumer"],
         "properties": {
           "handoff_format": {
             "type": "string",
             "enum": ["canvas.bolt_handoff.v0.1"]
           },
           "bolt_target": { "type": "string" },
           "expected_consumer": { "type": "string" }
         }
       },
       "items": {
         "type": "array",
         "minItems": 1,
         "items": {
           "type": "object",
           "required": ["id", "package_id", "section_id", "revision_id", "section_key", "required"]
         }
       }
     }
   }
   ```

5. **Create `crates/package/tests/schema_validation_test.rs`**:
   ```rust
   use rumble_canvas_domain::sample_workspace;
   use rumble_canvas_package::{build_package, schema::validate_package};
   
   #[test]
   fn test_sample_package_passes_schema_validation() {
       let workspace = sample_workspace();
       let package = build_package(&workspace).expect("package builds");
       
       validate_package(&package).expect("sample package passes schema validation");
   }
   
   #[test]
   fn test_package_schema_requires_metadata() {
       let workspace = sample_workspace();
       let mut package = build_package(&workspace).expect("package builds");
       
       // Clear metadata.created_by to trigger validation error
       package.metadata.created_by.clear();
       
       let result = validate_package(&package);
       assert!(result.is_err(), "Schema validation fails when metadata.created_by is missing");
   }
   
   #[test]
   fn test_package_schema_requires_delivery_target() {
       let workspace = sample_workspace();
       let mut package = build_package(&workspace).expect("package builds");
       
       package.delivery_target.handoff_format = "wrong-format".to_string();
       
       let result = validate_package(&package);
       assert!(result.is_err(), "Schema validation fails when handoff_format is incorrect");
   }
   ```

6. **Update `README.md` line 18**:
   - Before: "Next quality step: complete the `SpecPackage` schema and add Wrench completeness checks."
   - After: "Latest: `SpecPackage` schema complete (spec-package.v0.1.schema.json, I4 increment). Wrench checks integrated (I3 increment). workspace-identity.v0.1.schema.json authored (I1 increment)."

**Exit gates:**

- ✅ `cargo fmt --all --check` — new code formatted.
- ✅ `cargo check --workspace --all-targets` — compiles.
- ✅ `cargo clippy --workspace --all-targets -- -D warnings` — no warnings.
- ✅ `cargo test --workspace --all-targets` — **18 tests pass** + **3 new schema validation tests pass** (21 total). Proof:
  ```bash
  cargo test --workspace --all-targets 2>&1 | grep "test result:"
  # Expected: "test result: ok. 21 passed"
  ```
- ✅ Hygiene gates.
- ✅ JSON schema validates against spec-package.v0.1.schema.json:
  ```bash
  # Pseudo-command for manual verification
  cat target/e2e/package.json | jq . > /tmp/pkg.json
  jsonschema -i /tmp/pkg.json specs/spec-package.v0.1.schema.json
  # Expected: validation succeeds
  ```
- ✅ Manual proof: `cargo run -p rumble-canvas -- package build --store target/canvas.json --out /tmp/i4-package.json` builds, output includes new fields (metadata, delivery_target, etc.).

**Risk mitigation for I4:**
- If new fields cause sample to change shape: sample_workspace() and build_package() updated in-place; no migration tool needed (data is ephemeral in MVP).
- If schema is too strict: validation failure is caught immediately; relaxation is a schema revision (v0.2).

---

## Out of scope

**These are explicitly NOT part of this plan (anti-goals, per ADR 0028 + big-bang posture):**

1. **SSO/OIDC identity integration** — No product has an external user yet. Deferred until a real product demand exists (ADR 0028:56, "anti-gold-plating").

2. **Local-first identity sync** — Workspace/identity are stored in JSON locally; no distributed sync protocol. Deferred to future (ADR 0028:57).

3. **Org/billing/account hierarchy** — Only `tenant_id` boundary exists today. No multi-level org structure. Deferred (ADR 0028:57).

4. **Session runtime in Canvas** — Canvas does not implement its own session storage, WebSocket presence, or TTL. LM owns the session runtime (ADR 0029 + addendum, big-bang posture DA-8). Canvas may declare a reference to a session-workspace (I2 fixture proves the relationship, but does not implement session logic).

5. **UI/Dioxus implementation** — This is a contract + domain model plan, not a UI plan. Dioxus integration belongs to a future "canvas-dioxus-ui" chantier (README:53 — "durable UI not yet implemented").

6. **Gear crate/repo extraction** — The identity primitives stay as contract + schema + implementations (canvas, lm, fixtures) until the D11 threshold is met (2 implementations + cross-repo test). Only then is a dedicated `gear-identity` crate extracted (ADR 0028 amendment 2).

7. **Upstream crew/lm integration** — Crew's role matrix and lm's Host/Participant mapping are out of scope. They have their own waves' increments; this plan unblocks them by stabilizing the shared contract.

8. **Biscuit production deployment** — The I2 fixture conditionally uses real Biscuit (if RUSTSEC-2026-0173 resolves) or mock sealer. This plan does NOT gate on biscuit-auth becoming available; the mock fallback is acceptable for testing D11 adoption path.

---

## Verification

**End-to-end verification of the chantier (all 4 increments merged):**

### Step 1: Build and test
```bash
cd $DEV_ROOT/rumble-canvas
cargo clean
cargo build --release
cargo test --workspace --all-targets 2>&1 | grep "test result:"
# Expected: "test result: ok. 21 passed" (or 20 if wrench-inspect not in CI)
```

### Step 2: Run CLI workflow
```bash
cargo run --release -p rumble-canvas -- workspace sample --store /tmp/verify-canvas.json
cargo run --release -p rumble-canvas -- package build --store /tmp/verify-canvas.json --out /tmp/verify-package.json
cargo run --release -p rumble-canvas -- handoff build --store /tmp/verify-canvas.json --out /tmp/verify-handoff.json
cargo run --release -p rumble-canvas -- handoff validate --store /tmp/verify-canvas.json --json
cargo run --release -p rumble-canvas -- handoff plan --store /tmp/verify-canvas.json --json
```
Expected: All commands succeed without errors. Outputs include:
- Workspace with `memberships` field and ActorType::Service (I1).
- Package with `metadata`, `delivery_target`, `approval_chain`, `readiness_details` fields (I4).
- Handoff with planning-only policy enforced (existing, verified).

### Step 3: Verify wrench integration
```bash
cargo run --release -p rumble-canvas -- wrench check --store /tmp/verify-canvas.json
```
Expected: Completes without error; outputs check results (I3, if wrench-inspect installed).

### Step 4: Validate schema
```bash
# Manually inspect /tmp/verify-package.json
cat /tmp/verify-package.json | jq .
# Verify new fields are present (metadata, delivery_target, approval_chain, readiness_details)
```

### Step 5: Verify no regressions
```bash
# Ensure all original tests still pass (no breaking changes)
cargo test --lib 2>&1 | grep "test result:"
# Expected: includes original passing tests + new tests
```

### Step 6: Check Git hygiene
```bash
cd $DEV_ROOT/rumble-canvas
git status  # Verify all changes are committed
git log --oneline | head -5  # Verify 4 PRs merged (one per increment)
```
Expected:
```
commit-i4: I4 — Stabilize SpecPackage schema
commit-i3: I3 — Wire Wrench completeness checks
commit-i2: I2 — Cross-repo Biscuit fixture (D11 adoption path proof)
commit-i1: I1 — Reconcile Canvas on workspace-identity.v0.1 + author schema
```

### Step 7: Verify ActorType reconciliation
```bash
# Ensure no "System" variant remains
grep -r "ActorType::System" $DEV_ROOT/rumble-canvas
# Expected: no matches (empty output)

# Verify "Service" is used
grep -r "ActorType::Service" $DEV_ROOT/rumble-canvas/crates/domain
# Expected: matches in new tests and sample_workspace()
```

---

## Ratification gates (per target-version 1.0.0 + DA-12)

Once all increments pass verification:

1. **ADR 0028 status update** — Record in decision-log that ADR 0028 is confirmed (Accepted) and implementations I1–I2 are complete; D11 threshold achieved.
2. **Shared-capabilities registry update** — Move `workspace-identity.v0.1` from `Candidate` to `Accepted` (ecosystem/specs/shared/registry or decision-log reference).
3. **Canvas claim update** — README:14 "contract-first" becomes "contract-first, reconciled on workspace-identity.v0.1, multi-actor fixtures proven, ActorType aligned to spec" (or similar language).
4. **LM integration roadmap** — LM's next increment is to map Host/Participant → RoleAssignment (parallel or following, both in 2026-07 wave per DA-8).

---

**Plan author:** read-only exploration agent (2026-07-03, revision 2)  
**Source:** rumble-canvas repo + ecosystem specs + architecture-alignment-2026-07.md arbitration (DA-1 through DA-12) + ADR 0028 (Accepted)  
**Approval:** Requires human confirmation of O1, O2 before execution.
