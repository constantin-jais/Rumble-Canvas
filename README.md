# Rumble Canvas

**Layer:** Rumble — Product  
**Role:** product conception workspace  
**Mission:** turn product conversations into shared understanding, specs, screens, and implementation-ready deliverables.

---

## Stack role

- **Layer:** Rumble — Product.
- **Role:** product conception workspace.
- **Mission:** turn product conversations into shared understanding, specs, screens, and implementation-ready deliverables.
- **Maturity:** `contract-first`.
- **Scale-ready:** no — the repo proves contracts and a local CLI handoff, not a full multi-user product.
- **Current increment:** P0 contract.
- **Learning value:** specification, ambiguity, decisions, traceability, and implementation handoff.
- **Next quality step:** complete the `SpecPackage` schema and add Wrench completeness checks.

See the ecosystem cockpit in [`constantin-jais/ecosystem/status.md`](https://github.com/constantin-jais/constantin-jais/blob/main/ecosystem/status.md).

## Dogfooding

This repository is part of the forge dogfooding loop: the ecosystem should use its own tools to make specs, maturity, contracts, releases, and product documentation observable.

Current visible evidence:

- the local CLI path demonstrates package, handoff, validation, and dry-run planning;
- README maturity notes keep schema and UI limits explicit;
- contracts frame product-conception handoff before a full product runtime exists.

Expected next evidence:

- publish example SpecPackage and handoff outputs;
- connect completeness checks to visible Wrench evidence.

Dogfooding claims should stay backed by visible commands, fixtures, CI workflows, generated reports, or linked docs.

## Contributing

See:

- [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines;
- [ROADMAP.md](ROADMAP.md) for current contribution priorities;
- [issue templates](.github/ISSUE_TEMPLATE/) for bugs, docs issues, fixture/example requests, and design discussions.

## Usable today

The local Rust CLI can create a sample workspace, build a package and handoff, validate it, and produce a dry-run plan through `cosmatic`.

## Not scale-ready yet

This is not a full product workspace yet: the core contract path exists, but the durable UI, complete schema, completeness inspection, and team workflow hardening are still missing.

## Next product milestone

Stabilize the `SpecPackage` contract and wire Wrench checks so a Canvas handoff can be inspected before planning.

## Purpose

`rumble-canvas` is the product-conception workspace of the Rumble/Bolt/Wrench/Gear ecosystem.

The product is not “draw a UI from a prompt”. The product is **industrialized product conception**: teams discuss a need, clarify ambiguity, map roles and screens, generate specs, and produce artifacts that the agentic harness can implement safely.

## Owns

- Product-conception UX: conversations, decisions, assumptions, screens, flows, and deliverables.
- Screen-by-screen and role-by-role specification workflows.
- Transformation of messy product discussion into structured product artifacts.
- Human validation loops before anything becomes implementation work.

## Does Not Own

- Agent execution and safe-write policy: belongs to `cos-matic`.
- Raw document extraction: belongs to Wrench.
- Persistent memory/search substrate: belongs to Gear.
- Generic visual design or Figma-like editing as the primary product.

## Allowed Dependencies

- Uses `cos-matic` to turn validated specs into execution plans.
- Uses Wrench tools to ingest context, inspect designs, and validate generated artifacts.
- Uses Gear for local-first persistence, provenance, artifact storage, and reproducible delivery.

## Product Vision Challenge

`rumble-canvas` must be a product thinking machine, not a prompt-to-UI toy. Its value is alignment: from conversation to decisions, from decisions to specs, from specs to safe implementation.

---

## MVP CLI

This repository now starts with a Rust workspace that proves the product → harness flow before any UI work.

```bash
cargo run -p rumble-canvas -- workspace sample --store target/canvas.json
cargo run -p rumble-canvas -- package build --store target/canvas.json --out target/package.json
cargo run -p rumble-canvas -- handoff build --store target/canvas.json --out target/handoff.json
cargo run -p rumble-canvas -- handoff validate --store target/canvas.json --json
cargo run -p rumble-canvas -- handoff plan --store target/canvas.json --json
cargo run -p rumble-canvas -- workspace show --store target/canvas.json
```

`handoff validate` and `handoff plan` delegate to the `cosmatic` binary. `plan` always passes `--dry-run`; Canvas does not expose an execution command. Validation findings and dry-run plans are stored back into the local JSON store when `--store` is used.

See `docs/OPERATIONS.md` for the local deploy/runbook.

Architecture:

```text
crates/domain   # SpecWorkspace, sections, traceability, waivers, actor references
crates/package  # SpecPackage, PackageReadinessSnapshot, stable hashing, immutability
crates/handoff  # ImplementationHandoff v0.1 generation and Canvas-side safety invariants
crates/cli      # Local MVP CLI delegating validation/planning to cosmatic
```
