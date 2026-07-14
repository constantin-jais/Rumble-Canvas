# Product Readiness Cockpit

**Canonical date:** 2026-07-14  
**Snapshot:** `main@ba203cf`  
**Maturity:** contract-first  
**Availability:** discovery

This cockpit is a product-readiness snapshot, not a delivery promise.
A single open dependency PR does **not** mean the product is not ready;
`0 issue` is not readiness by itself.

## Current state

| Area | State | Evidence |
| --- | --- | --- |
| Local CLI end-to-end | Proven | `workspace sample`, `package build`, `handoff build`, `handoff validate`, `handoff plan`, `workspace show` |
| Local store | Proven | JSON store load/save round-trip in `crates/store` |
| SpecPackage hash / immutability | Proven | stable `package_hash`, approved package rejects mutation |
| Handoff semantics | Proven | planning-only, dry-run planning, execution refused by invariants |
| Identity / provenance | Partial | multi-actor workspace identity, memberships, role assignments, fixture token path |
| Diagnostics / examples | Partial | wrench integration notes, sample reports, plan docs |
| Durable collaborative UI | Blocked / later | no durable multi-user UI yet |
| Hosted workflow / release provenance | Blocked / later | not shipped here |
| Orchestration / execution | Not in scope | this product slice never executes implementation work |

## What is already proven locally and in CI

- Workspace/package/handoff/plan commands work on the local CLI.
- The store persists locally.
- `SpecPackage` hashing is deterministic and approved packages are immutable.
- Handoff generation enforces `allow_execution = false` and rejects execution paths.
- `handoff plan` is always dry-run.
- The repo keeps planning-only boundaries explicit; no orchestration/execution is owned here.

## What is partial

- Multi-actor identity/provenance exists, but durability and cross-actor productization are still incomplete.
- Diagnostics and examples exist, but the cockpit still needs richer proof surfacing.
- `cosmatic` is an optional dependency for validation/plan; it helps verify the path, but it must not be treated as live execution proof.

## What is blocked or later

- Durable collaborative UI.
- Multi-user product workflow.
- Hosted workflow.
- Release provenance for generated specs.
- Any orchestration or execution capability.

## Gates

- **P0** — local CLI proof already established.
- **P1** — dogfood plus durable identity/provenance.
- **P2** — UI/collaboration only after permissions boundaries are explicit.

## Reading map

- [README](../README.md)
- [ROADMAP](../ROADMAP.md)
- [OPERATIONS](./OPERATIONS.md)
- [Wrench integration](./WRENCH_INTEGRATION.md)

## Notes

- The canonical evidence surface is local-first and docs-first.
- `cosmatic` is a helper for validation and dry-run planning, not a product gate on its own.
- A green PR dependency is useful, but it does not automatically imply product readiness.
