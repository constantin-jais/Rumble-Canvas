# D11 Adoption Path — workspace-identity.v0.1 Integration

## D11 extraction threshold

The shared contract is already Accepted in the control plane. The evidence below is the D11 extraction threshold for any future shared identity crate/repo; extraction is not part of this increment.

1. Canvas reconciles onto `WorkspaceIdentity` (I1).
2. LM maps Host/Participant onto `RoleAssignment` (LM increment #2).
3. A cross-repo Biscuit-shaped fixture seals and verifies a token over canvas workspace facts (I2 — this test).

## Anatomy of a WorkspaceIdentity Token

A token sealed over a canvas WorkspaceIdentity contains facts:

- `organization(tenant_id)` — mandatory tenant isolation boundary from `workspace-identity.v0.1`.
- `actor(id, actor_type, role)` — who initiated the request and their assigned role.
- `workspace(workspace_id)` — workspace scope within the tenant.
- `permission(workspace_id, permission_primitive)` — closed vocabulary (read, comment, write, approve, invite, administer, delegate).

## Implementation notes

- **Biscuit sealing:** This increment uses a deterministic mock sealer to prove the WorkspaceIdentity authorization shape without adding a premature crypto dependency. Real Biscuit mint/verify lands in the M2 authorization increment.
- **LM integration:** Once lm publishes its session-api contract, cross-repo fixtures can include lm session context. Until then, I2 test is self-contained and uses mock key material.
- **Verification:** Tests verify both positive (actor CAN perform action) and negative (actor CANNOT perform action) paths. The deterministic mock fails closed on wrong tenant, workspace, or key material.

## Future: Cross-Repo Delegation

When LM publishes its session-api contract and the M2 real Biscuit verifier lands, this fixture can evolve into a full delegation chain:

- Canvas mints a token for an actor over a tenant-scoped workspace.
- LM verifies the token against the tenant/workspace scope and a public key provisioned by Canvas.
- LM chains the token as a fact into its own session authorization.

This prepares the adoption path: Canvas→Handoff→LM flow, authorized end-to-end once the real M2 verifier replaces the deterministic mock.
