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

## Installation

`wrench-inspect` is installed automatically in CI via the workflow. For local development:

```bash
cargo install --git https://github.com/constantin-jais/wrench-inspect.git --rev 973bd76a22c84003ec4f5c3a4379f9c93fe35278
```

If `wrench-inspect` is not available, the `wrench check` command gracefully skips checks with a warning.

## Future Extensions

- Artifact reference validation (gear-depot linkage).
- Permission matrix validation (crew-specific).
- Sector-wide checks (multi-repo).
