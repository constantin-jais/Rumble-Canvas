**English** · [Français](README.fr.md)

> [!NOTE]
> **Reserved · future home of Spec Studio** — rebuilt in the canonical base repository [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) ([multi-repo topology, ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).
> This repository will reopen as the real product repository when the owner activates it, consuming the base as a versioned dependency. The foundations described below are **being built now** — with links to the code that already exists.

# Spec Studio

**Contract-first product workspace for turning conversations into decisions, specs, and planning handoffs.** Create an intent; add requirements, decisions, and sourced contracts; freeze an immutable, content-addressed **SpecPackage** with approvals and evidence; emit a planning-only handoff that downstream planners consume without execution capability. Never silent, always traceable — every package is locked by rule, never mutated.

The canonical brief it answers: _"we talk about it, we decide it, we write it, we hand it off"_ — a bounded, self-contained workspace that makes every step explicit and every participant's role distinct (author, reviewer, approver). Both internal reasoning and external handoff are first-class products, independently versionable.

## Why it's different

- **Contract-first, not markdown.** A spec is not a document — it is a structured SpecPackage containing problem, actors, requirements (each with priority), sourced contracts, decision records, risk mitigations, and acceptance criteria with observable evidence. Markdown lives inside the evidence field, never as the package itself.
- **Immutable accepted state.** An accepted package is content-addressed by digest and **never** mutated. Approval creates a named lineage edge; supersession starts a new version, never a patch.
- **Deny by default on completeness.** Missing problem, open decisions, unmapped contracts across boundaries, unverifiable criteria, or inadequate approvals (no separation) block acceptance. Validation returns stable rule IDs; it never fills blanks automatically.
- **Handoff is a separate capability.** Planning-only handoff carries the accepted package reference and evidence, grants zero execution rights, and includes a Biscuit attestation verifying hash and audience. Planner reads; planner never runs.
- **Real-time collaboration, frozen on submission (planned).** The design ([#198](https://github.com/libre-ai/libre-ai/pull/198), owner-signed) has authors and reviewers co-edit the DRAFT workspace in real time through end-to-end-encrypted MLS — Libre AI's sovereign collaboration brick, still to be implemented — and freeze the CRDT to append-only comment threads once submitted for review.

## Status — spec-published, foundations under construction

Spec Studio is being rebuilt from locked contracts. It is **not released yet**; the domain and acceptance logic come first, and a good part of it already exists and is proven in the base repository:

| Foundation                                                        | State               | Evidence                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`spec-package.v1`** — content-addressed immutable schema        | ✅ built, validated | Contract and golden vectors ([specs.v1.schema.json](https://github.com/libre-ai/libre-ai/blob/main/contracts/schemas/spec-package.v1.schema.json))                                                                                                                                    |
| **`agent-handoff.v1`**, **`evidence-report.v1`** — exports        | ✅ built, validated | Planning-only handoff and evidence schemas ([agent-handoff.v1](https://github.com/libre-ai/libre-ai/blob/main/contracts/schemas/agent-handoff.v1.schema.json), [evidence-report.v1](https://github.com/libre-ai/libre-ai/blob/main/contracts/schemas/evidence-report.v1.schema.json)) |
| **Workspace lifecycle aggregate** — frame, submit, accept         | ✅ built, tested    | Domain logic, RLS protections, append-only events (#172)                                                                                                                                                                                                                              |
| **Accepted spec-package validator** — completeness rules          | ✅ built, tested    | Deny-by-default validator matching SpecPackage v1 schema (#166)                                                                                                                                                                                                                       |
| **Spec-workspace persistence** — PostgreSQL, snapshots + events   | ✅ built, tested    | Tenant-scoped, RLS, immutable accept, migration 0001 (#176)                                                                                                                                                                                                                           |
| **Command service** — write path orchestration                    | ✅ built, tested    | Composes domain logic and persistence for all lifecycle operations (#177)                                                                                                                                                                                                             |
| **Acceptance seam** — decision gate + validation                  | ✅ built, tested    | Pure `decideAcceptance()` + persisted transition, refusal handling (#178)                                                                                                                                                                                                             |
| **Content-addressed spec-packages store** — dedicated persistence | ✅ built, tested    | Idempotent storage, digest conflicts refused, immutable append-only (#201)                                                                                                                                                                                                            |
| **Read-only cockpit** — SSR server-rendered view                  | ✅ built, tested    | Workspace list and details; authoring UI arrives next (#180)                                                                                                                                                                                                                          |
| Authoring UI (frame/requirement/contract/review)                  | ⏳ next             | Create and edit DRAFT workspaces, add requirements, record decisions and contracts                                                                                                                                                                                                    |
| Command surface authorization (Biscuit, tenant isolation)         | ⏳ next             | Author/review/approve/export operations, handoff capability attenuation                                                                                                                                                                                                               |
| Planning-only consumer conformance                                | ⏳ ahead            | Missions/orchestrator integration, handoff verification                                                                                                                                                                                                                               |
| Concurrency and rollback qualification                            | ⏳ ahead            | Conflict resolution, safe timetravel, evidence immutability guarantee                                                                                                                                                                                                                 |

This repository is a public reserved home, intentionally without product code until activation (wave 4). **Benchmark target:** workflow and specification governance tooling (e.g. Notion, Figma design specs) — reached through explicit contract-first structure rather than freeform documents.

## How it works

1. **Frame** — author creates a workspace with problem statement, actors, constraints, and initial hypotheses. The workspace opens in DRAFT mode; all fields are mutable. Validation exposes missing problem, actors, or required decisions immediately.
2. **Specify and review** — author adds requirements (with priority), sourced contracts for cross-boundary dependencies, risk mitigations, and acceptance criteria with observable evidence. Reviewers read and comment; when ready, author submits the package for review. In the signed design, when `collab_enabled`, editor and reviewers share a real-time encrypted workspace (MLS per-epoch keys) and the CRDT freezes to append-only comments on submit — the collaboration brick that carries this is planned, not yet built.
3. **Accept** — approver invokes the acceptance seam: completeness validation runs (problem, actors, requirements, contracts, risks, criteria, approval separation), and on success, the workspace freezes into an immutable, content-addressed **SpecPackage** with digest and approval signatures. Supersession links the new version to the old one; past versions are never rewritten.
4. **Handoff** — authorized user emits a planning-only handoff: a message carrying the accepted package hash, evidence references, and Biscuit attestation. Downstream planner loads the handoff, verifies the digest, and consumes the spec for planning — never for execution or mutation.

## Architecture — built from interoperable bricks

Spec Studio is a product assembled from independently versioned bricks; each is usable and testable on its own, and the product is their composition (the multi-repo target of [ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).

| Brick                                        | Role                                  | Interface it exposes / consumes                                                                                  |
| -------------------------------------------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| **`spec-package.v1`** (JSON Schema + golden) | The immutable specification structure | Content-addressed by SHA-256 digest; validator returns rule IDs; no mutation API                                 |
| **`@libre-ai/web-platform`**                 | SSR / Bun BFF foundation              | Request handler, server-rendered cockpit view, accessibility-first markup                                        |
| **`@libre-ai/data`**                         | PostgreSQL persistence layer          | Workspace lifecycle store, append-only events, content-addressed spec-packages, RLS                              |
| **Contracts**                                | Locked interoperability surface       | `spec-package.v1`, `agent-handoff.v1`, `evidence-report.v1` schemas, `specifications.v1` OpenAPI, golden vectors |

The authorizing host passes canonical specification bytes to the validator; the validator returns rule-by-rule evidence. Any consumer that speaks the same contracts can read and verify the package.

## Where the work happens

All active development is in the base repository, under:

- `apps/specifications` — the product host (SSR cockpit, command service, persistence)
- `contracts/schemas/spec-package.v1.schema.json` — the immutable SpecPackage definition
- `contracts/schemas/agent-handoff.v1.schema.json`, `evidence-report.v1.schema.json` — handoff and evidence contracts
- `contracts/openapi/specifications.v1.yaml` — the API surface
- [`docs/apps/specifications.md`](https://github.com/libre-ai/libre-ai/blob/main/docs/apps/specifications.md) — the full product brief

To follow progress or contribute, open issues and pull requests in [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai). This repository stays reserved until activation.

## License

EUPL-1.2.
