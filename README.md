# rumble-canvas

**Outil** : Rumble
**Rôle** : workspace de conception produit vers specs, écrans et handoff
**deployment_class** : product-linkable
**Maturité** : contract-first — handoff CLI prouvé ; workspace complet encore absent
**Place dans la chaîne DoD** : transforme une conversation produit en `SpecPackage` et handoff planning-only pour Bolt, avec preuves Gear/Wrench à compléter.
**Doctrine** : le produit exprime le sens ; Bolt orchestre, Wrench inspecte, Gear persiste.
**Souveraineté** : licences MIT/Apache/MPL compatibles ; pas d’AGPL/SSPL dans la chaîne versionnée.

## Ce que ça fait

Structure décisions, hypothèses, écrans et livrables pour rendre une idée implémentable sans sauter les gates. Le chemin CLI/handoff existe ; le produit multi-utilisateur et les inspections complètes restent les prochains incréments.

## Où ça se branche

- Amont : discussion produit et specs dans [ecosystem/specs/rumble-canvas](https://github.com/constantin-jais/constantin-jais/tree/main/ecosystem/specs/rumble-canvas).
- Aval : [bolt-cos-matic](https://github.com/constantin-jais/bolt-cos-matic) via handoff, [gear-memory](https://github.com/constantin-jais/gear-memory) pour provenance, [wrench-inspect](https://github.com/constantin-jais/wrench-inspect) pour complétude.
- Contrats : `canvas.bolt_handoff.v0.1`, `SpecPackage` en stabilisation.

---

## Dogfooding

This repository is part of the **Libre IA** tool family — one tool, one job, stacked.

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

This is not a full product workspace yet: the core contract path exists, but the durable UI and team workflow hardening are still missing. Schema stabilization (spec-package.v0.1.schema.json, I4) and Wrench completeness checks (I3) are now in place.

## Next product milestone

Multi-actor workspace support via the workspace-identity.v0.1 contract (I1–I2 proven with Biscuit fixture), persistent storage with provenance, and durable UI for team workflows.

## Purpose

`rumble-canvas` is the product-conception workspace of the Rumble/Portal/Bolt/Wrench/Gear ecosystem.

The product is not “draw a UI from a prompt”. The product is **industrialized product conception**: teams discuss a need, clarify ambiguity, map roles and screens, generate specs, and produce artifacts that the agentic harness can implement safely.

## Owns

- Product-conception UX: conversations, decisions, assumptions, screens, flows, and deliverables.
- Screen-by-screen and role-by-role specification workflows.
- Transformation of messy product discussion into structured product artifacts.
- Human validation loops before anything becomes implementation work.

## Does Not Own

- Agent execution and safe-write policy: belongs to `bolt-cos-matic`.
- Raw document extraction: belongs to `gear-loader`.
- Persistent memory/search substrate: belongs to Gear.
- Generic visual design or Figma-like editing as the primary product.

## Allowed Dependencies

- Uses `bolt-cos-matic` to turn validated specs into execution plans.
- Uses Gear Loader to ingest context and source material when needed.
- Uses Wrench tools to inspect designs and validate generated artifacts.
- Uses Portal when Canvas needs reusable client-platform primitives, tokens, accessibility, or native/web adapters.
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
