# AGENTS.md

Canonical agent-context surface for this repository. `CLAUDE.md` is a minimal adapter that imports this file.

## Purpose

Spec Studio is a contract-first product workspace for turning conversations into decisions, specs, and planning handoffs: create an intent, add requirements, decisions, and sourced contracts, freeze an immutable content-addressed SpecPackage with approvals and evidence, and emit a planning-only handoff that downstream planners consume without execution capability.

## Scope / Non-scope

- **Reserved home.** This repository is the public reserved home of Spec Studio. The product is being rebuilt in the canonical base repository [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) (multi-repo topology, [ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)); it reopens as the real product repository when the owner activates it.
- The legacy implementation carried here (Rust workspace `crates/{domain,package,handoff,store,cli}`, `specs/spec-package.v0.1.schema.json`) is **frozen for reference**.
- Non-scope: new product development in this repository until activation.

## Commands

Verified against `Cargo.toml`:

- Rust workspace: `cargo test --workspace` (members: `crates/domain`, `crates/package`, `crates/handoff`, `crates/store`, `crates/cli`).

## CI gates

- `hygiene` (`.github/workflows/hygiene.yml`) — repository hygiene and secret-pattern scan.
- `Context hygiene` (`.github/workflows/context-hygiene.yml`).

## Links

- [README](README.md) · [Français](README.fr.md)
- [docs/product-readiness.md](docs/product-readiness.md) — canonical readiness cockpit
- [docs/OPERATIONS.md](docs/OPERATIONS.md), [docs/ADOPTION_PATH_D11.md](docs/ADOPTION_PATH_D11.md), [docs/WRENCH_INTEGRATION.md](docs/WRENCH_INTEGRATION.md)
- [specs/spec-package.v0.1.schema.json](specs/spec-package.v0.1.schema.json)
- [ROADMAP.md](ROADMAP.md), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md)
