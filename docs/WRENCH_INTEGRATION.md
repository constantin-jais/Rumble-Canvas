# Proof Kit integration — Spec Studio completeness checks

## Overview

Spec Studio integrates the Proof Kit binary `wrench-inspect` to run completeness checks before planning.
The unit of inspection is the compatibility contract `canvas.bolt_handoff.v0.1`
(`kind: planning_request`): `wrench check` builds the handoff from the stored
workspace and the latest `SpecPackage`, writes it to a temporary file, and runs
`wrench-inspect handoff inspect --json <file>`.

The checks are wrench-inspect's own handoff inspections:

- **Structure** — `kind` must be `planning_request`; required
  `ImplementationHandoff` fields (source, package version/hash, …) present.
- **Execution policy** — human approval for execution must be required.
- **Traceability** — every traceability reference resolves to an object
  present in the handoff context (screens, actions, sections, …).
- **Waiver separation** — waiver owner and reviewer must be distinct actors.
- **Risks / blockers / capabilities** — declared sections are well-formed.

## Usage

```bash
cargo run -p rumble-canvas -- wrench check --store target/canvas.json
```

Output (stderr carries findings, stdout the verdict):

```
Running wrench completeness checks...
summary: 0 error(s), 0 warning(s), 0 info(s)
✓ All wrench checks passed
```

On failure the command exits non-zero and each non-info finding is printed as
`SEVERITY code at path: message (recommendation)`.

## Report shape

`wrench-inspect handoff inspect --json` prints a report on stdout, then exits
`0` when `valid` and `1` otherwise (the report is printed in both cases):

```json
{
  "valid": true,
  "summary": { "errors": 0, "warnings": 0, "infos": 0 },
  "findings": [],
  "coverage": { "...": "..." },
  "next_actions": []
}
```

`crates/handoff/src/wrench_integration.rs` deserializes this into
`WrenchReport` (ignoring `coverage`) and treats an invalid handoff as a
report, not a process error.

## Installation

`wrench-inspect` is installed automatically in CI via the workflow. For local
development:

```bash
WRENCH_REV=cd430ba1a9dec4ccd85ac865a2ba1fbed58d8c03
curl --fail --location --retry 3 \
  "https://github.com/libre-ai/proof-kit/archive/$WRENCH_REV.tar.gz" \
  --output "/tmp/wrench-$WRENCH_REV.tar.gz"
tar -xzf "/tmp/wrench-$WRENCH_REV.tar.gz" -C /tmp
cargo install --locked --path "/tmp/proof-kit-$WRENCH_REV/inspect"
```

If `wrench-inspect` is not available, the local `wrench check` command is advisory and
skips checks with a visible warning; it must never be presented as a passed gate. The integration test
(`crates/handoff/tests/wrench_integration_test.rs`) is `#[ignore]`d for the
same reason. Protected CI installs the pinned binary and runs the test explicitly with `-- --ignored`,
so the release gate does not use the advisory fallback.

## Future Extensions

- Artifact reference validation through Artifact Supply manifests.
- Mission permission validation at the Agent Board boundary.
- Portfolio-wide checks across repositories.
